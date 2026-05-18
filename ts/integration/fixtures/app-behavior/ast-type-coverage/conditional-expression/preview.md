# integration/fixtures/app-behavior/ast-type-coverage/conditional-expression/input.ts

## Input

```ts
const x = cond ? a : b;
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_x_6["unused x<br/>L1"]
  n_scope_0_cond_10["global cond"]
  n_scope_0_a_17["global a"]
  n_scope_0_b_21["global b"]
  n_scope_0_cond_10 -->|read| n_scope_0_x_6
  n_scope_0_a_17 -->|read| n_scope_0_x_6
  n_scope_0_b_21 -->|read| n_scope_0_x_6
```
