# integration/fixtures/control-if/input.ts

## Input

```ts
let counter = 0;
const flag = true;

if (flag) {
  counter = 1;
} else {
  counter = 2;
}

const result = counter;
```

## Query

```sh
-r counter -A 0 -B 2
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  %% pruning roots counter=1 ancestors=2 descendants=0
  n_scope_0_counter_4["let counter<br/>L1"]
```
