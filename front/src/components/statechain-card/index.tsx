import { StateChainDto } from "@/dtos/statechain.dto";
import {
  Text,
  Image,
  VStack,
  Flex,
  HStack,
  Progress,
  Box,
} from "@chakra-ui/react";
import { useState } from "react";
import moment from "moment";

interface StateChainCardProps {
  // val: StateChainDto;
  val: any;
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
      <Box w="100%" flex="2">
        <Flex w="full" alignItems="center" justify="space-between">
          <VStack alignItems="flex-start" spacing="8px">
            <Text fontSize="16px" fontWeight="800">
              Id: {val.id}
            </Text>
            <Text isTruncated maxW={"160px"} fontSize={"16"} fontWeight={"800"}>
              Address: {JSON.stringify(val)}
            </Text>
            <Text isTruncated maxW={"160px"} fontSize={"16"} fontWeight={"400"}>
              Txid: {val.txid}
            </Text>
            <Text>{val.amount} Sats</Text>
          </VStack>
          <VStack alignItems="end" spacing="8px" flex="1">
            <Progress value={timeValue} size="xs" colorScheme="pink" w="50%" />
            <Text>
              Created At: {moment(val.created_at).format("HH:mm DD:MM:YYYY")}
            </Text>
            <Text>
              Updated At: {moment(val.updated_at).format("HH:mm DD:MM:YYYY")}
            </Text>
          </VStack>
        </Flex>
        <Box maxW="800px">{JSON.stringify(val)}</Box>
      </Box>
    </HStack>
  );
}
