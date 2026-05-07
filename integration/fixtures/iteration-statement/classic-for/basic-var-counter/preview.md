# integration/fixtures/iteration-statement/classic-for/basic-var-counter/input.ts

## Notice

```
uns: warning: L1:5: var declaration detected; rendered as node only (no edges).
```

## Input

```ts
for (var i = 0; i < 3; i++) {
  console.log(i);
}
console.log(i);
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_i_9["var i<br/>L1"]
  n_scope_0_console_32["global console"]
  subgraph s_scope_1["for L1-3"]
    direction RL
    for_test_scope_0_0["for ()<br/>L1"]
    subgraph s_scope_2["block L1-3"]
      direction RL
      expr_stmt_32["console.log()<br/>L2"]
    end
  end
  n_scope_0_console_32 -->|read| expr_stmt_32
  n_scope_0_console_32 -->|read| expr_stmt_50
  expr_stmt_50["console.log()<br/>L4"]
  classDef varNode stroke-dasharray:5 5;
  class n_scope_0_i_9 varNode;
```
