function pick(expr: boolean) {
  const a = "a";
  const b = "b";
  const ret = (() => (expr ? a : b))();
  return ret;
}

const result = pick(true);
