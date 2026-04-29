function classify(kind: string) {
  let label = "";

  switch (kind) {
    case "a":
      label = "alpha";
      return label;
    case "b":
      label = "beta";
      break;
    default:
      label = "other";
  }

  return label;
}

const result = classify("a");
