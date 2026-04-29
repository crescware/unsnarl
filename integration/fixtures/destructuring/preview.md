# integration/fixtures/destructuring/input.ts

## Input

```ts
const source = { a: 1, b: 2, c: 3, nested: { d: 4 } };
const list = [10, 20, 30, 40];

const { a, b: renamed } = source;
const { nested: { d } } = source;
const [first, , third, ...rest] = list;
const { ...spread } = source;

const sum = a + renamed + d + first + third + rest.length + Object.keys(spread).length;
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_source_6["source<br/>L1"]
  n_scope_0_list_61["list<br/>L2"]
  n_scope_0_a_95["a<br/>L4"]
  n_scope_0_renamed_101["renamed<br/>L4"]
  n_scope_0_d_139["d<br/>L5"]
  n_scope_0_first_162["first<br/>L6"]
  n_scope_0_third_171["third<br/>L6"]
  n_scope_0_rest_181["rest<br/>L6"]
  n_scope_0_spread_206["spread<br/>L7"]
  n_scope_0_sum_232["sum<br/>L9"]
  n_scope_0_source_6 -->|read| n_scope_0_a_95
  n_scope_0_source_6 -->|read| n_scope_0_renamed_101
  n_scope_0_source_6 -->|read| n_scope_0_d_139
  n_scope_0_list_61 -->|read| n_scope_0_first_162
  n_scope_0_list_61 -->|read| n_scope_0_third_171
  n_scope_0_list_61 -->|read| n_scope_0_rest_181
  n_scope_0_source_6 -->|read| n_scope_0_spread_206
  n_scope_0_a_95 -->|read| n_scope_0_sum_232
  n_scope_0_renamed_101 -->|read| n_scope_0_sum_232
  n_scope_0_d_139 -->|read| n_scope_0_sum_232
  n_scope_0_first_162 -->|read| n_scope_0_sum_232
  n_scope_0_third_171 -->|read| n_scope_0_sum_232
  n_scope_0_rest_181 -->|read| n_scope_0_sum_232
  n_scope_0_spread_206 -->|read| n_scope_0_sum_232
  classDef unused stroke-dasharray: 5 5;
  class n_scope_0_sum_232 unused;
```
