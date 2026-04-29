# jsx-component

## Input (`input.tsx`)

```tsx
import { Fragment } from "react";

interface Props {
  label: string;
}

const Hello = (props: Props) => (
  <Fragment>
    <span className="greeting">{props.label}</span>
  </Fragment>
);

const App = () => <Hello label="hi" />;

const root = App;
```

## Mermaid

```mermaid
flowchart LR
  n_scope_0_Fragment_9["Fragment : ImportBinding\nL1"]
  n_scope_0_Hello_79["Hello : Variable\nL7"]
  n_scope_0_App_196["App : Variable\nL13"]
  n_scope_0_root_237["root : Variable\nL15"]
  n_scope_1_props_88["props : Parameter\nL7"]
  n_scope_0_span_125["(unresolved:span)"]
  module_root -->|read| n_scope_1_props_88
  module_root -->|read| n_scope_0_Fragment_9
  module_root -->|read| n_scope_0_span_125
  module_root -->|read| n_scope_1_props_88
  module_root -->|read| n_scope_0_Hello_79
  module_root -->|read| n_scope_0_App_196
  module_root["(module)"]
  classDef unused fill:#fdd,stroke:#c00;
  class n_scope_0_root_237 unused;
```
