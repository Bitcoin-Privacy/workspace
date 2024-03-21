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
} from "@chakra-ui/react";
import { useClipboard } from "@chakra-ui/react";
import { FiCheck, FiCopy } from "react-icons/fi";

interface StateChainCardProps {
  val: StateChainDto;
  index: number;
}

export function StateChainCard(props: StateChainCardProps) {
  const { val, index } = props;
  const { onCopy, value, setValue, hasCopied } = useClipboard(val.txid);

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
    >
      <Image
        borderRadius="full"
        boxSize="50px"
        src="https://i.ibb.co/M6xxyd6/istockphoto-905413264-612x612.jpg"
      />
      <Flex w="full" alignItems={"center"}>
        <VStack>
          <Text fontSize={"20"} fontWeight={"1000"}>
            Bitcoin
          </Text>
          <Text>{val.value} Sats</Text>
        </VStack>

        <Spacer />

        <VStack alignItems={"end"}>
          <Button
            onClick={onCopy}
            bgColor={"cyan.100"}
            rightIcon={hasCopied ? <FiCheck /> : <FiCopy />}
            borderRadius={"8"}
            p="2px 8px"
          >
            <Text isTruncated maxW={"120px"}>
              TxId: {val.txid}
            </Text>
          </Button>
          <Text>vout: {val.n_locktime}</Text>
        </VStack>
      </Flex>
    </HStack>
  );
}
