# integration/fixtures/control-switch-return/input.ts

## Input

```ts
function classify(kind: string) {
  let label = "";

  switch (kind) {
    case "a":
      label = "alpha";
      return label;
    case "b":
      label = "beta";
      break;
    default:
      label = "other";
  }

  return label;
}

const result = classify("a");
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_result_243["unused result<br/>L18"]
  subgraph wrap_s_scope_1[" "]
    direction TB
    n_scope_0_classify_9["classify()<br/>L1"]
    subgraph s_scope_1["classify()<br/>L1"]
      direction RL
      return_scope_0_classify_9((return))
      n_scope_1_kind_18["kind<br/>L1"]
      n_scope_1_label_40["let label<br/>L2"]
      subgraph s_scope_2["switch L4"]
        direction RL
        subgraph s_scope_3["case &quot;a&quot; L5"]
          direction RL
          wr_ref_1(["let label<br/>L6"])
        end
        subgraph s_scope_4["case &quot;b&quot; L8"]
          direction RL
          wr_ref_3(["let label<br/>L9"])
        end
        subgraph s_scope_5["default L11"]
          direction RL
          wr_ref_4(["let label<br/>L12"])
        end
      end
    end
  end
  n_scope_1_label_40 -->|set| wr_ref_1
  n_scope_1_label_40 -->|set| wr_ref_3
  n_scope_1_label_40 -->|set| wr_ref_4
  n_scope_1_kind_18 -->|read| s_scope_2
  wr_ref_1 -->|read| return_scope_0_classify_9
  wr_ref_3 -->|read| return_scope_0_classify_9
  wr_ref_4 -->|read| return_scope_0_classify_9
  n_scope_0_classify_9 -->|read,call| n_scope_0_result_243
  classDef fnWrap fill:#1a2030,stroke:#5a7d99;
  class wrap_s_scope_1 fnWrap;
```
