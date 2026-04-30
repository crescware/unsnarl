# integration/fixtures/destructuring/input.ts

## Input

```ts
const source = { a: 1, b: 2, c: 3, nested: { d: 4 } };
const list = [10, 20, 30, 40];

const { a, b: renamed } = source;
const {
  nested: { d },
} = source;
const [first, , third, ...rest] = list;
const { ...spread } = source;

const sum =
  a + renamed + d + first + third + rest.length + Object.keys(spread).length;
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_source_6["source<br/>L1"]
  n_scope_0_list_61["list<br/>L2"]
  n_scope_0_a_95["a<br/>L4"]
  n_scope_0_renamed_101["renamed<br/>L4"]
  n_scope_0_d_141["d<br/>L6"]
  n_scope_0_first_165["first<br/>L8"]
  n_scope_0_third_174["third<br/>L8"]
  n_scope_0_rest_184["rest<br/>L8"]
  n_scope_0_spread_209["spread<br/>L9"]
  n_scope_0_sum_235["unused sum<br/>L11"]
  n_scope_0_source_6 -->|read| n_scope_0_a_95
  n_scope_0_source_6 -->|read| n_scope_0_renamed_101
  n_scope_0_source_6 -->|read| n_scope_0_d_141
  n_scope_0_list_61 -->|read| n_scope_0_first_165
  n_scope_0_list_61 -->|read| n_scope_0_third_174
  n_scope_0_list_61 -->|read| n_scope_0_rest_184
  n_scope_0_source_6 -->|read| n_scope_0_spread_209
  n_scope_0_a_95 -->|read| n_scope_0_sum_235
  n_scope_0_renamed_101 -->|read| n_scope_0_sum_235
  n_scope_0_d_141 -->|read| n_scope_0_sum_235
  n_scope_0_first_165 -->|read| n_scope_0_sum_235
  n_scope_0_third_174 -->|read| n_scope_0_sum_235
  n_scope_0_rest_184 -->|read| n_scope_0_sum_235
  n_scope_0_spread_209 -->|read| n_scope_0_sum_235
```
