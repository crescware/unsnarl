const flag = true;

function inner() {
  if (flag) {
    if (flag) {
      const x = 1;
      console.log(x);
    }
  }
}

function callerOf() {
  inner();
}

function unrelated() {
  return 42;
}

callerOf();
unrelated();
