#!/usr/bin/env node
import { main } from "./cli/main.js";

main(process.argv.slice(2)).then((code) => {
  process.exit(code);
});
