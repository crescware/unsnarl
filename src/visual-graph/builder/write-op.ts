export interface WriteOp {
  refId: string;
  varId: string;
  varName: string;
  line: number;
  offset: number;
  scopeId: string;
}
