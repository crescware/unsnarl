# integration/fixtures/control-switch-partial-break/input.ts

## Input

```ts
let label = "";
const kind = "a";

switch (kind) {
  case "a":
    label = "alpha";
  case "b":
    label = "beta";
    break;
  default:
    label = "other";
}

const result = label;
```

## Mermaid

```mermaid
flowchart RL
  n_scope_0_label_4["let label<br/>L1"]
  n_scope_0_kind_22["kind<br/>L2"]
  n_scope_0_result_168["result<br/>L14"]
  subgraph s_scope_1["switch L4"]
    direction RL
    subgraph s_scope_2["case L5"]
      direction RL
      wr_ref_1(["let label<br/>L6"])
    end
    subgraph s_scope_3["case L7"]
      direction RL
      wr_ref_2(["let label<br/>L8"])
    end
    subgraph s_scope_4["case L10"]
      direction RL
      wr_ref_3(["let label<br/>L11"])
    end
  end
  n_scope_0_label_4 -->|set| wr_ref_1
  wr_ref_1 -->|fallthrough| wr_ref_2
  n_scope_0_label_4 -->|set| wr_ref_3
  n_scope_0_kind_22 -->|read| s_scope_1
  wr_ref_2 -->|read| n_scope_0_result_168
  wr_ref_3 -->|read| n_scope_0_result_168
  classDef unused stroke-dasharray: 5 5;
  class n_scope_0_result_168 unused;
```
