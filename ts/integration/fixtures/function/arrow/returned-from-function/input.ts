function f(a: number) {
  return () => {
    return a;
  };
}

const g = f(0);
const v = g();
