# integration/fixtures/app-behavior/depth/switch/input.ts

## Input

```ts
const x = 1;

switch (x) {
  case 1:
    switch (x) {
      case 1:
        switch (x) {
          case 1:
            switch (x) {
              case 1:
                switch (x) {
                  case 1:
                    switch (x) {
                      case 1:
                        console.log(x);
                        break;
                    }
                    break;
                }
                break;
            }
            break;
        }
        break;
    }
    break;
}
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_x_6["x<br/>L1"]
  n_scope_0_console_296["global console"]
  subgraph s_scope_1["switch L3-27"]
    direction RL
    switch_discriminant_scope_0_14{"switch ()<br/>L3"}
    subgraph s_scope_2["case 1 L4-26"]
      direction RL
      elk_empty_s_scope_2[" "]
    end
  end
  n_scope_0_x_6 -->|read| switch_discriminant_scope_0_14
  n_scope_0_x_6 -->|read| s_scope_2
  n_scope_0_console_296 -->|read| s_scope_2
  classDef elkEmptyPlaceholder fill:transparent,stroke:transparent,color:transparent;
  class elk_empty_s_scope_2 elkEmptyPlaceholder;
```
