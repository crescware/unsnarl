const items = [1, 2, 3];

const ids = items.map((v) => v + 1);

const out = run((v) => v - 1);

function run(cb: (n: number) => number) {
  return cb(0);
}
