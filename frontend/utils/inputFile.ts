/**
 * Opens a file input prompt.
 *
 * @param properties - Any properties to set on the file input element.
 *
 * @returns The selected {@link FileList}, if any.
 */
export async function inputFile(
  properties: Partial<HTMLInputElement> = {},
): Promise<HTMLInputElement["files"]> {
  const input = document.createElement("input");
  input.type = "file";
  Object.assign(input, properties);

  await new Promise((resolve) => {
    input.addEventListener("change", resolve);
    input.addEventListener("cancel", resolve);
    input.click();
  });

  return input.files;
}
