import { useCallback } from "react";

const C = () => {
  const fnA = useCallback((arr: number[]) => arr.map((n) => n * 2), []);
  const fnB = (arr: number[]) => arr.map((n) => n + 1);
  return null;
};
