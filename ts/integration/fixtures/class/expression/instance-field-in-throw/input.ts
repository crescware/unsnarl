function failWith(seed: number) {
  throw class {
    x = seed;
  };
}

failWith(0);
