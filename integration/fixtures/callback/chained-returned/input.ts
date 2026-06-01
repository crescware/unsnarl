function compute(arr: number[]) {
  return arr.map((v) => v + 1).filter((v) => v > 0);
}
