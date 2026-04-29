# minimal-const-array

## Input (`input.ts`)

```ts
const a = "a";
const b = [a];
```

## Mermaid

```mermaid
flowchart LR
  n_scope_0_a_6["a : Variable\nL1"]
  n_scope_0_b_21["b : Variable\nL2"]
  n_scope_0_b_21 -->|read| n_scope_0_a_6
  classDef unused stroke-dasharray: 5 5;
  class n_scope_0_b_21 unused;
```
