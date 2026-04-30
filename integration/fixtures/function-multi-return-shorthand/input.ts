function main() {
  const a = "a";

  if (Math.random() < 0.5) {
    const b = "b0";
    return { a, b };
  }

  const b = "b1";
  return { a, b };
}
