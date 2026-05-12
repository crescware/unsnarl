import { useCallback } from "react";

const Counter = ({ start }: { start: number }) => {
  const inc = useCallback((n: number) => n + start, [start]);
  return <button>{inc(1)}</button>;
};
