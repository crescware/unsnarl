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
  n_scope_0_def_7["def<br/>L1"]
  n_scope_0_named_41["named<br/>L2"]
  n_scope_0_renamed_57["renamed<br/>L2"]
  n_scope_0_ns_98["import ns<br/>L3"]
  n_scope_0_a_131["a<br/>L5"]
  n_scope_0_b_146["b<br/>L6"]
  n_scope_0_c_163["c<br/>L7"]
  n_scope_0_d_182["d<br/>L8"]
  n_scope_0_def_7 -->|read| n_scope_0_a_131
  n_scope_0_named_41 -->|read| n_scope_0_b_146
  n_scope_0_renamed_57 -->|read| n_scope_0_c_163
  n_scope_0_ns_98 -->|read| n_scope_0_d_182
  mod_some_default["module some-default<br/>L1"]
  mod_some_named["module some-named<br/>L2"]
  mod_some_namespace["module some-namespace<br/>L3"]
  import_some_named__other["import other<br/>L2"]
  mod_some_default -->|read| n_scope_0_def_7
  mod_some_named -->|read| n_scope_0_named_41
  mod_some_named -->|read| import_some_named__other
  import_some_named__other -->|read| n_scope_0_renamed_57
  mod_some_namespace -->|read| n_scope_0_ns_98
  classDef unused stroke-dasharray: 5 5;
  class n_scope_0_a_131 unused;
  class n_scope_0_b_146 unused;
  class n_scope_0_c_163 unused;
  class n_scope_0_d_182 unused;
```
