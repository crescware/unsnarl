let label = "";
const kind = "a";

switch (kind) {
  case "a":
    label = "alpha";
  case "b":
    label = "beta";
    break;
  default:
    label = "other";
}

const result = label;
