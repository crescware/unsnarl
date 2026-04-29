let label = "";
const kind = "a";

switch (kind) {
  case "a":
  case "b":
    label = "ab";
    break;
  default:
    label = "other";
}

const result = label;
