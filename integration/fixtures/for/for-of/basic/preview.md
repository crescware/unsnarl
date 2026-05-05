# integration/fixtures/for/for-of/basic/input.ts

## Input

```ts
for (const item of [1, 2, 3]) {
  console.log(item);
}
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_console_34["global console"]
  subgraph s_scope_1["for L1-3"]
    direction RL
    for_test_scope_0_0["for ()<br/>L1"]
    n_scope_1_item_11["item<br/>L1"]
    expr_stmt_34["console.log()<br/>L2"]
  end
  n_scope_0_console_34 -->|read| expr_stmt_34
  n_scope_1_item_11 -->|read| expr_stmt_34
```
