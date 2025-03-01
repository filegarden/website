/** A mapping from each object to its unique key. */
const keysByObject = new WeakMap<object, symbol>();

/** Gets a unique key for an object. */
export default function keyFromIdentity(errorBox: object): symbol {
  let key = keysByObject.get(errorBox);

  if (key === undefined) {
    key = Symbol();
    keysByObject.set(errorBox, key);
  }

  return key;
}
