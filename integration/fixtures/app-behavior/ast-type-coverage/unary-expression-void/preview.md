# integration/fixtures/app-behavior/ast-type-coverage/unary-expression-void/input.ts

## Input

```ts
const x = void a;
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_x_6["unused x<br/>L1"]
  n_scope_0_a_15["global a"]
  n_scope_0_a_15 -->|read| n_scope_0_x_6
```
