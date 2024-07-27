import React, { memo } from "react";
import { Text, VStack } from "@chakra-ui/react";

interface IError {
  content: string;
}

export const Error = memo(function Error(props: IError) {
  const { content } = props;
  return (
    <VStack textAlign="center" p="0px 16px" spacing="20px">
      <Text color="white" fontWeight="700" fontSize="18px">
        {content}
      </Text>
    </VStack>
  );
});
