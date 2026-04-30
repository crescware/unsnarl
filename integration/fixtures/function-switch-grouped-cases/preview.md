# integration/fixtures/function-switch-grouped-cases/input.ts

## Input

```ts
function getDayType(day) {
  let ret = "";
  switch (day.toLowerCase()) {
    case 'mon':
    case 'tue':
    case 'wed':
    case 'thu':
    case 'fri':
      ret = 'weekday';
      break;
    case 'sat':
    case 'sun':
      ret = 'weekend';
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
      subgraph s_return_scope_0_getDayType_9["return L19"]
        direction RL
        ret_use_ref_4["ret<br/>L19"]
      end
      subgraph s_scope_2["switch L3-18"]
        direction RL
        subgraph s_scope_3["case 'mon' L4"]
          direction RL
        end
        subgraph s_scope_4["case 'tue' L5"]
          direction RL
        end
        subgraph s_scope_5["case 'wed' L6"]
          direction RL
        end
        subgraph s_scope_6["case 'thu' L7"]
          direction RL
        end
        subgraph s_scope_7["case 'fri' L8-10"]
          direction RL
          wr_ref_1(["let ret<br/>L9"])
        end
        subgraph s_scope_8["case 'sat' L11"]
          direction RL
        end
        subgraph s_scope_9["case 'sun' L12-14"]
          direction RL
          wr_ref_2(["let ret<br/>L13"])
        end
        subgraph s_scope_10["default L15-17"]
          direction RL
          wr_ref_3(["let ret<br/>L16"])
        end
      end
    end
  end
  n_scope_1_ret_33 -->|fallthrough| wr_ref_1
  n_scope_1_ret_33 -->|fallthrough| wr_ref_2
  n_scope_1_ret_33 -->|set| wr_ref_3
  n_scope_1_day_20 -->|read| s_scope_2
  wr_ref_1 -->|read| ret_use_ref_4
  wr_ref_2 -->|read| ret_use_ref_4
  wr_ref_3 -->|read| ret_use_ref_4
  classDef fnWrap fill:#1a2030,stroke:#5a7d99;
  class wrap_s_scope_1 fnWrap;
```
