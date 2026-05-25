function makeClass(seed: number) {
  return class {
    static x = seed;
  };
}

const C = makeClass(0);
