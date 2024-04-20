import { StateChainDto } from "@/dtos/statechain.dto";
import { Text, Image, VStack, Flex, HStack, Progress } from "@chakra-ui/react";
import { useState } from "react";

interface StateChainCardProps {
  val: StateChainDto;
  key: number;
}

export function StateChainCard(props: StateChainCardProps) {
  const { val, key } = props;
  const [timeValue] = useState(50);
  return (
    <HStack
      key={key}
      color="white"
      textAlign="start"
      w="100%"
      bg="#3a3a3a"
      p="8px 16px"
      borderRadius="8px"
      dir="row"
      alignItems={"center"}
      spacing="8px"
    >
      <Image
        alt=""
        borderRadius="full"
        boxSize="50px"
        src="https://i.ibb.co/R91rN3Q/statechain.png"
      />
      <Flex w="full" alignItems="center" justify="space-between">
        <VStack alignItems={"flex-start"} spacing="8px">
          <Text isTruncated maxW={"160px"} fontSize={"16"} fontWeight={"800"}>
            Address : {val.address}
          </Text>
          <Text isTruncated maxW={"160px"} fontSize={"16"} fontWeight={"400"}>
            Txid : {val.txid}
          </Text>
          <Text>{val.value} Sats</Text>
        </VStack>
        <VStack alignItems={"end"} spacing={"8px"} w="100%">
          <Progress value={timeValue} size="xs" colorScheme="pink" w="50%" />
          <Text>Time to live: {val.n_locktime}</Text>
        </VStack>
      </Flex>
    </HStack>
  );
}
