import {
  brand,
  minLength,
  parse,
  pipe,
  string,
  type InferOutput,
} from "valibot";

export const filledString$ = pipe(
  string(),
  minLength(1, "string must be non-empty"),
  brand("FilledString"),
);

export type FilledString = InferOutput<typeof filledString$>;

export function asFilledString(value: string): FilledString {
  return parse(filledString$, value);
}
