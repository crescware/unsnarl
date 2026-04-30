# integration/fixtures/function-and-class/input.ts

## Input

```ts
function add(a: number, b: number) {
  return a + b;
}

class Counter {
  start = 0;
}

const total = add(1, 2);
const c = new Counter();
const result = total;
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_Counter_62["class Counter<br/>L5"]
  n_scope_0_total_94["total<br/>L9"]
  n_scope_0_c_119["unused c<br/>L10"]
  n_scope_0_result_144["unused result<br/>L11"]
  subgraph wrap_s_scope_1[" "]
    direction TB
    n_scope_0_add_9["add()<br/>L1"]
    subgraph s_scope_1["add()<br/>L1"]
      direction RL
      return_scope_0_add_9((return))
      n_scope_1_a_13["a<br/>L1"]
      n_scope_1_b_24["b<br/>L1"]
    end
  end
  n_scope_1_a_13 -->|read| return_scope_0_add_9
  n_scope_1_b_24 -->|read| return_scope_0_add_9
  n_scope_0_add_9 -->|read,call| n_scope_0_total_94
  n_scope_0_Counter_62 -->|read,call| n_scope_0_c_119
  n_scope_0_total_94 -->|read| n_scope_0_result_144
  classDef fnWrap fill:#1a2030,stroke:#5a7d99;
  class wrap_s_scope_1 fnWrap;
```
