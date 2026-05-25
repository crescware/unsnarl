const seed = 1;

const a = seed + 1;
const b = a + 1;
const c = b + 1;

function compute(x) {
  return x + seed;
}

const d = compute(c);
