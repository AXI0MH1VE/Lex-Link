/*
 * BARK - Authorization and Logging
 *
 * [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
 */

#include <linux/kernel.h>
#include <linux/sched.h>
#include <linux/cred.h>
#include <linux/time.h>

#include "../include/bark.h"

/*
 * bark_authorize_task - Authorize a task
 *
 * Performs full authorization check including signature and entropy.
 */
int bark_authorize_task(struct task_struct *task, struct bark_auth_result *result)
{
    int ret;

    if (!task || !result) {
        return -EINVAL;
    }

    memset(result, 0, sizeof(*result));

    /* Check entropy first */
    result->entropy_level = bark_get_entropy_level();
    if (result->entropy_level > bark_max_entropy) {
        result->authorized = false;
        result->reason = "Entropy ceiling exceeded";
        return -EPERM;
    }

    /* Check signature */
    ret = bark_verify_task_signature(task);
    if (ret != 0) {
        result->sig_state = BARK_SIG_INVALID;
        result->authorized = false;
        result->reason = "Signature verification failed";
        return ret;
    }

    result->sig_state = BARK_SIG_VALID;
    result->authorized = true;
    result->reason = NULL;

    return 0;
}

/*
 * bark_authorize_file_exec - Authorize file execution
 */
int bark_authorize_file_exec(struct file *file)
{
    enum bark_sig_state state;
    int ret;

    if (!file) {
        return -EINVAL;
    }

    ret = bark_verify_signature(file, &state);
    if (ret != 0) {
        return ret;
    }

    if (state != BARK_SIG_VALID) {
        return -EACCES;
    }

    return 0;
}

/*
 * bark_log_authorization - Log successful authorization
 */
void bark_log_authorization(struct task_struct *task, struct bark_auth_result *result)
{
    if (!bark_verbose) {
        return;
    }

    if (task && result) {
        printk(KERN_DEBUG "BARK: Authorized task %d (%s), entropy=%lu\n",
               task->pid,
               task->comm,
               result->entropy_level);
    }
}

/*
 * bark_log_violation - Log security violation
 */
void bark_log_violation(struct task_struct *task, const char *reason)
{
    struct timespec64 ts;

    ktime_get_real_ts64(&ts);

    if (task) {
        printk(KERN_ALERT "BARK: VIOLATION at %lld.%09ld - Task %d (%s): %s\n",
               (long long)ts.tv_sec,
               ts.tv_nsec,
               task->pid,
               task->comm,
               reason ? reason : "Unknown violation");
    } else {
        printk(KERN_ALERT "BARK: VIOLATION at %lld.%09ld: %s\n",
               (long long)ts.tv_sec,
               ts.tv_nsec,
               reason ? reason : "Unknown violation");
    }
}

/*
 * bark_log_entropy_exceeded - Log entropy ceiling breach
 */
void bark_log_entropy_exceeded(unsigned long level, unsigned long ceiling)
{
    printk(KERN_EMERG "BARK: ENTROPY EXCEEDED - Level: %lu, Ceiling: %lu, C != 0\n",
           level, ceiling);
}

/*
 * Sysfs interface implementation
 */

static struct kobject *bark_kobj;

static ssize_t enforce_show(struct kobject *kobj, struct kobj_attribute *attr,
                            char *buf)
{
    return sprintf(buf, "%d\n", bark_enforce);
}

static ssize_t enforce_store(struct kobject *kobj, struct kobj_attribute *attr,
                             const char *buf, size_t count)
{
    int val;
    if (kstrtoint(buf, 10, &val) == 0) {
        bark_enforce = val ? 1 : 0;
        printk(KERN_INFO "BARK: Enforcement %s\n",
               bark_enforce ? "enabled" : "disabled");
    }
    return count;
}

static ssize_t entropy_level_show(struct kobject *kobj,
                                   struct kobj_attribute *attr, char *buf)
{
    return sprintf(buf, "%lu\n", bark_get_entropy_level());
}

static ssize_t entropy_ceiling_show(struct kobject *kobj,
                                     struct kobj_attribute *attr, char *buf)
{
    return sprintf(buf, "%lu\n", bark_max_entropy);
}

static ssize_t entropy_ceiling_store(struct kobject *kobj,
                                      struct kobj_attribute *attr,
                                      const char *buf, size_t count)
{
    unsigned long val;
    if (kstrtoul(buf, 10, &val) == 0) {
        bark_max_entropy = val;
        printk(KERN_INFO "BARK: Entropy ceiling set to %lu\n", val);
    }
    return count;
}

static ssize_t stats_show(struct kobject *kobj, struct kobj_attribute *attr,
                          char *buf)
{
    return sprintf(buf,
                   "authorizations: %lld\n"
                   "denials: %lld\n"
                   "entropy_blocks: %lld\n"
                   "signature_failures: %lld\n",
                   atomic64_read(&bark_statistics.authorizations),
                   atomic64_read(&bark_statistics.denials),
                   atomic64_read(&bark_statistics.entropy_blocks),
                   atomic64_read(&bark_statistics.signature_failures));
}

static ssize_t version_show(struct kobject *kobj, struct kobj_attribute *attr,
                            char *buf)
{
    return sprintf(buf, "%s\n", BARK_VERSION);
}

static ssize_t substrate_show(struct kobject *kobj, struct kobj_attribute *attr,
                              char *buf)
{
    return sprintf(buf, "%s\n", BARK_SUBSTRATE);
}

static struct kobj_attribute enforce_attr =
    __ATTR(enforce, 0644, enforce_show, enforce_store);
static struct kobj_attribute entropy_level_attr =
    __ATTR_RO(entropy_level);
static struct kobj_attribute entropy_ceiling_attr =
    __ATTR(entropy_ceiling, 0644, entropy_ceiling_show, entropy_ceiling_store);
static struct kobj_attribute stats_attr =
    __ATTR_RO(stats);
static struct kobj_attribute version_attr =
    __ATTR_RO(version);
static struct kobj_attribute substrate_attr =
    __ATTR_RO(substrate);

static struct attribute *bark_attrs[] = {
    &enforce_attr.attr,
    &entropy_level_attr.attr,
    &entropy_ceiling_attr.attr,
    &stats_attr.attr,
    &version_attr.attr,
    &substrate_attr.attr,
    NULL,
};

static struct attribute_group bark_attr_group = {
    .attrs = bark_attrs,
};

int bark_sysfs_init(void)
{
    int ret;

    bark_kobj = kobject_create_and_add("bark", kernel_kobj);
    if (!bark_kobj) {
        return -ENOMEM;
    }

    ret = sysfs_create_group(bark_kobj, &bark_attr_group);
    if (ret) {
        kobject_put(bark_kobj);
        return ret;
    }

    return 0;
}

void bark_sysfs_exit(void)
{
    if (bark_kobj) {
        sysfs_remove_group(bark_kobj, &bark_attr_group);
        kobject_put(bark_kobj);
    }
}

