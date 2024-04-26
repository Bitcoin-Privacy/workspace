import { UtxoDto } from "@/dtos";
import { StateCoinDto } from "@/dtos/statechain.dto";
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
  val: StateCoinDto;
  key: number;
}

export function StateChainCard(props: StateChainCardProps) {
  const { val, key } = props;
  const { onCopy, value, setValue, hasCopied } = useClipboard(val.funding_txid);
  const [timeValue, setTimeValue] = useState(50);
  return (
    <HStack
      key={key}
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
            Address : {val.aggregated_address}
          </Text>
          <Text isTruncated maxW={"160px"} fontSize={"16"} fontWeight={"400"}>
            Txid : {val.funding_txid}
          </Text>
          <Text>{val.amount} Sats</Text>
        </VStack>

        <Spacer />

        <VStack alignItems={"end"} spacing={"8px"} w="100%">
          <Progress value={timeValue} size="xs" colorScheme="pink" w="50%" />

          <Text>Time to live: {val.n_lock_time}</Text>
        </VStack>
      </Flex>
    </HStack>
  );
}
