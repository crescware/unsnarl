function makeFactory(seed: number) {
  return class {
    next() {
      return seed;
    }
  };
}

const C = makeFactory(0);
