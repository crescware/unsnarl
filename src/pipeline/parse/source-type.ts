// ECMAScript goal symbol for the parsed source. Script and Module have
// different scoping rules (e.g. module is always strict, top-level
// `import`/`export` is module-only) so callers must pick one explicitly
// rather than relying on file extension alone.
export const SOURCE_TYPE = {
  Script: "script",
  Module: "module",
} as const;
export type SourceType = (typeof SOURCE_TYPE)[keyof typeof SOURCE_TYPE];
