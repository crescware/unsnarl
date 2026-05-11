export async function run(): Promise<void> {
  await Promise.resolve()
    .then((value) => {
      console.log("then handler", value);
      console.log("then handler second line");
    })
    .catch((error) => {
      console.error("catch handler", error);
      console.error("catch handler second line");
    });
}
