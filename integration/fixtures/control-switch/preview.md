# control-switch

## Input (`input.ts`)

```ts
let label = "";
const kind = "a";

switch (kind) {
  case "a":
    label = "alpha";
    break;
  case "b":
    label = "beta";
    break;
  default:
    label = "other";
}

const result = label;
```

## Mermaid

```mermaid
flowchart RL
  n_scope_0_label_4["label<br/>L1"]
  n_scope_0_kind_22["kind<br/>L2"]
  n_scope_0_result_179["result<br/>L15"]
  n_scope_0_kind_22 -->|read| module_root
  n_scope_0_label_4 -->|write| n_scope_0_label_4
  n_scope_0_label_4 -->|write| n_scope_0_label_4
  n_scope_0_label_4 -->|write| n_scope_0_label_4
  n_scope_0_label_4 -->|read| n_scope_0_result_179
  module_root((module))
  classDef unused stroke-dasharray: 5 5;
  class n_scope_0_result_179 unused;
```
