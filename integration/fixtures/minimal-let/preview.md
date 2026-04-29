# minimal-let

## Input (`input.ts`)

```ts
let count = 0;
count = 1;
```

## Mermaid

```mermaid
flowchart RL
  n_scope_0_count_4["count<br/>L1"]
  wr_ref_0(["count<br/>L2"])
  n_scope_0_count_4 -->|set| wr_ref_0
```
