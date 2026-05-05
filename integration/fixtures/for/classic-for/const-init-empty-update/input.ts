// const is allowed in init but not in update (init only per spec);
// allowed when update is omitted
for (const j = 0; ; ) {
  console.log(j);
  break;
}
