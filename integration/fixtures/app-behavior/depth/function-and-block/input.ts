const a = true;
const b = true;
const c = true;

function f1() {
  if (a) {
    if (b) {
      function f2() {
        if (c) {
          const x = 1;
          console.log(x);
        }
      }
      f2();
    }
  }
}

f1();
