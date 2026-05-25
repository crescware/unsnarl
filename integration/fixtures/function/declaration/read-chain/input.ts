function f() {
  const a = "a";
  const b = [a];
  const c = { value: b };
  const d = c;
  return d;
}

const g = f();
const h = [g];
