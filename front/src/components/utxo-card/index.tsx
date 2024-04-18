import { UtxoDto } from "@/dtos";
import { Text, Image, VStack, Flex, HStack, Button } from "@chakra-ui/react";
import { useClipboard } from "@chakra-ui/react";
import { FaCheckCircle, FaClock } from "react-icons/fa";
import { FiCheck, FiCopy } from "react-icons/fi";

interface UTXOCardProps {
  val: UtxoDto;
  key: number;
}

export function UTXOCard(props: UTXOCardProps) {
  const { val, key } = props;
  const { onCopy, value, setValue, hasCopied } = useClipboard(val.txid);

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
    >
      <Image
        alt=""
        borderRadius="full"
        boxSize="50px"
        src="https://i.ibb.co/M6xxyd6/istockphoto-905413264-612x612.jpg"
      />
      <Flex w="full" alignItems={"center"} justify="space-between">
        <VStack>
          <Text fontSize={"20"} fontWeight={"1000"}>
            Bitcoin
          </Text>
          <Text>{val.value} Sats</Text>
        </VStack>

        <VStack alignItems={"end"}>
          <Button
            onClick={onCopy}
            bgColor={"cyan.100"}
            rightIcon={hasCopied ? <FiCheck /> : <FiCopy />}
            borderRadius={"8"}
            p="2px 8px"
          >
            <Text isTruncated maxW={"160px"}>
              TxId: {val.txid}
            </Text>
          </Button>
          <Text>vout: {val.vout}</Text>
          <Flex align="center" gap="10px">
            Confirm:
            {val.status.confirmed ? (
              <FaCheckCircle color="#41c300" />
            ) : (
              <FaClock color="#fa8100" />
            )}
          </Flex>
        </VStack>
      </Flex>
    </HStack>
  );
}
