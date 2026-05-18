function classify(kind: string, force: boolean) {
  let label = "";

  switch (kind) {
    case "a":
      if (force) {
        return "alpha-forced";
      } else {
        break;
      }
    default:
      label = "other";
  }

  return label;
}

const result = classify("a", true);
