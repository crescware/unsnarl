# imports

## Input (`input.ts`)

```ts
import def from "some-default";
import { named, other as renamed } from "some-named";
import * as ns from "some-namespace";

const a = def;
const b = named;
const c = renamed;
const d = ns;
```

## Mermaid

```mermaid
flowchart RL
  n_scope_0_def_7["def : ImportBinding\nL1"]
  n_scope_0_named_41["named : ImportBinding\nL2"]
  n_scope_0_renamed_57["renamed : ImportBinding\nL2"]
  n_scope_0_ns_98["ns : ImportBinding\nL3"]
  n_scope_0_a_131["a : Variable\nL5"]
  n_scope_0_b_146["b : Variable\nL6"]
  n_scope_0_c_163["c : Variable\nL7"]
  n_scope_0_d_182["d : Variable\nL8"]
  n_scope_0_def_7 -->|read| n_scope_0_a_131
  n_scope_0_named_41 -->|read| n_scope_0_b_146
  n_scope_0_renamed_57 -->|read| n_scope_0_c_163
  n_scope_0_ns_98 -->|read| n_scope_0_d_182
  classDef unused stroke-dasharray: 5 5;
  class n_scope_0_a_131 unused;
  class n_scope_0_b_146 unused;
  class n_scope_0_c_163 unused;
  class n_scope_0_d_182 unused;
```
