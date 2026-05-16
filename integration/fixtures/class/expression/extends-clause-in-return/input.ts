function makeSubclass(Base: new () => unknown) {
  return class extends Base {};
}

const C = makeSubclass(Object);
