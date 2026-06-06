function pick(flag: boolean) {
  const left = "yes";
  const right = "no";
  return flag ? left : right;
}

const result = pick(true);
