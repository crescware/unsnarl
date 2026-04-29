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
  n_scope_0_def_7["import def<br/>L1"]
  n_scope_0_named_41["import named<br/>L2"]
  n_scope_0_renamed_57["import renamed<br/>L2"]
  n_scope_0_ns_98["import ns<br/>L3"]
  n_scope_0_a_131["a<br/>L5"]
  n_scope_0_b_146["b<br/>L6"]
  n_scope_0_c_163["c<br/>L7"]
  n_scope_0_d_182["d<br/>L8"]
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
