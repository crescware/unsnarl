# integration/fixtures/app-behavior/ast-type-coverage/ts-undefined-keyword/input.ts

## Input

```ts
const x: undefined = undefined;
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_x_6["unused x<br/>L1"]
  n_scope_0_undefined_21["global undefined"]
  n_scope_0_undefined_21 -->|read| n_scope_0_x_6
```
