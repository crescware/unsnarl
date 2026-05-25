let label = "";
const a = 2;
const b = 3;
const target = 5;

switch (target) {
  case a + b:
    label = "sum";
    break;
  case a * b:
    label = "product";
    break;
  case Number.MAX_SAFE_INTEGER:
    label = "maximum-safe-integer-value-marker";
    break;
  default:
    label = "other";
}

const result = label;
