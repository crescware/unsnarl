const Counter = class Inner {
  next() {
    return new Inner();
  }
};

const c = new Counter();
