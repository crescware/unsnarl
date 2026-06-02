declare function wrap(xs: number[]): number[];
const items = [1, 2, 3];
const wrapped = wrap(items.map((v) => v + 1));
