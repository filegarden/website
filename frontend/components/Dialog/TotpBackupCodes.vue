<script setup lang="ts">
const { backupCodes } = defineProps<{
  /** The new TOTP backup codes. */
  backupCodes: string[];
}>();

const backupCodesElement = useTemplateRef("backup-codes");

function copyBackupCodes() {
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The element must be mounted since a button was clicked.
  backupCodesElement.value!.select();

  // eslint-disable-next-line @typescript-eslint/no-deprecated -- The Clipboard API only works in secure contexts.
  document.execCommand("copy");
}

function printBackupCodes() {
  const body = document.createDocumentFragment();

  const h1 = document.createElement("h1");
  h1.append("File Garden");
  body.append(h1);

  const h2 = document.createElement("h2");
  h2.append("2FA Backup Codes");
  body.append(h2);

  const ul = document.createElement("ul");

  for (const backupCode of backupCodes) {
    const li = document.createElement("li");
    const code = document.createElement("code");

    code.append(backupCode);

    li.append(code);
    ul.append(li);
  }

  body.append(ul);

  return printNode(body);
}
</script>

<template>
  <Dialog size="medium">
    <template #heading>2FA Setup</template>

    <p>Your account is now more secure against stolen passwords.</p>

    <p>
      <strong>
        Save these backup codes, or you'll be permanently locked out of your
        account if you lose access to your authenticator app!
      </strong>
    </p>

    <textarea
      ref="backup-codes"
      aria-label="2FA Backup Codes"
      class="totp-backup-codes"
      :value="backupCodes.join(' ')"
      readonly
    ></textarea>

    <div class="totp-backup-codes-actions">
      <Button @click="copyBackupCodes">Copy</Button>
      <Button @click="printBackupCodes">Print</Button>
    </div>

    <p>Protect these codes like a password. Don't share them.</p>

    <template #actions>
      <Button type="submit">Got it</Button>
    </template>
  </Dialog>
</template>

<style scoped lang="scss">
.totp-backup-codes {
  font-family: var(--font-family-monospace);
  resize: none;

  margin: 1em 0;
  // The margins can't collapse without this.
  display: block;
}

.totp-backup-codes-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5em;
}
</style>
