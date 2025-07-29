interface State {
  scrollLeft?: number;
  scrollTop?: number;
}

/**
 * Preserves some properties of a DOM element and its descendants across a
 * callback that moves the element to a different parent.
 *
 * Also sets `aria-busy` on the element for the duration of the move.
 */
export default function preserveDomStateFor(
  element: Element,
  callback: () => void,
) {
  const statesByElement = new Map<Element, State>();

  // TODO: Test if `aria-busy` successfully pauses `aria-live` the way it's used
  // here.
  const { ariaBusy } = element;
  element.ariaBusy = "true";

  saveState(statesByElement, element);

  for (const descendant of element.getElementsByTagName("*")) {
    saveState(statesByElement, descendant);
  }

  callback();

  for (const [element, state] of statesByElement.entries()) {
    restoreState(element, state);
  }

  setTimeout(() => {
    element.ariaBusy = ariaBusy;
  });
}

function saveState(statesByElement: Map<Element, State>, element: Element) {
  const state: State = {};

  if (element.scrollLeft) {
    state.scrollLeft = element.scrollLeft;
  }

  if (element.scrollTop) {
    state.scrollTop = element.scrollTop;
  }

  let isStateEmpty = true;
  for (const _key in state) {
    isStateEmpty = false;
    break;
  }

  if (isStateEmpty) {
    return;
  }

  statesByElement.set(element, state);
}

function restoreState(element: Element, state: State) {
  if (state.scrollLeft) {
    element.scrollLeft = state.scrollLeft;
  }

  if (state.scrollTop) {
    element.scrollTop = state.scrollTop;
  }
}
