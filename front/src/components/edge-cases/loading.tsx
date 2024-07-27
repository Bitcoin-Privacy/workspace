import React, { memo } from "react";
import { Text, VStack, Spinner } from "@chakra-ui/react";

interface ILoading {
  content: string;
}

export const Loading = memo(function Loading(props: ILoading) {
  const { content } = props;
  return (
    <VStack textAlign="center" p="0px 16px" spacing="20px">
      <Spinner />
      <Text color="white" fontWeight="700" fontSize="18px">
        {content}
      </Text>
    </VStack>
  );
});
