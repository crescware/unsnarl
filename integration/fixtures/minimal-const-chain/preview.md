# integration/fixtures/minimal-const-chain/input.ts

## Input

```ts
const a = "a";
const b = [a];
const c = { value: b };
const d = c;
```

## Mermaid

```mermaid
flowchart RL
  n_scope_0_a_6["a<br/>L1"]
  n_scope_0_b_21["b<br/>L2"]
  n_scope_0_c_36["c<br/>L3"]
  n_scope_0_d_60["d<br/>L4"]
  n_scope_0_a_6 -->|read| n_scope_0_b_21
  n_scope_0_b_21 -->|read| n_scope_0_c_36
  n_scope_0_c_36 -->|read| n_scope_0_d_60
  classDef unused stroke-dasharray: 5 5;
  class n_scope_0_d_60 unused;
```
