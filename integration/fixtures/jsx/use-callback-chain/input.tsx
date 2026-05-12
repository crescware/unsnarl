import { useCallback } from "react";

const Comp = () => {
  const a = useCallback(() => 1, []);
  const b = useCallback(() => a() + 1, [a]);
  const c = useCallback(() => b() + 1, [b]);
  return <button onClick={c}>{c()}</button>;
};
