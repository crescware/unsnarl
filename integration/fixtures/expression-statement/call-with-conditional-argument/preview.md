# integration/fixtures/expression-statement/call-with-conditional-argument/input.ts

## Input

```ts
const flag = true;
const left = "on";
const right = "off";

console.log(flag ? left : right);
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_flag_6["flag<br/>L1"]
  n_scope_0_left_25["left<br/>L2"]
  n_scope_0_right_44["right<br/>L3"]
  n_scope_0_console_60["global console"]
  n_scope_0_console_60 -->|read| expr_stmt_60
  n_scope_0_flag_6 -->|read| expr_stmt_60
  n_scope_0_left_25 -->|read| expr_stmt_60
  n_scope_0_right_44 -->|read| expr_stmt_60
  expr_stmt_60["console.log()<br/>L5"]
```
