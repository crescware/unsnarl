# integration/fixtures/const-chain-five/input.ts

## Input

```ts
const a = 1;
const b = a;
const c = b;
const d = c;
const e = d;
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_a_6["a<br/>L1"]
  n_scope_0_b_19["b<br/>L2"]
  n_scope_0_c_32["c<br/>L3"]
  n_scope_0_d_45["d<br/>L4"]
  n_scope_0_e_58["unused e<br/>L5"]
  n_scope_0_a_6 -->|read| n_scope_0_b_19
  n_scope_0_b_19 -->|read| n_scope_0_c_32
  n_scope_0_c_32 -->|read| n_scope_0_d_45
  n_scope_0_d_45 -->|read| n_scope_0_e_58
```
