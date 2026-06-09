function pick(flag: boolean, items: number[], fallback: number[]) {
  return flag ? items.map((v) => v * 2) : fallback;
}

const result = pick(true, [1, 2, 3], [0]);
