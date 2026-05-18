async function asyncLoop() {
  async function* gen() {
    yield 1;
    yield 2;
  }
  for await (const v of gen()) {
    console.log(v);
  }
}
