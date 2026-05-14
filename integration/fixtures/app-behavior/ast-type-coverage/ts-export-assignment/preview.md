# integration/fixtures/app-behavior/ast-type-coverage/ts-export-assignment/input.ts

## Input

```ts
const x = 1;
export = x;
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_x_6["unused x<br/>L1"]
```
