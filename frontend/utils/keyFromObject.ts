/** A mapping from each object to its unique key. */
const keysByObject = new WeakMap<object, symbol>();

/**
 * Gets a symbol that can act as a key unique to the identity of an object.
 *
 * @param object - The object to get a symbol for.
 *
 * @returns A unique symbol for the specified object. This symbol is always
 * the same for the same object.
 */
export default function keyFromObject(object: object): symbol {
  let key = keysByObject.get(object);

  if (key === undefined) {
    key = Symbol("keyFromObject");
    keysByObject.set(object, key);
  }

  return key;
}
