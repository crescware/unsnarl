# integration/fixtures/app-behavior/ast-type-coverage/sequence-expression/input.ts

## Input

```ts
for (let i = 0; i < 10; i++, i++) {
  console.log(i);
}
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_console_38["global console"]
  subgraph s_scope_1["for L1-3"]
    direction RL
    for_test_scope_0_0["for ()<br/>L1"]
    n_scope_1_i_9["let i<br/>L1"]
    wr_ref_2(["let i<br/>L1"])
    wr_ref_3(["let i<br/>L1"])
    subgraph s_scope_2["block L1-3"]
      direction RL
      expr_stmt_38["console.log()<br/>L2"]
    end
  end
  n_scope_1_i_9 -->|set| wr_ref_2
  wr_ref_2 -->|set| wr_ref_3
  n_scope_1_i_9 -->|read| for_test_scope_0_0
  n_scope_0_console_38 -->|read| expr_stmt_38
  wr_ref_3 -->|read| expr_stmt_38
  classDef nestL1 fill:#11192a,stroke:transparent;
  class s_scope_1 nestL1;
  classDef nestL2 fill:#1a2538,stroke:transparent;
  class s_scope_2 nestL2;
```
