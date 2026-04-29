# minimal-function-chain

## Input (`input.ts`)

```ts
function f() {
  const a = "a";
  const b = [a];
  const c = { value: b };
  const d = c;
  return d;
}
```

## Mermaid

```mermaid
flowchart RL
  subgraph n_scope_0_f_9["f : FunctionName<br/>L1"]
    direction RL
    return_scope_0_f_9((return))
    n_scope_1_a_23["a : Variable<br/>L2"]
    n_scope_1_b_40["b : Variable<br/>L3"]
    n_scope_1_c_57["c : Variable<br/>L4"]
    n_scope_1_d_83["d : Variable<br/>L5"]
  end
  n_scope_1_a_23 -->|read| n_scope_1_b_40
  n_scope_1_b_40 -->|read| n_scope_1_c_57
  n_scope_1_c_57 -->|read| n_scope_1_d_83
  n_scope_1_d_83 -->|read| return_scope_0_f_9
  classDef unused stroke-dasharray: 5 5;
  class n_scope_0_f_9 unused;
```
