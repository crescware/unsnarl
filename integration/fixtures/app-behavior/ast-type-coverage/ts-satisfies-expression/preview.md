# integration/fixtures/app-behavior/ast-type-coverage/ts-satisfies-expression/input.ts

## Input

```ts
const x = a satisfies string;
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_x_6["unused x<br/>L1"]
  n_scope_0_a_10["global a"]
  n_scope_0_a_10 -->|read| n_scope_0_x_6
```
