import { main } from "./src/main";

void main().catch((error: unknown) => {
  console.error(error);
  process.exit(1);
});
