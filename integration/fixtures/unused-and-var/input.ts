import { used, neverCalled } from "module";

var legacy = 1;
const a = used;
const ignored = 99;

console.log(a);
