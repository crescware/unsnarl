# integration/fixtures/declaration/var/with-implicit-global-and-read/input.ts

## Notice

```
uns: warning: L1:0: var declaration detected; rendered as node only (no edges).
```

## Input

```ts
var a = 0;
global = 1;
console.log(a, global);
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_a_4["var a<br/>L1"]
  n_scope_0_global_11["global global"]
  n_scope_0_console_23["global console"]
  wr_ref_0(["global<br/>L2"])
  n_scope_0_global_11 -->|set| wr_ref_0
  n_scope_0_console_23 -->|read| expr_stmt_23
  wr_ref_0 -->|read| expr_stmt_23
  expr_stmt_23["console.log()<br/>L3"]
  classDef varNode stroke-dasharray:5 5;
  class n_scope_0_a_4 varNode;
```
