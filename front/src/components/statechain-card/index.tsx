import { UtxoDto } from "@/dtos";
import { StateChainDto } from "@/dtos/statechain.dto";
import {
  Box,
  Text,
  Image,
  VStack,
  Flex,
  HStack,
  Spacer,
  Button,
  Progress,
} from "@chakra-ui/react";
import { useClipboard } from "@chakra-ui/react";
import { useState } from "react";
import { FiCheck, FiCopy } from "react-icons/fi";

interface StateChainCardProps {
  val: StateChainDto;
  index: number;
}

export function StateChainCard(props: StateChainCardProps) {
  const { val, index } = props;
  const { onCopy, value, setValue, hasCopied } = useClipboard(val.txid);
  const [timeValue, setTimeValue] = useState(50);
  return (
    <HStack
      key={index}
      color="white"
      textAlign="start"
      w="80%"
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
      <Flex w="full" alignItems={"center"}>
        <VStack alignItems={"flex-start"} spacing="8px">
          <Text isTruncated maxW={"160px"} fontSize={"16"} fontWeight={"800"}>
            Address : {val.address}
          </Text>
          <Text isTruncated maxW={"160px"} fontSize={"16"} fontWeight={"400"}>
            Txid : {val.txid}
          </Text>
          <Text>{val.value} Sats</Text>
        </VStack>

        <Spacer />

        <VStack alignItems={"end"} spacing={"8px"} w="100%">
          <Progress value={timeValue} size="xs" colorScheme="pink" w="50%" />

          <Text>Time to live: {val.n_locktime}</Text>
        </VStack>
      </Flex>
    </HStack>
  );
}
