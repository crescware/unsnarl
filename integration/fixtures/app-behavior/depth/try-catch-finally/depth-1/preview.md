# integration/fixtures/app-behavior/depth/try-catch-finally/input.ts

## Input

```ts
try {
  try {
    try {
      try {
        try {
          try {
            const x = 1;
            console.log(x);
          } catch {
            const e6 = "e6";
            console.log(e6);
          }
        } catch {
          const e5 = "e5";
          console.log(e5);
        }
      } catch {
        const e4 = "e4";
        console.log(e4);
      }
    } catch {
      const e3 = "e3";
      console.log(e3);
    }
  } catch {
    const e2 = "e2";
    console.log(e2);
  }
} catch {
  const e1 = "e1";
  console.log(e1);
}
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_console_103["global console"]
  subgraph s_scope_1["try L1-29"]
    direction RL
    subgraph s_scope_11["catch L25-28"]
      direction RL
      n_scope_11_e2_453["e2<br/>L26"]
      expr_stmt_468["console.log()<br/>L27"]
    end
  end
  subgraph s_scope_12["catch L29-32"]
    direction RL
    n_scope_12_e1_507["e1<br/>L30"]
    expr_stmt_520["console.log()<br/>L31"]
  end
  n_scope_0_console_103 -->|read| s_scope_1
  n_scope_0_console_103 -->|read| expr_stmt_468
  n_scope_11_e2_453 -->|read| expr_stmt_468
  n_scope_0_console_103 -->|read| expr_stmt_520
  n_scope_12_e1_507 -->|read| expr_stmt_520
```
