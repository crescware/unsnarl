# integration/fixtures/app-behavior/expression-statement-head/member-assignment-literal-rhs/input.ts

## Input

```ts
const a = { z: 0 };
a.z = 1;
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_a_6["a<br/>L1"]
  n_scope_0_a_6 -->|read| expr_stmt_20
  expr_stmt_20["a.z = ...<br/>L2"]
```
