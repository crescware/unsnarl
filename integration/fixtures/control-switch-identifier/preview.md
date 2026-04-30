# integration/fixtures/control-switch-identifier/input.ts

## Input

```ts
const RED = "r";
const GREEN = "g";
const BLUE = "b";

let label = "";
const color = RED;

switch (color) {
  case RED:
    label = "red";
    break;
  case GREEN:
    label = "green";
    break;
  case BLUE:
    label = "blue";
    break;
  default:
    label = "unknown";
}

const result = label;
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_RED_6["RED<br/>L1"]
  n_scope_0_GREEN_23["GREEN<br/>L2"]
  n_scope_0_BLUE_42["BLUE<br/>L3"]
  n_scope_0_label_59["let label<br/>L5"]
  n_scope_0_color_77["color<br/>L6"]
  n_scope_0_result_283["unused result<br/>L22"]
  subgraph s_scope_1["switch L8-20"]
    direction RL
    subgraph s_scope_2["case RED L9-11"]
      direction RL
      wr_ref_3(["let label<br/>L10"])
    end
    subgraph s_scope_3["case GREEN L12-14"]
      direction RL
      wr_ref_5(["let label<br/>L13"])
    end
    subgraph s_scope_4["case BLUE L15-17"]
      direction RL
      wr_ref_7(["let label<br/>L16"])
    end
    subgraph s_scope_5["default L18-19"]
      direction RL
      wr_ref_8(["let label<br/>L19"])
    end
  end
  n_scope_0_label_59 -->|set| wr_ref_3
  n_scope_0_label_59 -->|set| wr_ref_5
  n_scope_0_label_59 -->|set| wr_ref_7
  n_scope_0_label_59 -->|set| wr_ref_8
  n_scope_0_RED_6 -->|read| n_scope_0_color_77
  n_scope_0_color_77 -->|read| s_scope_1
  n_scope_0_RED_6 -->|read| module_root
  n_scope_0_GREEN_23 -->|read| module_root
  n_scope_0_BLUE_42 -->|read| module_root
  wr_ref_3 -->|read| n_scope_0_result_283
  wr_ref_5 -->|read| n_scope_0_result_283
  wr_ref_7 -->|read| n_scope_0_result_283
  wr_ref_8 -->|read| n_scope_0_result_283
  module_root((module))
```
