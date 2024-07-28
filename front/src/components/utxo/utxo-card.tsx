import { UtxoDto } from "@/dtos";
import {
  Text,
  Image,
  VStack,
  Flex,
  Button,
  Badge,
  Box,
  HStack,
} from "@chakra-ui/react";
import { useClipboard } from "@chakra-ui/react";
import { FaCheckCircle, FaClock } from "react-icons/fa";
import { FiCheck, FiCopy } from "react-icons/fi";

interface IUtxoCard {
  data: UtxoDto;
  key: number;
}

export function UtxoCard(props: IUtxoCard) {
  const { data: val, key } = props;
  const { onCopy, hasCopied } = useClipboard(val.txid);

  return (
    <HStack
      key={key}
      w="100%"
      bg="gray.900"
      borderRadius="8px"
      p="16px"
      justify="space-between"
      alignItems="center"
    >
      <Image borderRadius="full" boxSize="54px" src="/bitcoin-icon.jpg" />
      <HStack justify="space-between" ml="16px" alignItems="center" w="100%">
        <VStack spacing="8px" alignItems="start">
          <Badge
            borderRadius="8"
            colorScheme="yellow"
            p="4px 10px"
            fontSize="medium"
          >
            BitCoin
          </Badge>
          <Button
            onClick={onCopy}
            bg="cyan.200"
            variant="solid"
            rightIcon={hasCopied ? <FiCheck /> : <FiCopy />}
            borderRadius="8px"
            textAlign="left"
          >
            <Text
              fontSize="16"
              textOverflow="ellipsis"
              wordBreak="break-all"
              overflow="hidden"
              maxW={{ base: "100px", sm: "200px", md: "380px" }}
            >
              TxId: {val.txid}
            </Text>
          </Button>
        </VStack>
        <Box h="full" textAlign="right">
          <Box fontSize="medium" fontWeight="700">
            {val.value} Sats
          </Box>
          <Text> Vout: {val.vout}</Text>
          <Flex align="center" gap="10px">
            Confirm:
            {val.status.confirmed ? (
              <FaCheckCircle color="#41c300" />
            ) : (
              <FaClock color="#fa8100" />
            )}
          </Flex>
        </Box>
      </HStack>
    </HStack>
  );
}
