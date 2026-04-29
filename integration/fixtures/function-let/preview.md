# function-let

## Input (`input.ts`)

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
flowchart RL
  subgraph n_scope_0_f_9["f()<br/>L1"]
    direction RL
    return_scope_0_f_9((return))
    n_scope_1_v_21["v<br/>L2"]
    wr_ref_0(["v<br/>L3"])
    wr_ref_1(["v<br/>L4"])
  end
  n_scope_1_v_21 -->|set| wr_ref_0
  wr_ref_0 -->|set| wr_ref_1
  wr_ref_1 -->|read| return_scope_0_f_9
  classDef unused stroke-dasharray: 5 5;
  class n_scope_0_f_9 unused;
```
