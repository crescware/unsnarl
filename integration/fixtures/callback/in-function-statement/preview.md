# integration/fixtures/callback/in-function-statement/input.ts

## Input

```ts
const arr = [1, 2, 3];
function f() {
  arr.forEach((v) => v + 1);
}
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_arr_6["arr<br/>L1"]
  subgraph wrap_s_scope_1[" "]
    direction TB
    n_scope_0_f_32["unused f()<br/>L2"]
    subgraph s_scope_1["f()<br/>L2-4"]
      direction RL
      subgraph expr_stmt_40["arr.forEach()<br/>L3"]
        direction RL
        subgraph s_scope_2["arr.forEach(args[0])<br/>L3"]
          direction RL
          n_scope_2_v_53["v<br/>L3"]
          subgraph s_return_scope_0_f_32_59_64["return L3"]
            direction RL
            ret_use_ref_2["v<br/>L3"]
          end
        end
      end
    end
  end
  n_scope_0_arr_6 -->|read| expr_stmt_40
  n_scope_2_v_53 -->|read| ret_use_ref_2
  classDef nestL1 fill:#11192a,stroke:transparent;
  class wrap_s_scope_1 nestL1;
  classDef nestL2 fill:#1a2538,stroke:transparent;
  class s_scope_1 nestL2;
  classDef nestL3 fill:#243047,stroke:transparent;
  class expr_stmt_40 nestL3;
  classDef nestL4 fill:#2d3b57,stroke:transparent;
  class s_scope_2 nestL4;
  classDef nestL5 fill:#364666,stroke:transparent;
  class s_return_scope_0_f_32_59_64 nestL5;
  classDef edgeTargetSubgraph stroke:#888;
  class expr_stmt_40 edgeTargetSubgraph;
```
