# integration/fixtures/app-behavior/ast-type-coverage/ts-mapped-type/input.ts

## Input

```ts
type T<O> = { [K in keyof O]: O[K] };
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
```
