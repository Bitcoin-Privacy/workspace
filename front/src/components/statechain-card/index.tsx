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

export function StateChainCard(props: StateChainCardProps) {
  const { val, key, deriv } = props;
  const router = useRouter();
  const { onCopy, value, setValue, hasCopied } = useClipboard(
    val.statechain_id
  );

  const [showDetailButton, setShowDetailButton] = useState(false);
  const handleCardClick = () => {
    setShowDetailButton(!showDetailButton);
  };

  const handleDetailButtonClick = () => {
    // Navigate to detail screen with corresponding ID
    console.log(val.statechain_id);
    router.push(`${router.asPath}/statecoins/${val.statechain_id}`);
  };

  return (
    <VStack
      key={key}
      color="white"
      w="100%"
      textAlign="start"
      bg="#3a3a3a"
      paddingTop={{ base: "16px", md: "32px" }}
      paddingX={{ base: "8px", md: "32px" }} // Adjust padding based on screen size
      borderRadius="8px"
      alignItems="space-between"
      spacing="16px"
      fontSize="md" // Adjust font size based on screen size
      flexDirection={{ base: "row", md: "column" }} // Change direction based on screen size
      onClick={handleCardClick} // Ha
    >
      <HStack>
        <Image
          borderRadius="full"
          boxSize="50px"
          src="https://i.ibb.co/R91rN3Q/statechain.png"
          mb={{ base: "16px", md: 0 }} // Adjust margin bottom based on screen size
        />
        <Flex w="full" alignItems="center" justifyContent="space-between">
          <VStack
            alignItems="flex-start"
            spacing="8px"
            mr={{ base: 0, md: "16px" }}
            paddingBottom="20px"
          >
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

          <VStack
            alignItems="flex-end"
            spacing="8px"
            w={{ base: "100%", md: "50%" }}
          >
            <Progress value={50} size="xs" colorScheme="pink" w="100%" />
            <Text>Time to live: {val.n_lock_time}</Text>
          </VStack>
        </Flex>
      </HStack>
      {showDetailButton && (
        <Box
          textAlign={"center"}
          borderTop={"1px"}
          borderTopColor={"cyan.200"}
          p={"8px 4px"}
        >
          <Button
            rightIcon={<IoIosArrowForward />}
            textColor={"cyan.200"}
            variant="link"
            w="40%"
            onClick={handleDetailButtonClick}
          >
            {/* <Link
              href={`profile/${b64EncodeUnicode(deriv)}/statecoins/${val.statechain_id}`}
            > */}
            Details
            {/* </Link> */}
          </Button>
        </Box>
      )}
    </VStack>
  );
}
