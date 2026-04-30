# integration/fixtures/control-switch-break/input.ts

## Input

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
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_label_4["let label<br/>L1"]
  n_scope_0_kind_22["kind<br/>L2"]
  n_scope_0_result_179["unused result<br/>L15"]
  subgraph s_scope_1["switch L4-13"]
    direction RL
    subgraph s_scope_2["case &quot;a&quot; L5-7"]
      direction RL
      wr_ref_1(["let label<br/>L6"])
    end
    subgraph s_scope_3["case &quot;b&quot; L8-10"]
      direction RL
      wr_ref_2(["let label<br/>L9"])
    end
    subgraph s_scope_4["default L11-12"]
      direction RL
      wr_ref_3(["let label<br/>L12"])
    end
  end
  n_scope_0_label_4 -->|set| wr_ref_1
  n_scope_0_label_4 -->|set| wr_ref_2
  n_scope_0_label_4 -->|set| wr_ref_3
  n_scope_0_kind_22 -->|read| s_scope_1
  wr_ref_1 -->|read| n_scope_0_result_179
  wr_ref_2 -->|read| n_scope_0_result_179
  wr_ref_3 -->|read| n_scope_0_result_179
```
