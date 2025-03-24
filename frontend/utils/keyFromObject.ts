/** A mapping from each object to its unique key. */
const keysByObject = new WeakMap<object, symbol>();

/** Gets a key which is unique to the identity of an object. */
export default function keyFromObject(object: object): symbol {
  let key = keysByObject.get(object);

  if (key === undefined) {
    key = Symbol("keyFromObject");
    keysByObject.set(object, key);
  }

  return key;
}
