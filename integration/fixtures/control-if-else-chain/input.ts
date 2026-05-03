function classify(n: number) {
  let label;
  if (n > 0) {
    label = "positive";
  } else if (n < 0) {
    label = "negative";
  } else {
    label = "zero";
  }
  return label;
}

const result = classify(-1);
