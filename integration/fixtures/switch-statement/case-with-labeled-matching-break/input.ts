function classify(x: number) {
  switch (x) {
    case 1:
      outer: {
        break outer;
      }
    case 2:
      return 2;
    default:
      return 0;
  }
}

const out = classify(1);
