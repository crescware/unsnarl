# control-try

## Input (`input.ts`)

```ts
let value = 0;
let attempts = 0;
const raw = "42";

try {
  attempts = 1;
  value = Number(raw);
} catch (err) {
  attempts = -1;
  value = -1;
} finally {
  attempts += 1;
}

const result = value + attempts;
```

## Mermaid

```mermaid
flowchart RL
  n_scope_0_value_4["let value<br/>L1"]
  n_scope_0_attempts_19["let attempts<br/>L2"]
  n_scope_0_raw_39["raw<br/>L3"]
  n_scope_0_result_182["result<br/>L15"]
  n_scope_0_Number_84["global Number<br/>L7"]
  subgraph s_scope_1["try L5"]
    direction RL
    wr_ref_1(["let value<br/>L7"])
    wr_ref_0(["let attempts<br/>L6"])
  end
  subgraph s_scope_2["catch L8"]
    direction RL
    n_scope_2_err_106["catch err<br/>L8"]
    wr_ref_5(["let value<br/>L10"])
    wr_ref_4(["let attempts<br/>L9"])
  end
  subgraph s_scope_3["finally L11"]
    direction RL
    wr_ref_6(["let attempts<br/>L12"])
  end
  n_scope_0_value_4 -->|set| wr_ref_1
  wr_ref_1 -->|set| wr_ref_5
  n_scope_0_attempts_19 -->|set| wr_ref_0
  wr_ref_0 -->|set| wr_ref_4
  wr_ref_4 -->|set| wr_ref_6
  n_scope_0_Number_84 -->|read,call| wr_ref_1
  n_scope_0_raw_39 -->|read| wr_ref_1
  wr_ref_5 -->|read| n_scope_0_result_182
  wr_ref_6 -->|read| n_scope_0_result_182
  classDef unused stroke-dasharray: 5 5;
  class n_scope_0_result_182 unused;
  class n_scope_2_err_106 unused;
```
