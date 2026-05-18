# integration/fixtures/app-behavior/ast-type-coverage/ts-infer-type/input.ts

## Input

```ts
type T<X> = X extends Array<infer U> ? U : never;
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
```
