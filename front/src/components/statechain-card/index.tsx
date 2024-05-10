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
  const { onCopy, value, setValue, hasCopied } = useClipboard(
    val.statechain_id
  );
  const [timeValue, setTimeValue] = useState(50);
  return (
    <HStack
      key={key}
      color="white"
      w="100%"
      textAlign="start"
      bg="#3a3a3a"
      p="32px 32px"
      borderRadius="8px"
      dir="row"
      alignItems={"center"}
      spacing="16px"
      fontSize={"l"}
    >
      <Image
        borderRadius="full"
        boxSize="50px"
        src="https://i.ibb.co/R91rN3Q/statechain.png"
      />
      <Flex w="full" alignItems={"center"} justifyContent={"center"}>
        <VStack alignItems={"flex-start"} spacing="8px">
          {/* <Text isTruncated maxW={"160px"} fontSize={"16"} fontWeight={"800"}>
            Address : {val.aggregated_address}
          </Text> */}
          {/* <Text isTruncated maxW={"160px"}  fontWeight={"400"}>
            Id : {val.statechain_id}
          </Text> */}
          <Button
            onClick={onCopy}
            bgColor={"gray"}
            rightIcon={hasCopied ? <FiCheck /> : <FiCopy />}
            borderRadius={"8"}
            p=" 4px"
            maxW={"200px"}
          >
            <Text isTruncated maxW={"200px"}>
              TxId: {val.statechain_id}
            </Text>
          </Button>
          <Text>{val.amount} Sats</Text>
        </VStack>

        <VStack alignItems={"end"} spacing={"8px"} w="100%">
          <Progress value={timeValue} size="xs" colorScheme="pink" w="50%" />

          <Text>Time to live: {val.n_lock_time}</Text>
        </VStack>
      </Flex>
    </HStack>
  );
}
