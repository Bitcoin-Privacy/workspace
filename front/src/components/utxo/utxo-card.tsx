import { UtxoDto } from "@/dtos";
import {
  Text,
  Image,
  VStack,
  Flex,
  Button,
  Badge,
  Box,
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
    <Flex
      key={key}
      w="100%"
      bg={"gray.900"}
      borderRadius="8px"
      px="16px"
      py="16px"
      justifyContent={"space-between"}
      direction={"column"}
      wrap={"wrap"}
      alignItems={"center"}
    >
      <Flex justifyContent={"space-between"}>
        <Image
          borderRadius="full"
          boxSize="8%"
          src="https://i.ibb.co/M6xxyd6/istockphoto-905413264-612x612.jpg"
        />
        <Flex
          w={"100%"}
          justifyContent={"space-between"}
          ml={"16px"}
          alignItems={"center"}
        >
          <VStack spacing="8px" alignItems="start">
            <Badge
              borderRadius="8"
              colorScheme="yellow"
              p="4px 10px"
              isTruncated
              maxW={"150px"}
              fontSize={"larger"}
            >
              Bitcoin
            </Badge>
            <Button
              onClick={onCopy}
              bg={"cyan.200"}
              variant="solid"
              rightIcon={hasCopied ? <FiCheck /> : <FiCopy />}
              borderRadius="8px"
              maxW={{ base: "40%", md: "70%" }}
              textAlign={"left"}
            >
              <Text fontSize="16" isTruncated>
                TxId: {val.txid}
              </Text>
            </Button>
          </VStack>
          <Flex
            h="full"
            direction={"column"}
            justifyContent={"space-around"}
            alignItems={"end"}
          >
            <Box fontSize={"x-large"} fontWeight={"800"}>
              {" "}
              {val.value} SAT
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
          </Flex>
        </Flex>
      </Flex>
    </Flex>
  );
}
