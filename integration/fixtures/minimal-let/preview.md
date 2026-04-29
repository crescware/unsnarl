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
  n_scope_0_count_4 -->|write| n_scope_0_count_4
```
