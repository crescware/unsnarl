// basic
var a = 0;

// multiple declarators
var b = 1,
  c = 2;

// array pattern + default + hole + rest
var [d = 100, , e, ...f] = [3, , 5, 6, 7];

// object pattern + rename + default + rest
var { g, h: renamed = 200, ...others } = { g: 8, h: 9, x: 10, y: 11 };

// non-identifier property name
var { "kebab-case": kebab, 0: zeroth } = { "kebab-case": 13, 0: 14 };

// nested (array inside object) + default
var { nested: [p = 0, q] = [] } = { nested: [15, 16] };

// nested (object inside array)
var [{ r, s = 0 }, [t, u]] = [{ r: 1, s: 2 }, [3, 4]];

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
  kebab,
  zeroth,
  p,
  q,
  r,
  s,
  t,
  u,
);
