/**
 * Gets the last descendant element of an element in the DOM tree.
 *
 * @param element - The element to get the last descendant element of.
 *
 * @returns The last descendant element.
 */
export function lastDescendantElement(element: Element): Element | null {
  let descendant = element.lastElementChild;

  if (descendant) {
    while (descendant.lastElementChild) {
      descendant = descendant.lastElementChild;
    }
  }

  return descendant;
}

/**
 * Gets an element's previous element in the DOM tree.
 *
 * @param element - The element to get the previous element of.
 *
 * @returns The previous element.
 */
export function previousElement(element: Element): Element | null {
  return element.previousElementSibling
    ? previousElementBeforeClosing(element.previousElementSibling)
    : element.parentElement;
}

/**
 * Gets the previous element in the DOM tree before an element's closing tag.
 *
 * @param element - The element to get the previous element before the closing
 * of.
 *
 * @returns The previous element.
 */
export function previousElementBeforeClosing(element: Element): Element {
  return lastDescendantElement(element) ?? element;
}

/**
 * Gets an element's next element in the DOM tree.
 *
 * @param element - The element to get the next element of.
 *
 * @returns The next element.
 */
export function nextElement(element: Element): Element | null {
  return element.firstElementChild ?? nextElementAfterClosing(element);
}

/**
 * Gets the next element in the DOM tree after an element's closing tag.
 *
 * @param element - The element to get the next element after the closing of.
 *
 * @returns The next element.
 */
export function nextElementAfterClosing(element: Element): Element | null {
  while (true) {
    if (element.nextElementSibling) {
      return element.nextElementSibling;
    }

    if (element.parentElement === null) {
      return null;
    }

    element = element.parentElement;
  }
}

type ElementsInRangePosition = "before" | "after-opening" | "closing";

function elementBeforePosition(
  position: ElementsInRangePosition,
  element: Element,
): Element | null {
  if (position === "before") {
    return previousElement(element);
  }

  if (position === "after-opening") {
    return element;
  }

  position satisfies "closing";
  return previousElementBeforeClosing(element);
}

function elementAfterPosition(
  position: ElementsInRangePosition,
  element: Element,
): Element | null {
  if (position === "before") {
    return element;
  }

  if (position === "after-opening") {
    return nextElement(element);
  }

  position satisfies "closing";
  return nextElementAfterClosing(element);
}

/**
 * Returns a sequential range of elements in the DOM tree.
 *
 * @param startPosition - The range's start position relative to `startElement`.
 * @param startElement - The element defining the start of the range.
 * @param endPosition - The range's end position relative to `startElement`.
 * @param endElement - The element defining the end of the range.
 *
 * @yields The next element in the range.
 */
export function* elementsInRange(
  startPosition: ElementsInRangePosition,
  startElement: Element,
  endPosition: ElementsInRangePosition,
  endElement: Element,
): Generator<Element, void, never> {
  let element = elementAfterPosition(startPosition, startElement);

  // The element at the end position is recomputed every iteration in case of
  // DOM mutations.
  while (element !== elementAfterPosition(endPosition, endElement)) {
    if (element === null) {
      throw new Error(
        "`elementsInRange` was provided an invalid range (start position after end position in the DOM tree)",
      );
    }

    yield element;

    element = nextElement(element);
  }
}

/**
 * Returns a sequential range of elements in the DOM tree, in reverse order.
 *
 * @param startPosition - The range's start position relative to `startElement`.
 * @param startElement - The element defining the start of the range.
 * @param endPosition - The range's end position relative to `startElement`.
 * @param endElement - The element defining the end of the range.
 *
 * @yields The previous element in the range.
 */
export function* elementsInRangeReversed(
  startPosition: ElementsInRangePosition,
  startElement: Element,
  endPosition: ElementsInRangePosition,
  endElement: Element,
): Generator<Element, void, never> {
  let element = elementBeforePosition(endPosition, endElement);

  // The element at the start position is recomputed every iteration in case of
  // DOM mutations.
  while (element !== elementBeforePosition(startPosition, startElement)) {
    if (element === null) {
      throw new Error(
        "`elementsInRangeReversed` was provided an invalid range (start position after end position in the DOM tree)",
      );
    }

    yield element;

    element = previousElement(element);
  }
}
