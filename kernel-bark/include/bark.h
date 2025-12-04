/*
 * BARK - Binary Authority Regulatory Kernel
 * Linux Security Module for Substrate Authority Enforcement
 *
 * [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
 */

#ifndef _BARK_H
#define _BARK_H

#include <linux/types.h>
#include <linux/lsm_hooks.h>
#include <linux/security.h>

/* Module information */
#define BARK_VERSION "1.0.0"
#define BARK_NAME "bark"
#define BARK_SUBSTRATE "Alexis Adams"

/* Configuration defaults */
#define BARK_DEFAULT_MAX_ENTROPY 1000
#define BARK_DEFAULT_ENFORCE 1

/* Error codes */
#define BARK_ERR_ENTROPY_EXCEEDED -EPERM
#define BARK_ERR_SIGNATURE_INVALID -EACCES
#define BARK_ERR_NOT_AUTHORIZED -EACCES

/* Signature verification states */
enum bark_sig_state {
    BARK_SIG_UNKNOWN = 0,
    BARK_SIG_VALID,
    BARK_SIG_INVALID,
    BARK_SIG_MISSING,
};

/* Process authorization result */
struct bark_auth_result {
    enum bark_sig_state sig_state;
    unsigned long entropy_level;
    bool authorized;
    const char *reason;
};

/* BARK security blob for tasks */
struct bark_task_security {
    enum bark_sig_state sig_state;
    u64 authorization_time;
    u32 authorization_count;
    bool is_substrate_process;
};

/* BARK security blob for files */
struct bark_file_security {
    enum bark_sig_state sig_state;
    char signature_hash[65]; /* SHA-256 hex string + null */
};

/* Function declarations */

/* Core operations */
int bark_init(void);
void bark_exit(void);

/* Signature verification */
int bark_verify_signature(struct file *file, enum bark_sig_state *state);
int bark_verify_task_signature(struct task_struct *task);
bool bark_is_substrate_signed(const char *signature);

/* Entropy monitoring */
unsigned long bark_get_entropy_level(void);
int bark_check_entropy_ceiling(void);
void bark_update_entropy(unsigned long delta);

/* Authorization */
int bark_authorize_task(struct task_struct *task, struct bark_auth_result *result);
int bark_authorize_file_exec(struct file *file);

/* LSM hooks */
int bark_task_alloc(struct task_struct *task, unsigned long clone_flags);
void bark_task_free(struct task_struct *task);
int bark_bprm_check_security(struct linux_binprm *bprm);
int bark_file_permission(struct file *file, int mask);
int bark_task_fix_setuid(struct cred *new, const struct cred *old, int flags);

/* Sysfs interface */
int bark_sysfs_init(void);
void bark_sysfs_exit(void);

/* Audit logging */
void bark_log_authorization(struct task_struct *task, struct bark_auth_result *result);
void bark_log_violation(struct task_struct *task, const char *reason);
void bark_log_entropy_exceeded(unsigned long level, unsigned long ceiling);

/* Utility functions */
static inline struct bark_task_security *bark_task(struct task_struct *task)
{
    /* Would return task's security blob in real implementation */
    return NULL;
}

static inline struct bark_file_security *bark_file(struct file *file)
{
    /* Would return file's security blob in real implementation */
    return NULL;
}

/* Module parameters (extern declarations) */
extern int bark_enforce;
extern unsigned long bark_max_entropy;
extern int bark_verbose;

/* Statistics */
struct bark_stats {
    atomic64_t authorizations;
    atomic64_t denials;
    atomic64_t entropy_blocks;
    atomic64_t signature_failures;
};

extern struct bark_stats bark_statistics;

#endif /* _BARK_H */

