/** An {@link Element} with a `focus` method. */
export type FocusableElement = Element & Pick<HTMLElement, "focus">;

/**
 * A type guard that checks if the `focus` method is present on an `Element`.
 *
 * @param element - The element to check.
 *
 * @returns Whether the element has a `focus` method.
 */
export function hasFocusMethod(element: Element): element is FocusableElement {
  return typeof (element as Element & { focus?: unknown }).focus === "function";
}

/** Options for {@link attemptFocus}. */
export interface AttemptFocusOptions {
  /**
   * Whether to focus any focusable element or only elements that are part of a
   * tab sequence.
   */
  filter?: "focusable" | "tabbable";
}

/**
 * Tries to focus an element regardless of whether it's focusable.
 *
 * @param element - The element to focus.
 *
 * @returns Whether the element was successfully focused.
 */
export function attemptFocus(
  element: Element,
  { filter = "focusable" }: AttemptFocusOptions = {},
): boolean {
  if (!hasFocusMethod(element)) {
    return false;
  }

  if (
    filter === "tabbable" &&
    typeof (element as typeof element & { tabIndex?: unknown }).tabIndex ===
      "number" &&
    (element as typeof element & { tabIndex: number }).tabIndex < 0
  ) {
    return false;
  }

  element.focus();
  return element === document.activeElement;
}
