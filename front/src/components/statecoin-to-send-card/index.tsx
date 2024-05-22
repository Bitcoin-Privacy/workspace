import { AppApi } from "@/apis";
import { UtxoDto } from "@/dtos";
import { StateCoinDto } from "@/dtos/statechain.dto";
import { b64EncodeUnicode } from "@/utils";
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
  Badge,
} from "@chakra-ui/react";
import { useClipboard } from "@chakra-ui/react";
import Link from "next/link";
import { NextRouter, useRouter } from "next/router";
import { useEffect, useState } from "react";
import { FaCheckCircle, FaClock } from "react-icons/fa";
import { FiCheck, FiCopy } from "react-icons/fi";
import { IoIosArrowForward } from "react-icons/io";

interface StateChainCardProps {
  val: StateCoinDto;
  key: number;
  deriv: string;
}

export function StatecoinToSendCard(props: StateChainCardProps) {
  const { val, key, deriv } = props;
  const router = useRouter();
  const { onCopy, value, setValue, hasCopied } = useClipboard(
    val.statechain_id
  );

  return (
    <Flex
      key={key}
      color="white"
      w="100%"
      bg={"gray.900"}
      justifyContent="space-between"
      borderRadius="8px"
      alignItems="center"
      p="16px 16px"
    >
      <Image
        borderRadius="full"
        boxSize={{ base: "10%", md: "14%" }} // Responsive box size
        src="https://i.ibb.co/R91rN3Q/statechain.png"
      />

      <VStack spacing="8px" alignItems="end" maxW="80%">
        <Button
          onClick={onCopy}
          colorScheme="gray"
          rightIcon={hasCopied ? <FiCheck /> : <FiCopy />}
          borderRadius="8"
          p="4px 10px"
          fontSize="large"
          maxW="80%" // Ensure max width is 100%
          textAlign={{ base: "center", md: "left" }} // Center text on small screens
        >
          <Text isTruncated>TxId: {val.statechain_id}</Text>
        </Button>
        <Badge
          borderRadius="8"
          colorScheme="green"
          p="4px 10px"
          fontSize={"medium"}
          isTruncated
          maxW={"150px"}
        >
          {val.amount} SAT
        </Badge>
      </VStack>
    </Flex>
  );
}
