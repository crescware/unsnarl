import { literal } from "valibot";

export const identifier$ = literal("identifier");
export const member$ = literal("member");
export const call$ = literal("call");
export const new$ = literal("new");
export const await$ = literal("await");
export const assign$ = literal("assign");
export const update$ = literal("update");
// Marker for an operand whose AST shape isn't reducible to the head
// vocabulary (literal, computed member, arrow, template literal, etc.).
// Rendered as "..." so the surrounding structure still reads as an
// assignment / update without dragging arbitrary source slices into
// the diagram.
export const elided$ = literal("elided");
export const raw$ = literal("raw");
