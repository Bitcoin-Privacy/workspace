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
    p={{ base: "16px", md: "32px" }} // Adjust padding based on screen size
    borderRadius="8px"
    alignItems="center"
    spacing="16px"
    fontSize="md" // Adjust font size based on screen size
    flexDirection={{ base: "column", md: "row" }} // Change direction based on screen size
  >
    <Image
      borderRadius="full"
      boxSize="50px"
      src="https://i.ibb.co/R91rN3Q/statechain.png"
      mb={{ base: "16px", md: 0 }} // Adjust margin bottom based on screen size
    />
    <Flex w="full" alignItems="center" justifyContent="space-between">
      <VStack alignItems="flex-start" spacing="8px" mr={{ base: 0, md: "16px" }}>
        <Button
          onClick={onCopy}
          bgColor="gray"
          rightIcon={hasCopied ? <FiCheck /> : <FiCopy />}
          borderRadius="8"
          p="4px"
          maxW="100%" // Adjust max width to occupy full width on small screens
          textAlign={{ base: "center", md: "left" }} // Center text on small screens
        >
          <Text isTruncated maxW="200px">
            TxId: {val.statechain_id}
          </Text>
        </Button>
        <Text>{val.amount} Sats</Text>
      </VStack>

      <VStack alignItems="flex-end" spacing="8px" w={{ base: "100%", md: "50%" }}>
        <Progress value={timeValue} size="xs" colorScheme="pink" w="100%" />
        <Text>Time to live: {val.n_lock_time}</Text>
      </VStack>
    </Flex>
  </HStack>
  );
}
