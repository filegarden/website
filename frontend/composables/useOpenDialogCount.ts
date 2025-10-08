/**
 * Gets a global state that keeps track of how many dialogs are currently open.
 *
 * @returns The state for the open dialog count.
 */
export default function useOpenDialogCount() {
  return useState(() => 0);
}
