function classify(x: number) {
  switch (x) {
    case 1:
      outer: return 1;
    case 2:
      outer: {
        return 2;
      }
    case 3:
      outer: throw new Error("three");
    default:
      return 0;
  }
}

const out = classify(1);
