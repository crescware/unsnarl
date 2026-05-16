function dec(value: unknown, _ctx: unknown) {
  return value;
}

function makeClass() {
  return class {
    @dec
    m() {}
  };
}

const C = makeClass();
