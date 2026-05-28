import { memo } from "react";
import { Box, Text } from "ui";

export const Panel = memo((title: string, body: string) => {
  return (
    <Box label={title}>
      <Text>{body}</Text>
    </Box>
  );
});
