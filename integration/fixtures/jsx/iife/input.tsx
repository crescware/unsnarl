const Counter = ({ start }: { start: number }) => {
  const value = (() => {
    const doubled = start * 2;
    return doubled;
  })();
  return <button>{value}</button>;
};
