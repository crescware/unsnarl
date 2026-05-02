import { DEFAULT_GENERATIONS } from "./default-generations.js";

// grep -A/-B semantics: an explicit -A says "I asked for descendants only,"
// so the unspecified side falls to 0 instead of the symmetric DEFAULT.
// -C still fills in for whichever side is unspecified. The symmetric DEFAULT
// only applies when the user gave no radius flag at all.
export function resolveGenerations(args: {
  descendants: number | null;
  ancestors: number | null;
  context: number | null;
}): { descendants: number; ancestors: number } {
  const noFlag =
    args.descendants === null &&
    args.ancestors === null &&
    args.context === null;
  const fallback = noFlag ? DEFAULT_GENERATIONS : (args.context ?? 0);
  return {
    descendants: args.descendants ?? fallback,
    ancestors: args.ancestors ?? fallback,
  };
}
