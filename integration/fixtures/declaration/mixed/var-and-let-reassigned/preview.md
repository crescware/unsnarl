# integration/fixtures/declaration/mixed/var-and-let-reassigned/input.ts

## Notice

```
uns: warning: L1:0: var declaration detected; rendered as node only (no edges).
```

## Input

```ts
var a = 0;
a = 1;
let b = 2;
b = 3;
console.log(a, b);
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_a_4["var a<br/>L1"]
  n_scope_0_b_22["let b<br/>L3"]
  n_scope_0_console_36["global console"]
  wr_ref_1(["let b<br/>L4"])
  n_scope_0_b_22 -->|set| wr_ref_1
  n_scope_0_console_36 -->|read| expr_stmt_36
  wr_ref_1 -->|read| expr_stmt_36
  expr_stmt_36["console.log()<br/>L5"]
  classDef varNode stroke-dasharray:5 5;
  class n_scope_0_a_4 varNode;
```
