/** A mapping from each object to its unique key. */
const keysByObject = new WeakMap<object, symbol>();

/** Gets a key which is unique to the identity of an object. */
export default function keyFromObject(errorBox: object): symbol {
  let key = keysByObject.get(errorBox);

  if (key === undefined) {
    key = Symbol();
    keysByObject.set(errorBox, key);
  }

  return key;
}
