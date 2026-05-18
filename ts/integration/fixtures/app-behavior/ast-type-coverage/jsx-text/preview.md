# integration/fixtures/app-behavior/ast-type-coverage/jsx-text/input.tsx

## Input

```tsx
const x = (<a>hello</a>);
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_x_6["unused x<br/>L1"]
  n_scope_0_a_12["global a"]
  n_scope_0_a_12 -->|read| n_scope_0_x_6
```
