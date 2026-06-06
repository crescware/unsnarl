const left = "L";
const right = "R";

const pick = (enabled: boolean) => (enabled ? left : right);

const result = pick(true);
