# integration/fixtures/expression-statement/conditional/input.ts

## Input

```ts
const enabled = true;

enabled
  ? "the value selected when the condition is true, kept long on purpose"
  : "the value selected when the condition is false, also kept long";
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_enabled_6["enabled<br/>L1"]
  n_scope_0_enabled_6 -->|read| expr_stmt_23
  expr_stmt_23["enabled
  ? &quot;the value selected when the condition is true, kept long on purpose&quot;
  : &quot;the value selected when the condition is false, also kept long&quot;<br/>L3-5"]
```
