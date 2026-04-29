# simple-const-let

## Input (`input.ts`)

```ts
const greeting = "hello";
let counter = 0;
counter = counter + 1;
counter += 2;
counter++;
const message = greeting;
const unused = 99;
```

## Mermaid

```mermaid
flowchart LR
  n_scope_0_greeting_6["greeting : Variable\nL1"]
  n_scope_0_counter_30["counter : Variable\nL2"]
  n_scope_0_message_97["message : Variable\nL6"]
  n_scope_0_unused_123["unused : Variable\nL7"]
  module_root -->|write| n_scope_0_counter_30
  module_root -->|read| n_scope_0_counter_30
  module_root -->|read,write| n_scope_0_counter_30
  module_root -->|read,write| n_scope_0_counter_30
  module_root -->|read| n_scope_0_greeting_6
  module_root["(module)"]
  classDef unused fill:#fdd,stroke:#c00;
  class n_scope_0_message_97 unused;
  class n_scope_0_unused_123 unused;
```
