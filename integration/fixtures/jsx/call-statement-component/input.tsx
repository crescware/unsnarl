import { wrap } from "./hoc";
import { Box, Text } from "./ui";

wrap((title: string, body: string) => {
  return (
    <Box label={title}>
      <Text>{body}</Text>
    </Box>
  );
});
