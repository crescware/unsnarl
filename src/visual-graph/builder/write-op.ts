export type WriteOp = Readonly<{
  refId: string;
  varId: string;
  varName: string;
  line: number;
  offset: number;
  scopeId: string;
}>;
