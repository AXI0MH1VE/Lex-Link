/*
 * BARK - Binary Authority Regulatory Kernel
 * Main module entry point
 *
 * [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
 */

#include <linux/init.h>
#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/lsm_hooks.h>
#include <linux/security.h>
#include <linux/slab.h>

#include "../include/bark.h"

/* Module metadata */
MODULE_LICENSE("Proprietary");
MODULE_AUTHOR("Alexis Adams");
MODULE_DESCRIPTION("BARK - Binary Authority Regulatory Kernel LSM");
MODULE_VERSION(BARK_VERSION);

/* Module parameters */
int bark_enforce = BARK_DEFAULT_ENFORCE;
module_param(bark_enforce, int, 0644);
MODULE_PARM_DESC(bark_enforce, "Enable enforcement mode (default: 1)");

unsigned long bark_max_entropy = BARK_DEFAULT_MAX_ENTROPY;
module_param(bark_max_entropy, ulong, 0644);
MODULE_PARM_DESC(bark_max_entropy, "Maximum entropy ceiling (default: 1000)");

int bark_verbose = 0;
module_param(bark_verbose, int, 0644);
MODULE_PARM_DESC(bark_verbose, "Enable verbose logging (default: 0)");

/* Global statistics */
struct bark_stats bark_statistics = {
    .authorizations = ATOMIC64_INIT(0),
    .denials = ATOMIC64_INIT(0),
    .entropy_blocks = ATOMIC64_INIT(0),
    .signature_failures = ATOMIC64_INIT(0),
};

/* LSM blob sizes */
static struct lsm_blob_sizes bark_blob_sizes __lsm_ro_after_init = {
    .lbs_task = sizeof(struct bark_task_security),
    .lbs_file = sizeof(struct bark_file_security),
};

/*
 * bark_task_alloc - LSM hook for task allocation
 *
 * Verifies that new processes meet entropy and signature requirements.
 */
static int bark_task_alloc_hook(struct task_struct *task, unsigned long clone_flags)
{
    struct bark_auth_result result = {0};
    int ret;

    if (!bark_enforce) {
        return 0;
    }

    /* Check entropy ceiling */
    if (bark_check_entropy_ceiling() != 0) {
        if (bark_verbose) {
            printk(KERN_EMERG "BARK: Entropy Limit Exceeded. Process Blocked.\n");
        }
        bark_log_entropy_exceeded(bark_get_entropy_level(), bark_max_entropy);
        atomic64_inc(&bark_statistics.entropy_blocks);
        return BARK_ERR_ENTROPY_EXCEEDED;
    }

    /* Verify substrate signature */
    ret = bark_authorize_task(task, &result);
    if (ret != 0) {
        if (bark_verbose) {
            printk(KERN_ALERT "BARK: Unauthorized Projection. C != 0. Reason: %s\n",
                   result.reason ? result.reason : "unknown");
        }
        bark_log_violation(task, result.reason);
        atomic64_inc(&bark_statistics.denials);
        return BARK_ERR_NOT_AUTHORIZED;
    }

    bark_log_authorization(task, &result);
    atomic64_inc(&bark_statistics.authorizations);

    return 0;
}

/*
 * bark_task_free_hook - LSM hook for task cleanup
 */
static void bark_task_free_hook(struct task_struct *task)
{
    struct bark_task_security *tsec = bark_task(task);
    
    if (tsec) {
        /* Cleanup any task-specific security data */
        memset(tsec, 0, sizeof(*tsec));
    }
}

/*
 * bark_bprm_check - LSM hook for binary execution check
 *
 * Verifies executable signature before allowing execution.
 */
static int bark_bprm_check_hook(struct linux_binprm *bprm)
{
    enum bark_sig_state state;
    int ret;

    if (!bark_enforce) {
        return 0;
    }

    ret = bark_verify_signature(bprm->file, &state);
    if (ret != 0 || state != BARK_SIG_VALID) {
        if (bark_verbose) {
            printk(KERN_ALERT "BARK: Binary signature verification failed for %s\n",
                   bprm->filename);
        }
        atomic64_inc(&bark_statistics.signature_failures);
        return BARK_ERR_SIGNATURE_INVALID;
    }

    return 0;
}

/*
 * bark_file_permission_hook - LSM hook for file access
 */
static int bark_file_permission_hook(struct file *file, int mask)
{
    /* For now, allow all file access if other checks pass */
    return 0;
}

/*
 * bark_task_fix_setuid_hook - LSM hook for setuid changes
 */
static int bark_task_fix_setuid_hook(struct cred *new, const struct cred *old, int flags)
{
    if (!bark_enforce) {
        return 0;
    }

    /* Verify the process is authorized to change credentials */
    if (bark_verify_task_signature(current) != 0) {
        if (bark_verbose) {
            printk(KERN_ALERT "BARK: Unauthorized credential change blocked\n");
        }
        return BARK_ERR_NOT_AUTHORIZED;
    }

    return 0;
}

/* LSM hook list */
static struct security_hook_list bark_hooks[] __lsm_ro_after_init = {
    LSM_HOOK_INIT(task_alloc, bark_task_alloc_hook),
    LSM_HOOK_INIT(task_free, bark_task_free_hook),
    LSM_HOOK_INIT(bprm_check_security, bark_bprm_check_hook),
    LSM_HOOK_INIT(file_permission, bark_file_permission_hook),
    LSM_HOOK_INIT(task_fix_setuid, bark_task_fix_setuid_hook),
};

/*
 * bark_init - Module initialization
 */
static int __init bark_lsm_init(void)
{
    int ret;

    printk(KERN_INFO "BARK: Initializing Binary Authority Regulatory Kernel v%s\n",
           BARK_VERSION);
    printk(KERN_INFO "BARK: [AXIOMHIVE PROJECTION - SUBSTRATE: %s]\n",
           BARK_SUBSTRATE);

    /* Initialize sysfs interface */
    ret = bark_sysfs_init();
    if (ret != 0) {
        printk(KERN_ERR "BARK: Failed to initialize sysfs interface\n");
        return ret;
    }

    /* Register LSM hooks */
    security_add_hooks(bark_hooks, ARRAY_SIZE(bark_hooks), BARK_NAME);

    printk(KERN_INFO "BARK: Enforcement: %s\n", bark_enforce ? "ENABLED" : "DISABLED");
    printk(KERN_INFO "BARK: Max Entropy: %lu\n", bark_max_entropy);
    printk(KERN_INFO "BARK: Policy: C = 0\n");

    return 0;
}

/*
 * bark_exit - Module cleanup
 */
static void __exit bark_lsm_exit(void)
{
    printk(KERN_INFO "BARK: Statistics - Authorizations: %lld, Denials: %lld\n",
           atomic64_read(&bark_statistics.authorizations),
           atomic64_read(&bark_statistics.denials));

    bark_sysfs_exit();

    printk(KERN_INFO "BARK: Module unloaded\n");
}

/* LSM definition */
DEFINE_LSM(bark) = {
    .name = BARK_NAME,
    .init = bark_lsm_init,
    .blobs = &bark_blob_sizes,
};

module_init(bark_lsm_init);
module_exit(bark_lsm_exit);

