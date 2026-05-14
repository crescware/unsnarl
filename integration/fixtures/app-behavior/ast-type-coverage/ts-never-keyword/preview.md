# integration/fixtures/app-behavior/ast-type-coverage/ts-never-keyword/input.ts

## Input

```ts
const x: never = null as never;
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_x_6["unused x<br/>L1"]
```
