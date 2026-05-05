// basic
const a = 0;

// multiple declarators
const b = 1,
  c = 2;

// array pattern + default + hole + rest
const [d = 100, , e, ...f] = [3, , 5, 6, 7];

// object pattern + rename + default + rest
const { g, h: renamed = 200, ...others } = { g: 8, h: 9, x: 10, y: 11 };

// computed property name
const key = "dynamic";
const { [key]: dynamicValue } = { dynamic: 12 };

// computed property name (Symbol) + rename + default
const sym = Symbol("id");
const { [sym]: id = "default" } = { [sym]: "abc" };

// non-identifier property name
const { "kebab-case": kebab, 0: zeroth } = { "kebab-case": 13, 0: 14 };

// nested (array inside object) + default
const { nested: [p = 0, q] = [] } = { nested: [15, 16] };

// nested (object inside array)
const [{ r, s = 0 }, [t, u]] = [{ r: 1, s: 2 }, [3, 4]];

console.log(
  a,
  b,
  c,
  d,
  e,
  f,
  g,
  renamed,
  others,
  dynamicValue,
  id,
  kebab,
  zeroth,
  p,
  q,
  r,
  s,
  t,
  u,
);
