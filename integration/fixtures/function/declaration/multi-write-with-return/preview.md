# integration/fixtures/function/declaration/multi-write-with-return/input.ts

## Input

```ts
function f() {
  let v = 0;
  v = 1;
  v = 2;
  return v;
}
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  subgraph wrap_s_scope_1[" "]
    direction TB
    n_scope_0_f_9["unused f()<br/>L1"]
    subgraph s_scope_1["f()<br/>L1-6"]
      direction RL
      n_scope_1_v_21["let v<br/>L2"]
      wr_ref_1(["let v<br/>L3"])
      wr_ref_2(["let v<br/>L4"])
      subgraph s_return_scope_0_f_9_48_57["return L5"]
        direction RL
        ret_use_ref_3["v<br/>L5"]
      end
    end
  end
  n_scope_1_v_21 -->|set| wr_ref_1
  wr_ref_1 -->|set| wr_ref_2
  wr_ref_2 -->|read| ret_use_ref_3
  classDef nestL1 fill:#11192a,stroke:transparent;
  class wrap_s_scope_1 nestL1;
  classDef nestL2 fill:#1a2538,stroke:transparent;
  class s_scope_1 nestL2;
  classDef nestL3 fill:#243047,stroke:transparent;
  class s_return_scope_0_f_9_48_57 nestL3;
```
