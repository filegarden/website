/**
 * A type guard that checks if the `focus` method is present on an `Element`.
 */
export default function hasFocusMethod(
  element: Element,
): element is Element & Pick<HTMLElement, "focus"> {
  return typeof (element as Element & { focus?: unknown }).focus === "function";
}
