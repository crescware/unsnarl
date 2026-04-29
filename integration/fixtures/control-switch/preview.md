# control-switch

## Input (`input.ts`)

```ts
let label = "";
const kind = "a";

switch (kind) {
  case "a":
    label = "alpha";
    break;
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
  n_scope_0_result_179["result<br/>L15"]
  subgraph s_scope_1["switch L4"]
    direction RL
    wr_ref_1(["let label<br/>L6"])
    wr_ref_2(["let label<br/>L9"])
    wr_ref_3(["let label<br/>L12"])
  end
  n_scope_0_label_4 -->|set| wr_ref_1
  wr_ref_1 -->|set| wr_ref_2
  wr_ref_2 -->|set| wr_ref_3
  n_scope_0_kind_22 -->|read| module_root
  wr_ref_3 -->|read| n_scope_0_result_179
  module_root((module))
  classDef unused stroke-dasharray: 5 5;
  class n_scope_0_result_179 unused;
```
