function check(flag: boolean) {
  const a = new Error("a");
  const b = new Error("b");
  throw flag ? a : b;
}

check(true);
