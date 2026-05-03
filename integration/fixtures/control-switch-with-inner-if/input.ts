function classify(n: number, big: boolean) {
  let label;
  switch (n) {
    case 0:
      if (big) {
        label = "zero-big";
      } else {
        label = "zero-small";
      }
      break;
    case 1:
      label = "one";
      break;
    default:
      label = "other";
  }
  return label;
}

const result = classify(0, true);
