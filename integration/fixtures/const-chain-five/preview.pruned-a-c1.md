# integration/fixtures/const-chain-five/input.ts

## Input

```ts
const a = 1;
const b = a;
const c = b;
const d = c;
const e = d;
```

## Query

```sh
-r a -C 1
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  %% pruning: roots=[a(1)] ancestors=1 descendants=1
  n_scope_0_a_6["a<br/>L1"]
  n_scope_0_b_19["b<br/>L2"]
  n_scope_0_a_6 -->|read| n_scope_0_b_19
```
