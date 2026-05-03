import { IMPORT_KIND } from "../../serializer/import-kind.js";
import { VARIABLE_DECLARATION_KIND } from "../../serializer/variable-declaration-kind.js";
import type { VisualNode } from "../../visual-graph/model.js";
import { NODE_KIND } from "../../visual-graph/node-kind.js";
import { escape } from "./escape.js";

export function nodeHead(n: VisualNode): string {
  const name = escape(n.name);
  if (n.isJsxElement) {
    // Mermaid `["..."]` labels require HTML-escaped angle brackets so the
    // parser does not mistake them for syntax; the renderer surfaces them
    // as literal `<` / `>` in the output.
    return `&lt;${name}&gt;`;
  }
  switch (n.kind) {
    case NODE_KIND.FunctionName:
      return `${name}()`;
    case NODE_KIND.ClassName:
      return `class ${name}`;
    case NODE_KIND.ImportBinding: {
      const isRenamedNamed =
        n.importKind === IMPORT_KIND.Named && n.importedName !== n.name;
      return isRenamedNamed ? name : `import ${name}`;
    }
    case NODE_KIND.CatchClause:
      return `catch ${name}`;
    case NODE_KIND.ImplicitGlobalVariable:
      return `global ${name}`;
    case NODE_KIND.WriteOp:
      return n.declarationKind === VARIABLE_DECLARATION_KIND.Let
        ? `let ${name}`
        : name;
    case NODE_KIND.ModuleSource:
      return `module ${name}`;
    case NODE_KIND.ImportIntermediate:
      return `import ${name}`;
    case NODE_KIND.Variable:
      if (n.initIsFunction) {
        return `${name}()`;
      }
      if (n.declarationKind === VARIABLE_DECLARATION_KIND.Let) {
        return `let ${name}`;
      }
      return name;
    case NODE_KIND.Parameter:
    case NODE_KIND.ReturnUse:
    case NODE_KIND.IfTest:
    case NODE_KIND.ModuleSink:
      return name;
  }
}
