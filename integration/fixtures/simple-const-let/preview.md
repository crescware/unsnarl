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
flowchart RL
  n_scope_0_greeting_6["greeting : Variable<br/>L1"]
  n_scope_0_counter_30["counter : Variable<br/>L2"]
  n_scope_0_message_97["message : Variable<br/>L6"]
  n_scope_0_unused_123["unused : Variable<br/>L7"]
  n_scope_0_counter_30 -->|write| n_scope_0_counter_30
  n_scope_0_counter_30 -->|read| n_scope_0_counter_30
  n_scope_0_counter_30 -->|read,write| n_scope_0_counter_30
  n_scope_0_counter_30 -->|read,write| module_root
  n_scope_0_greeting_6 -->|read| n_scope_0_message_97
  module_root((module))
  classDef unused stroke-dasharray: 5 5;
  class n_scope_0_message_97 unused;
  class n_scope_0_unused_123 unused;
```
