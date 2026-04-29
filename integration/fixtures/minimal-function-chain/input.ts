function f() {
  const a = "a";
  const b = [a];
  const c = { value: b };
  const d = c;
  return d;
}
