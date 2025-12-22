/**
 * Like {@link WeakSet}, but iterable with additional methods from the
 * {@link Set} class.
 */
export default class IterableWeakSet<T extends WeakKey>
  implements
    WeakSet<T>,
    Pick<
      Set<T>,
      "clear" | "entries" | "keys" | "values" | typeof Symbol.iterator
    >
{
  readonly [Symbol.toStringTag] = this.constructor.name;

  readonly #refs = new Set<WeakRef<T>>();
  readonly #refsByValue = new WeakMap<T, WeakRef<T>>();
  readonly #registry = new FinalizationRegistry((ref: WeakRef<T>) => {
    this.#refs.delete(ref);
  });

  constructor(iterable?: ConstructorParameters<typeof WeakSet<T>>[0]) {
    if (iterable === undefined) {
      return;
    }

    for (const value of iterable) {
      this.add(value);
    }
  }

  get size() {
    return this.#refs.size;
  }

  *[Symbol.iterator](): SetIterator<T> {
    for (const ref of this.#refs) {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion -- The registry would have deleted the ref if its value were reclaimed.
      yield ref.deref()!;
    }
  }

  has(value: T) {
    return this.#refsByValue.has(value);
  }

  add(value: T) {
    if (this.has(value)) {
      return this;
    }

    const ref = new WeakRef(value);
    this.#refs.add(ref);
    this.#refsByValue.set(value, ref);
    this.#registry.register(value, ref, ref);

    return this;
  }

  delete(value: T) {
    const ref = this.#refsByValue.get(value);
    if (!ref) {
      return false;
    }

    this.#refs.delete(ref);
    this.#refsByValue.delete(value);
    this.#registry.unregister(ref);
    return true;
  }

  clear() {
    for (const value of this) {
      this.delete(value);
    }
  }

  values() {
    return this[Symbol.iterator]();
  }

  keys() {
    return this.values();
  }

  *entries(): SetIterator<[T, T]> {
    for (const value of this) {
      yield [value, value];
    }
  }

  /** @see {@link Set.forEach} */
  forEach(
    callback: (value: T, value2: T, set: this) => void,
    thisArg?: unknown,
  ): void {
    for (const value of this) {
      callback.call(thisArg, value, value, this);
    }
  }
}
