const records: { meta: { tag?: string } }[] = [
  { meta: {} },
  { meta: { tag: "T" } },
];
for (const {
  meta: { tag = "default" },
} of records) {
  console.log(tag);
}
