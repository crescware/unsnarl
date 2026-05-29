async function main() {
  await using a = acquire();
  a.release();
}
