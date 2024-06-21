import { StateCoinTransferDto } from "@/dtos";
import { useProfilePage } from "@/hooks";
import { Button, Flex, HStack, Image, Text, VStack } from "@chakra-ui/react";
import { FC } from "react";

interface IStateChainTransferCard {
  key: number;
  deriv: string;
  data: StateCoinTransferDto;
}

export const StateChainTransferCard: FC<IStateChainTransferCard> = (props) => {
  const { key, deriv, data } = props;
  const {
    methods: { onVerifyTransferStatecoinClick },
  } = useProfilePage();

  return (
    <HStack
      key={key}
      color="white"
      textAlign="start"
      w="90%"
      maxW="90%"
      bg="#3a3a3a"
      p="8px 16px"
      borderRadius="8px"
      dir="row"
      alignItems={"center"}
      spacing="8px"
    >
      <Image
        borderRadius="full"
        boxSize="50px"
        src="https://i.ibb.co/R91rN3Q/statechain.png"
      />
      <Flex w="full" alignItems="center" justify="space-between">
        <VStack alignItems={"flex-start"} spacing="8px">
          <Text isTruncated maxW={"160px"} fontSize={"16"} fontWeight={"400"}>
            Authkey : {data.auth_key}
          </Text>
        </VStack>

        <VStack alignItems={"end"} spacing={"8px"} w="100%">
          <Button
            onClick={() =>
              onVerifyTransferStatecoinClick(
                deriv,
                data.transfer_message,
                data.auth_key,
              )
            }
          >
            Verify
          </Button>
        </VStack>
      </Flex>
    </HStack>
  );
};
