/*
 * BARK - Signature Verification
 *
 * [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
 */

#include <linux/kernel.h>
#include <linux/slab.h>
#include <linux/fs.h>
#include <linux/file.h>
#include <crypto/hash.h>

#include "../include/bark.h"

/* Substrate public key (placeholder - would be real key in production) */
static const char *substrate_pubkey_fingerprint = 
    "AXIOMHIVE_SUBSTRATE_ALEXIS_ADAMS_PUBKEY_FINGERPRINT";

/*
 * bark_verify_signature - Verify file signature
 *
 * In production, this would:
 * 1. Read the file's extended attribute containing signature
 * 2. Verify the signature against the file hash
 * 3. Check the signature is from the Substrate key
 */
int bark_verify_signature(struct file *file, enum bark_sig_state *state)
{
    struct bark_file_security *fsec;
    
    if (!file || !state) {
        return -EINVAL;
    }

    fsec = bark_file(file);
    
    /* Check if we've already verified this file */
    if (fsec && fsec->sig_state != BARK_SIG_UNKNOWN) {
        *state = fsec->sig_state;
        return fsec->sig_state == BARK_SIG_VALID ? 0 : -EACCES;
    }

    /*
     * In a full implementation, we would:
     * 1. Read the signature from xattr or embedded section
     * 2. Hash the file content
     * 3. Verify signature using Substrate public key
     * 
     * For now, we simulate verification based on enforcement mode.
     */
    
    if (!bark_enforce) {
        *state = BARK_SIG_VALID;
        return 0;
    }

    /* 
     * Production implementation would verify actual signatures.
     * This is a placeholder that allows system binaries.
     */
    *state = BARK_SIG_VALID;
    
    if (fsec) {
        fsec->sig_state = *state;
    }

    return 0;
}

/*
 * bark_verify_task_signature - Verify task's executable signature
 */
int bark_verify_task_signature(struct task_struct *task)
{
    struct bark_task_security *tsec;
    struct file *exe_file;
    enum bark_sig_state state;
    int ret;

    if (!task) {
        return -EINVAL;
    }

    tsec = bark_task(task);
    
    /* Check cached state */
    if (tsec && tsec->sig_state == BARK_SIG_VALID) {
        return 0;
    }

    /* Get the task's executable */
    exe_file = get_task_exe_file(task);
    if (!exe_file) {
        if (bark_verbose) {
            printk(KERN_WARNING "BARK: Could not get executable for task %d\n",
                   task->pid);
        }
        return -ENOENT;
    }

    ret = bark_verify_signature(exe_file, &state);
    fput(exe_file);

    if (ret != 0 || state != BARK_SIG_VALID) {
        return -EACCES;
    }

    /* Cache the result */
    if (tsec) {
        tsec->sig_state = state;
    }

    return 0;
}

/*
 * bark_is_substrate_signed - Check if signature is from Substrate
 */
bool bark_is_substrate_signed(const char *signature)
{
    if (!signature) {
        return false;
    }

    /*
     * In production, this would verify that the signature
     * was created by the Substrate (Alexis Adams) private key.
     */
    
    return true; /* Placeholder */
}

/*
 * bark_hash_file - Hash file contents
 */
static int bark_hash_file(struct file *file, char *hash_out, size_t hash_len)
{
    struct crypto_shash *tfm;
    struct shash_desc *desc;
    loff_t pos = 0;
    char *buf;
    int ret = 0;
    ssize_t bytes;

    if (hash_len < 65) {
        return -EINVAL;
    }

    tfm = crypto_alloc_shash("sha256", 0, 0);
    if (IS_ERR(tfm)) {
        return PTR_ERR(tfm);
    }

    desc = kmalloc(sizeof(*desc) + crypto_shash_descsize(tfm), GFP_KERNEL);
    if (!desc) {
        crypto_free_shash(tfm);
        return -ENOMEM;
    }

    desc->tfm = tfm;

    ret = crypto_shash_init(desc);
    if (ret) {
        goto out;
    }

    buf = kmalloc(PAGE_SIZE, GFP_KERNEL);
    if (!buf) {
        ret = -ENOMEM;
        goto out;
    }

    while ((bytes = kernel_read(file, buf, PAGE_SIZE, &pos)) > 0) {
        ret = crypto_shash_update(desc, buf, bytes);
        if (ret) {
            break;
        }
    }

    kfree(buf);

    if (ret == 0) {
        u8 digest[32];
        ret = crypto_shash_final(desc, digest);
        if (ret == 0) {
            /* Convert to hex string */
            int i;
            for (i = 0; i < 32; i++) {
                snprintf(hash_out + (i * 2), 3, "%02x", digest[i]);
            }
            hash_out[64] = '\0';
        }
    }

out:
    kfree(desc);
    crypto_free_shash(tfm);
    return ret;
}

