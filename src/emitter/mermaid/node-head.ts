import { IMPORT_KIND, VARIABLE_DECLARATION_KIND } from "../../constants.js";
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
    case "FunctionName":
      return `${name}()`;
    case "ClassName":
      return `class ${name}`;
    case "ImportBinding": {
      const isRenamedNamed =
        n.importKind === IMPORT_KIND.Named &&
        n.importedName !== null &&
        n.importedName !== undefined &&
        n.importedName !== n.name;
      return isRenamedNamed ? name : `import ${name}`;
    }
    case "CatchClause":
      return `catch ${name}`;
    case "ImplicitGlobalVariable":
      return `global ${name}`;
    case "WriteOp":
      return n.declarationKind === VARIABLE_DECLARATION_KIND.Let
        ? `let ${name}`
        : name;
    case "ModuleSource":
      return `module ${name}`;
    case "ImportIntermediate":
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
