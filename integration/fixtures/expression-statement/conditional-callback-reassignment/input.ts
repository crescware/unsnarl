let result = [0];
const items = [1, 2, 3];
const fallback = [9];
const enabled = true;

result = enabled ? items.map((v) => v * 2) : fallback;
