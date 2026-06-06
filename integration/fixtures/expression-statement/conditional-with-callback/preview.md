# integration/fixtures/expression-statement/conditional-with-callback/input.ts

## Input

```ts
const items = [1, 2, 3];
const enabled = true;

enabled ? items.map((v) => v * 2) : items;
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_items_6["items<br/>L1"]
  n_scope_0_enabled_31["enabled<br/>L2"]
  subgraph expr_stmt_48["enabled ? items.map((v) =&gt; v * 2) : items<br/>L4"]
    direction RL
    subgraph s_scope_1["items.map(args[0])<br/>L4"]
      direction RL
      n_scope_1_v_69["v<br/>L4"]
      subgraph s_return_scope_1_75_80["return L4"]
        direction RL
        ret_use_ref_4["v<br/>L4"]
      end
    end
  end
  n_scope_0_enabled_31 -->|read| expr_stmt_48
  n_scope_0_items_6 -->|read| expr_stmt_48
  n_scope_1_v_69 -->|read| ret_use_ref_4
  classDef nestL1 fill:#11192a,stroke:transparent;
  class expr_stmt_48 nestL1;
  classDef nestL2 fill:#1a2538,stroke:transparent;
  class s_scope_1 nestL2;
  classDef nestL3 fill:#243047,stroke:transparent;
  class s_return_scope_1_75_80 nestL3;
  classDef edgeTargetSubgraph stroke:#888;
  class expr_stmt_48 edgeTargetSubgraph;
```
