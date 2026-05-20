function classify(kind: string, force: boolean) {
  let label = "";

  outer: switch (kind) {
    case "a":
      if (force) {
        return "alpha-forced";
      } else {
        break outer;
      }
    default:
      label = "other";
  }

  return label;
}

const result = classify("a", true);
