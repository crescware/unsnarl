function makeClass(key: string) {
  return class {
    [key] = 0;
  };
}

const C = makeClass("x");
