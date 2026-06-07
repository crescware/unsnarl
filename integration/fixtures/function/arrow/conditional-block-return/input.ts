const left = "L";
const right = "R";

const pick = (flag: boolean) => {
  return flag ? left : right;
};

const result = pick(true);
