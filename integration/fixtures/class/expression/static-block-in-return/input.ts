function makeClass(seed: number) {
  return class {
    static {
      seed;
    }
  };
}

const C = makeClass(0);
