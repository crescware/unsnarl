# destructuring

## Input (`input.ts`)

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
flowchart LR
  n_scope_0_source_6["source : Variable\nL1"]
  n_scope_0_list_61["list : Variable\nL2"]
  n_scope_0_a_95["a : Variable\nL4"]
  n_scope_0_renamed_101["renamed : Variable\nL4"]
  n_scope_0_d_139["d : Variable\nL5"]
  n_scope_0_first_162["first : Variable\nL6"]
  n_scope_0_third_171["third : Variable\nL6"]
  n_scope_0_rest_181["rest : Variable\nL6"]
  n_scope_0_spread_206["spread : Variable\nL7"]
  n_scope_0_sum_232["sum : Variable\nL9"]
  n_scope_0_Object_286["(unresolved:Object)"]
  module_root -->|read| n_scope_0_source_6
  module_root -->|read| n_scope_0_source_6
  module_root -->|read| n_scope_0_list_61
  module_root -->|read| n_scope_0_source_6
  module_root -->|read| n_scope_0_a_95
  module_root -->|read| n_scope_0_renamed_101
  module_root -->|read| n_scope_0_d_139
  module_root -->|read| n_scope_0_first_162
  module_root -->|read| n_scope_0_third_171
  module_root -->|read| n_scope_0_rest_181
  module_root -->|read| n_scope_0_Object_286
  module_root -->|read| n_scope_0_spread_206
  module_root["(module)"]
  classDef unused fill:#fdd,stroke:#c00;
  class n_scope_0_sum_232 unused;
```
