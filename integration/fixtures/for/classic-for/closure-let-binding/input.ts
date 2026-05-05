const fns: (() => number)[] = [];
for (let q = 0; q < 3; q++) {
  fns.push(() => q);
}
console.log(fns.length);
