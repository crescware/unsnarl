import {
  AST_TYPE,
  IMPORT_KIND,
  NODE_KIND,
  VARIABLE_DECLARATION_KIND,
} from "../../constants.js";
import type { VisualNode } from "../../visual-graph/model.js";
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
        n.importKind === IMPORT_KIND.Named &&
        n.importedName !== null &&
        n.importedName !== undefined &&
        n.importedName !== n.name;
      return isRenamedNamed ? name : `import ${name}`;
    }
    case AST_TYPE.CatchClause:
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
    default:
      if (n.initIsFunction) {
        return `${name}()`;
      }
      if (n.declarationKind === VARIABLE_DECLARATION_KIND.Let) {
        return `let ${name}`;
      }
      return name;
  }
}
