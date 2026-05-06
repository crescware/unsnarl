function g(): unknown {
  try {
    throw new Error("oops");
  } catch (e) {
    e = "rewritten";
    return e;
  }
}
