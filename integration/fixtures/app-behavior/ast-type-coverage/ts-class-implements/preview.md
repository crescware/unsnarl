# integration/fixtures/app-behavior/ast-type-coverage/ts-class-implements/input.ts

## Input

```ts
interface I {}
class C implements I {}
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_C_21["unused class C<br/>L2"]
  n_scope_1_C_21["unused class C<br/>L2"]
```
