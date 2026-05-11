# integration/fixtures/iteration-statement/for-of/basic/input.ts

## Input

```ts
for (const item of [1, 2, 3]) {
  console.log(item);
}
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_console_34["global console"]
  subgraph s_scope_1["for L1-3"]
    direction RL
    for_test_scope_0_0["for ()<br/>L1"]
    n_scope_1_item_11["item<br/>L1"]
    subgraph s_scope_2["block L1-3"]
      direction RL
      expr_stmt_34["console.log()<br/>L2"]
    end
  end
  n_scope_0_console_34 -->|read| expr_stmt_34
  n_scope_1_item_11 -->|read| expr_stmt_34
  classDef nestL1 fill:#1e2738,stroke:#3d4a63;
  class s_scope_1 nestL1;
  classDef nestL2 fill:#233045,stroke:#475670;
  class s_scope_2 nestL2;
```
