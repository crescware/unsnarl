function f(a: number) {
  return function () {
    return a;
  };
}

const g = f(0);
const v = g();
