# integration/fixtures/lprefix-other-match/input.ts

## Notice

```
uns: 'L12' is ambiguous.
  No exact identifier match was found; interpreting as line number.
  To disambiguate, use '-r 12'.
```

## Input

```ts
const l5 = 1;
const l99 = l5 + 2;
const sum = l5 + l99;
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
