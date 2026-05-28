{
  const before = 1;
  console.log(before);
}
run(() => {
  console.log("callback body");
});
{
  const after = 2;
  console.log(after);
}
