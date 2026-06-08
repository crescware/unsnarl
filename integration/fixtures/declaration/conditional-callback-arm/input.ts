const items = [1, 2, 3];
const fallback = [0];
const enabled = true;

const result = enabled ? items.map((v) => v * 2) : fallback;
