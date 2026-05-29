import { wrap } from "./hoc";
import { Box, Text } from "./ui";

export const Panel = wrap((title: string, body: string) => {
  return (
    <Box label={title}>
      <Text>{body}</Text>
    </Box>
  );
});
