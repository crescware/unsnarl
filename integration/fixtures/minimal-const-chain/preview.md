# minimal-const-chain

## Input (`input.ts`)

```ts
const a = "a";
const b = [a];
const c = { value: b };
const d = c;
```

## Mermaid

```mermaid
flowchart RL
  n_scope_0_a_6["a : Variable<br/>L1"]
  n_scope_0_b_21["b : Variable<br/>L2"]
  n_scope_0_c_36["c : Variable<br/>L3"]
  n_scope_0_d_60["d : Variable<br/>L4"]
  n_scope_0_a_6 -->|read| n_scope_0_b_21
  n_scope_0_b_21 -->|read| n_scope_0_c_36
  n_scope_0_c_36 -->|read| n_scope_0_d_60
  classDef unused stroke-dasharray: 5 5;
  class n_scope_0_d_60 unused;
```
