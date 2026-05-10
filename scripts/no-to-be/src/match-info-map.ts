type MatchInfo = Readonly<{ label: string; message: string }>;

export const matchInfoMap: ReadonlyMap<string, MatchInfo> = new Map([
  [
    ".toBeDefined(",
    {
      label: ".toBeDefined()",
      message: "Use expect(exists(v)).toEqual(true) instead of .toBeDefined()",
    },
  ],
  [
    ".toBeNull(",
    {
      label: ".toBeNull()",
      message: "Use .toEqual(null) instead of .toBeNull()",
    },
  ],
  [
    ".toBeInstanceOf(",
    {
      label: ".toBeInstanceOf()",
      message:
        "Use expect(v instanceof T).toEqual(true) instead of .toBeInstanceOf(T)",
    },
  ],
  [
    ".toBeTruthy(",
    {
      label: ".toBeTruthy()",
      message: "Use .toEqual(true) instead of .toBeTruthy()",
    },
  ],
  [
    ".toBeFalsy(",
    {
      label: ".toBeFalsy()",
      message: "Use .toEqual(false) instead of .toBeFalsy()",
    },
  ],
  [
    ".toBe(",
    {
      label: ".toBe()",
      message: "Use .toEqual() instead of .toBe()",
    },
  ],
]);
