import { StateCoinDto } from "@/dtos/statechain.dto";
import {
  Box,
  Text,
  Image,
  VStack,
  Flex,
  Button,
  Badge,
  Avatar,
} from "@chakra-ui/react";
import { useClipboard } from "@chakra-ui/react";
import { useRouter } from "next/router";
import { FiCheck, FiCopy } from "react-icons/fi";
import { IoIosArrowForward } from "react-icons/io";

interface StateChainCardProps {
  val: StateCoinDto;
  key: number;
  deriv: string;
}

export function StateChainCard(props: StateChainCardProps) {
  const { val, key } = props;
  const router = useRouter();
  const { onCopy, hasCopied } = useClipboard(val.statechain_id);
  const handleDetailButtonClick = () => {
    console.log(val.statechain_id);
    router.push(`${router.asPath}/statecoins/${val.statechain_id}`);
  };

  return (
    <Flex
      key={key}
      w="100%"
      bg={"gray.900"}
      borderRadius="8px"
      px="16px"
      pt="16px"
      justifyContent={"space-between"}
      direction={"column"}
      wrap={"wrap"}
    >
      <Flex justifyContent={"space-between"} mb={"6px"}>
        <Avatar h="54px" w="54px" src="/statecoin-icon.png" />
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
              Statecoin
            </Badge>
            <Button
              onClick={onCopy}
              bg={"cyan.200"}
              variant="solid"
              rightIcon={hasCopied ? <FiCheck /> : <FiCopy />}
              borderRadius="8px"
              maxW={{ base: "60%", md: "100%" }} // Ensure max width is 100%
              textAlign={"left"}
            >
              <Text fontSize="16" isTruncated>
                TxId: {val.statechain_id}
              </Text>
            </Button>
          </VStack>
          <Flex
            h="full"
            direction={"column"}
            justifyContent={"space-around"}
            textAlign={"center"}
            alignItems={"end"}
          >
            <Box fontSize={"x-large"} fontWeight="800">
              {" "}
              {val.amount} SAT
            </Box>
            <Text> Due date: {val.n_lock_time}</Text>
          </Flex>
        </Flex>
      </Flex>
      <Box
        w="100%"
        textAlign={"center"}
        borderTop={"1px"}
        borderTopColor={"cyan.200"}
        mt={"8px"}
        alignItems={"center"}
        py="8px"
      >
        <Button
          rightIcon={<IoIosArrowForward />}
          textColor={"cyan.200"}
          variant="link"
          onClick={handleDetailButtonClick}
        >
          {/* <Link
        href={`profile/${b64EncodeUnicode(deriv)}/statecoins/${val.statechain_id}`}
      > */}
          Details
          {/* </Link> */}
        </Button>
      </Box>
    </Flex>
  );
}

// {showDetailButton && (
//   <Box
//     textAlign={"center"}
//     borderTop={"1px"}
//     borderTopColor={"cyan.200"}
//     p={"8px 4px"}
//   >
//     <Button
//       rightIcon={<IoIosArrowForward />}
//       textColor={"cyan.200"}
//       variant="link"
//       w="40%"
//       onClick={handleDetailButtonClick}
//     >
//       {/* <Link
//         href={`profile/${b64EncodeUnicode(deriv)}/statecoins/${val.statechain_id}`}
//       > */}
//       Details
//       {/* </Link> */}
//     </Button>
//   </Box>
// )}
// </VStack>
