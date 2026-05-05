# integration/fixtures/for-statement/for-of/object-destructuring/input.ts

## Input

```ts
const items = [
  { id: 1, label: "x" },
  { id: 2, label: "y" },
];
for (const { id, label } of items) {
  console.log(id, label);
}
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_items_6["items<br/>L1"]
  n_scope_0_console_108["global console"]
  subgraph s_scope_1["for L5-7"]
    direction RL
    for_test_scope_0_69["for ()<br/>L5"]
    n_scope_1_id_82["id<br/>L5"]
    n_scope_1_label_86["label<br/>L5"]
    expr_stmt_108["console.log()<br/>L6"]
  end
  n_scope_0_items_6 -->|read| for_test_scope_0_69
  n_scope_0_console_108 -->|read| expr_stmt_108
  n_scope_1_id_82 -->|read| expr_stmt_108
  n_scope_1_label_86 -->|read| expr_stmt_108
```
