import { useEffect } from "react";

const Counter = ({ start, step }: { start: number; step: number }) => {
  useEffect(() => {
    const next = start + step;
    console.log(next);
  }, [start, step]);
  return <button>{start}</button>;
};
