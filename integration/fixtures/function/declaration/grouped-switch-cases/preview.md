# integration/fixtures/function/declaration/grouped-switch-cases/input.ts

## Input

```ts
function getDayType(day) {
  let ret = "";
  switch (day.toLowerCase()) {
    case "mon":
    case "tue":
    case "wed":
    case "thu":
    case "fri":
      ret = "weekday";
      break;
    case "sat":
    case "sun":
      ret = "weekend";
      break;
    default:
      ret = null;
      break;
  }
  return ret;
}
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  subgraph wrap_s_scope_1[" "]
    direction TB
    n_scope_0_getDayType_9["unused getDayType()<br/>L1"]
    subgraph s_scope_1["getDayType()<br/>L1-20"]
      direction RL
      n_scope_1_day_20["day<br/>L1"]
      n_scope_1_ret_33["let ret<br/>L2"]
      subgraph s_scope_2["switch L3-18"]
        direction RL
        switch_discriminant_scope_1_45{"switch ()<br/>L3"}
        subgraph s_scope_3["case &quot;mon&quot; L4"]
          direction RL
          elk_empty_s_scope_3["No nodes"]
        end
        subgraph s_scope_4["case &quot;tue&quot; L5"]
          direction RL
          elk_empty_s_scope_4["No nodes"]
        end
        subgraph s_scope_5["case &quot;wed&quot; L6"]
          direction RL
          elk_empty_s_scope_5["No nodes"]
        end
        subgraph s_scope_6["case &quot;thu&quot; L7"]
          direction RL
          elk_empty_s_scope_6["No nodes"]
        end
        subgraph s_scope_7["case &quot;fri&quot; L8-10"]
          direction RL
          wr_ref_2(["let ret<br/>L9"])
          bc_break_183["break<br/>L10"]
        end
        subgraph s_scope_8["case &quot;sat&quot; L11"]
          direction RL
          elk_empty_s_scope_8["No nodes"]
        end
        subgraph s_scope_9["case &quot;sun&quot; L12-14"]
          direction RL
          wr_ref_3(["let ret<br/>L13"])
          bc_break_251["break<br/>L14"]
        end
        subgraph s_scope_10["default L15-17"]
          direction RL
          wr_ref_4(["let ret<br/>L16"])
          bc_break_295["break<br/>L17"]
        end
      end
      subgraph s_return_scope_0_getDayType_9_308_319["return L19"]
        direction RL
        ret_use_ref_5["ret<br/>L19"]
      end
    end
  end
  n_scope_1_ret_33 -->|fallthrough| wr_ref_2
  n_scope_1_ret_33 -->|fallthrough| wr_ref_3
  n_scope_1_ret_33 -->|set| wr_ref_4
  n_scope_1_day_20 -->|read| switch_discriminant_scope_1_45
  wr_ref_2 -->|read| ret_use_ref_5
  wr_ref_3 -->|read| ret_use_ref_5
  wr_ref_4 -->|read| ret_use_ref_5
  classDef nestL1 fill:#11192a,stroke:transparent;
  class wrap_s_scope_1 nestL1;
  classDef nestL2 fill:#1a2538,stroke:transparent;
  class s_scope_1 nestL2;
  classDef nestL3 fill:#243047,stroke:transparent;
  class s_scope_2 nestL3;
  class s_return_scope_0_getDayType_9_308_319 nestL3;
  classDef nestL4 fill:#2d3b57,stroke:transparent;
  class s_scope_3 nestL4;
  class s_scope_4 nestL4;
  class s_scope_5 nestL4;
  class s_scope_6 nestL4;
  class s_scope_7 nestL4;
  class s_scope_8 nestL4;
  class s_scope_9 nestL4;
  class s_scope_10 nestL4;
  classDef elkEmptyPlaceholder fill:transparent,stroke:transparent;
  class elk_empty_s_scope_3 elkEmptyPlaceholder;
  class elk_empty_s_scope_4 elkEmptyPlaceholder;
  class elk_empty_s_scope_5 elkEmptyPlaceholder;
  class elk_empty_s_scope_6 elkEmptyPlaceholder;
  class elk_empty_s_scope_8 elkEmptyPlaceholder;
```
