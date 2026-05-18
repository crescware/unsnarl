# integration/fixtures/declaration/var/redeclaration/input.ts

## Notice

```
uns: warning: L1:0: var declaration detected; rendered as node only (no edges).
uns: warning: L2:0: var declaration detected; rendered as node only (no edges).
```

## Input

```ts
var x = 1;
var x = 2;
console.log(x);
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_x_4["var x<br/>L1"]
  n_scope_0_console_22["global console"]
  n_scope_0_console_22 -->|read| expr_stmt_22
  expr_stmt_22["console.log()<br/>L3"]
  classDef varNode stroke-dasharray:5 5;
  class n_scope_0_x_4 varNode;
```
