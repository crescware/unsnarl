# integration/fixtures/app-behavior/ast-type-coverage/import-expression/input.ts

## Input

```ts
const x = import("y");
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_x_6["unused x<br/>L1"]
```
