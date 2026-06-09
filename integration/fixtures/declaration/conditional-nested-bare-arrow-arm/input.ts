const outer = true;
const inner = false;
const a = "a";
const b = "b";
const c = "c";
const x = outer ? (inner ? () => a : b) : c;
