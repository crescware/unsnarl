const flag = true;
const a = "a";
const b = "b";
const handler = flag
  ? function () {
      return a;
    }
  : function () {
      return b;
    };
