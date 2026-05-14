function f(a = 1, ...rest: number[]) {
  return rest.length + a;
}

const result = f(2, 3, 4);
