import type { DEFINITION_TYPE } from "../../analyzer/definition-type.js";
import type { IMPORT_KIND } from "../../serializer/import-kind.js";
import type { VariableDeclarationKind } from "../../serializer/variable-declaration-kind.js";
import type { Span } from "../primitive/span.js";

type CommonDefFields = Readonly<{
  name: Readonly<{ name: string; span: Span }>;
  node: Readonly<{ type: string; span: Span }>;
  parent: Readonly<{ type: string; span: Span }> | null;
}>;

export type SerializedDefinition =
  | (CommonDefFields &
      Readonly<{
        type: typeof DEFINITION_TYPE.Variable;
        init: Readonly<{ type: string; span: Span }> | null;
        declarationKind: VariableDeclarationKind | null;
      }>)
  | (CommonDefFields &
      Readonly<{
        type: typeof DEFINITION_TYPE.ImportBinding;
        importKind: typeof IMPORT_KIND.Named;
        importedName: string;
        importSource: string;
      }>)
  | (CommonDefFields &
      Readonly<{
        type: typeof DEFINITION_TYPE.ImportBinding;
        importKind: typeof IMPORT_KIND.Default;
        importSource: string;
      }>)
  | (CommonDefFields &
      Readonly<{
        type: typeof DEFINITION_TYPE.ImportBinding;
        importKind: typeof IMPORT_KIND.Namespace;
        importSource: string;
      }>)
  | (CommonDefFields & Readonly<{ type: typeof DEFINITION_TYPE.FunctionName }>)
  | (CommonDefFields & Readonly<{ type: typeof DEFINITION_TYPE.ClassName }>)
  | (CommonDefFields & Readonly<{ type: typeof DEFINITION_TYPE.Parameter }>)
  | (CommonDefFields & Readonly<{ type: typeof DEFINITION_TYPE.CatchClause }>)
  | (CommonDefFields &
      Readonly<{ type: typeof DEFINITION_TYPE.ImplicitGlobalVariable }>);
