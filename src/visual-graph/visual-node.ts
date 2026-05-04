import type { IMPORT_KIND } from "../serializer/import-kind.js";
import type { VariableDeclarationKind } from "../serializer/variable-declaration-kind.js";
import type { NODE_KIND } from "./node-kind.js";
import type { VISUAL_ELEMENT_TYPE } from "./visual-element-type.js";

// Common shape across every kind. Mutable: builder.ts and the various
// builder/* helpers may patch endLine / unused after the node is first
// inserted into its container. Wrapping in Readonly would force a
// refactor of every post-construction patch site.
type CommonNodeFields = {
  type: typeof VISUAL_ELEMENT_TYPE.Node;
  id: string;
  name: string;
  line: number;
  // Set when the reference logically extends past its identifier line --
  // currently the JSX element case where <A>...</A> spans line..endLine.
  // Renderers display L{line}-{endLine} and prune treats line queries as
  // matching anywhere within the closed range. null when single-line.
  endLine: number | null;
  // Marks a reference whose identifier names a JSX element opening tag, so
  // renderers can wrap the label as `<Name>` regardless of whether the
  // element happens to be single-line (endLine is null).
  isJsxElement: boolean;
  unused: boolean;
};

export type VisualNode =
  | (CommonNodeFields & { kind: typeof NODE_KIND.FunctionName })
  | (CommonNodeFields & { kind: typeof NODE_KIND.ClassName })
  | (CommonNodeFields & { kind: typeof NODE_KIND.Parameter })
  | (CommonNodeFields & { kind: typeof NODE_KIND.CatchClause })
  | (CommonNodeFields & { kind: typeof NODE_KIND.ImplicitGlobalVariable })
  | (CommonNodeFields & { kind: typeof NODE_KIND.ReturnUse })
  | (CommonNodeFields & { kind: typeof NODE_KIND.IfTest })
  | (CommonNodeFields & { kind: typeof NODE_KIND.ModuleSink })
  | (CommonNodeFields & { kind: typeof NODE_KIND.ModuleSource })
  | (CommonNodeFields & { kind: typeof NODE_KIND.ImportIntermediate })
  | (CommonNodeFields & { kind: typeof NODE_KIND.ExpressionStatement })
  | (CommonNodeFields & {
      kind: typeof NODE_KIND.Variable;
      declarationKind: VariableDeclarationKind | null;
      initIsFunction: boolean;
    })
  | (CommonNodeFields & {
      kind: typeof NODE_KIND.WriteOp;
      declarationKind: VariableDeclarationKind | null;
    })
  | (CommonNodeFields & {
      kind: typeof NODE_KIND.ImportBinding;
      importKind: typeof IMPORT_KIND.Named;
      importedName: string;
    })
  | (CommonNodeFields & {
      kind: typeof NODE_KIND.ImportBinding;
      importKind: typeof IMPORT_KIND.Default;
    })
  | (CommonNodeFields & {
      kind: typeof NODE_KIND.ImportBinding;
      importKind: typeof IMPORT_KIND.Namespace;
    });
