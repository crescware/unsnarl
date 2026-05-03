# integration/fixtures/lprefix-no-collision/input.ts

## Input

```ts
const alpha = 1;
const beta = alpha + 2;
const gamma = beta * 3;
```

## Query

```sh
-r L12 -C 1
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  %% pruning roots L12=0 ancestors=1 descendants=1
  %% pruning warning query L12 matched 0 roots
```
