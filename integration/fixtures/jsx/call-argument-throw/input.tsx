import { wrap } from "./hoc";
import { Box, Text } from "./ui";

export const Panel = wrap((title: string, body: string) => {
  throw (
    <Box label={title}>
      <Text>{body}</Text>
    </Box>
  );
});
