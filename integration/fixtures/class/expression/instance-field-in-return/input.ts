function makeClass(seed: number) {
  return class {
    x = seed;
  };
}

const C = makeClass(0);
const c = new C();
