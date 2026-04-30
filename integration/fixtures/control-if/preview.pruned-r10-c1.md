# integration/fixtures/control-if/input.ts

## Input

```ts
let counter = 0;
const flag = true;

if (flag) {
  counter = 1;
} else {
  counter = 2;
}

const result = counter;
```

## Query

```sh
-r 10 -C 1
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  %% pruning roots 10=2 ancestors=1 descendants=1
  n_scope_0_counter_4["let counter<br/>L1"]
  n_scope_0_result_97["unused result<br/>L10"]
  subgraph cont_if_scope_0_37["if-else L4-8"]
    direction RL
    subgraph s_scope_1["if L4-6"]
      direction RL
      wr_ref_1(["let counter<br/>L5"])
    end
    subgraph s_scope_2["else L6-8"]
      direction RL
      wr_ref_2(["let counter<br/>L7"])
    end
  end
  n_scope_0_counter_4 -->|set| wr_ref_1
  n_scope_0_counter_4 -->|set| wr_ref_2
  wr_ref_1 -->|read| n_scope_0_result_97
  wr_ref_2 -->|read| n_scope_0_result_97
```
