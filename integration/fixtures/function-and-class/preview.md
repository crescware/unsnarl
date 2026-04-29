# function-and-class

## Input (`input.ts`)

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
flowchart LR
  n_scope_0_add_9["add : FunctionName\nL1"]
  n_scope_0_Counter_62["Counter : ClassName\nL5"]
  n_scope_0_total_94["total : Variable\nL9"]
  n_scope_0_c_119["c : Variable\nL10"]
  n_scope_0_result_144["result : Variable\nL11"]
  n_scope_1_a_13["a : Parameter\nL1"]
  n_scope_1_b_24["b : Parameter\nL1"]
  n_scope_0_add_9 -->|read| n_scope_1_a_13
  n_scope_0_add_9 -->|read| n_scope_1_b_24
  n_scope_0_add_9 -->|read| n_scope_1_a_13
  n_scope_0_add_9 -->|read| n_scope_1_b_24
  n_scope_0_total_94 -->|read,call| n_scope_0_add_9
  n_scope_0_c_119 -->|read,call| n_scope_0_Counter_62
  n_scope_0_result_144 -->|read| n_scope_0_total_94
  classDef unused stroke-dasharray: 5 5;
  class n_scope_0_c_119 unused;
  class n_scope_0_result_144 unused;
```
