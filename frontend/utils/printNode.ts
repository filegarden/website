export default async function printNode(node: Node) {
  const iframe = document.createElement("iframe");
  iframe.hidden = true;
  iframe.srcdoc =
    '<!DOCTYPE html><html><head><meta charset="utf8"></head><body></body></html>';

  document.body.append(iframe);

  await new Promise((resolve) => {
    iframe.addEventListener("load", resolve);
  });

  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The window is non-nullable once loaded.
  const contentWindow = iframe.contentWindow!;
  contentWindow.document.body.append(node);

  contentWindow.print();

  await new Promise((resolve) => {
    contentWindow.addEventListener("afterprint", resolve);
  });

  iframe.remove();
}
