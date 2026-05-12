import { useMemo } from "react";

const Counter = ({ start }: { start: number }) => {
  const value = useMemo(() => {
    const doubled = start * 2;
    return doubled;
  }, [start]);
  return <button>{value}</button>;
};
