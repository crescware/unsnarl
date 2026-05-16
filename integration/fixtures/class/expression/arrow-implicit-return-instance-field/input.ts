const makeClass = (seed: number) =>
  class {
    x = seed;
  };

const C = makeClass(0);
