import { StateCoinDto } from "@/dtos/statechain.dto";
import { Text, Image, VStack, Flex, Badge, Avatar } from "@chakra-ui/react";
import { Copier } from "..";

interface StateChainCardProps {
  val: StateCoinDto;
  key: number;
  deriv: string;
}

export function StatecoinToSendCard(props: StateChainCardProps) {
  const { val, key } = props;

  return (
    <Flex
      key={key}
      color="white"
      w="100%"
      bg={"gray.900"}
      borderRadius="8px"
      alignItems="center"
      p="16px 28px"
      gap="28px"
    >
      <Avatar h="54px" w="54px" src="/statecoin-icon.png" />
      <VStack spacing="8px" alignItems="start" maxW="80%">
        <Copier content={val.statechain_id} />
        <Badge
          borderRadius="8"
          colorScheme="green"
          p="4px 10px"
          fontSize={"medium"}
          isTruncated
          maxW={"150px"}
        >
          {val.amount} SAT
        </Badge>
      </VStack>
    </Flex>
  );
}
