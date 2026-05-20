const Comp = () => {
  const a = () => 1;
  const b = () => a() + 1;
  const c = () => b() + 1;
  return <button onClick={c}>{c()}</button>;
};
