import React, { useEffect, useState } from "react";
import {
  Box,
  Text,
  Button,
  Tooltip,
  Flex,
  useDisclosure,
  Modal,
  ModalHeader,
  ModalBody,
  ModalFooter,
  ModalOverlay,
  ModalContent,
} from "@chakra-ui/react";
import { useStatecoinDetail } from "@/hooks/atoms/use-statecoin-detail";
import { Layout, NavBar } from "@/components";
import QRCodeGenerator from "@/components/qr-code-generator";
import { AppApi, StatechainApi } from "@/apis";
import { StatecoinDetailDto } from "@/dtos";
import { open } from "@tauri-apps/api/shell";
import { FaCheckCircle, FaClock } from "react-icons/fa";
export default function StatecoinDetail() {
  const { deriv, statechainId, router } = useStatecoinDetail();
  const [status, setStatus] = useState<boolean>(false);
  const [statecoin, setStatecoin] = useState<StatecoinDetailDto>();
  const getData = async (id: string) => {
    let res = await StatechainApi.getStatecoinDetailById(id);
    setStatecoin(res);
    let sta = await AppApi.getStatus(res.funding_txid);
    setStatus(sta);
  };

  useEffect(() => {
    if (statechainId) {
      getData(statechainId as string);
    }
  }, [statechainId]);

  const { isOpen, onOpen, onClose } = useDisclosure();
  const [isError, setIsError] = useState<boolean>(false);
  const [withdrawError, setWithdrawError] = useState<string>("");

  return (
    <Layout>
      <Modal closeOnOverlayClick={false} isOpen={isError} onClose={onClose}>
        <ModalOverlay />
        <ModalContent>
          <ModalHeader>ERROR !!!!</ModalHeader>

          <ModalBody pb={6}>{withdrawError}</ModalBody>
          <ModalFooter>
            <Button
              colorScheme="red"
              onClick={() => {
                onClose;
                setIsError(false);
              }}
            >
              Cancel
            </Button>
          </ModalFooter>
        </ModalContent>
      </Modal>
      <NavBar title={"Detail Page"} />

      <Flex
        mt="40px"
        h="full"
        borderRadius={"8"}
        justifyContent={"space-between"}
        px="36px"
        w={"full"}
      >
        <Flex
          direction={"column"}
          gap={"12px"}
          p={"10px 10px"}
          alignItems={"center"}
          justifyContent={"center"}
          w="full"
        >
          <QRCodeGenerator text={statecoin?.funding_txid} size="200px" />
          <Tooltip label="View transaction onchain" placement="auto-end">
            <Button
              colorScheme="blue"
              variant="ghost"
              isTruncated
              maxW="80%"
              onClick={() => {
                open(
                  `https://blockstream.info/testnet/tx/${statecoin?.funding_txid}`,
                );
              }}
              p="4px"
              textAlign={{ base: "left", md: "left" }} // Center text on small screens
            >
              <Text isTruncated maxW="400px">
                {statecoin?.aggregated_address}
              </Text>
            </Button>
          </Tooltip>
          <Button
            w="80%"
            maxW="80%"
            colorScheme="red"
            onClick={async () => {
              try {
                let res = await StatechainApi.withdrawStatecoin(
                  statecoin?.statechain_id as string,
                  deriv,
                );
                console.log("Withdraw statecoin API response ", res);
              } catch (error: any) {
                setWithdrawError(error);
                console.log("Withdraw statecoin API error ", error);
                setIsError(true);
              }
            }}
          >
            Withdraw
          </Button>
          <Button w="80%" maxW="80%" colorScheme="teal">
            Broadcast backup transaction
          </Button>
        </Flex>

        <Flex
          direction={"column"}
          gap={"24px"}
          alignItems="flex-start"
          p={6}
          alignSelf={"center"}
          bg={"gray.800"}
          borderRadius={"16px"}
        >
          <Box>
            <Text fontSize={"larger"} fontWeight="bold">
              Statechain ID:
            </Text>
            <Text isTruncated maxW="400px">
              {statecoin?.statechain_id}
            </Text>
          </Box>
          <Box>
            <Text fontSize={"larger"} fontWeight="bold">
              Deposit transaction ID:
            </Text>
            <Text isTruncated maxW="400px">
              {statecoin?.funding_txid}
            </Text>
          </Box>
          <Box>
            <Text fontSize={"larger"} fontWeight="bold">
              Transaction Number (tx_n):
            </Text>
            <Text>{statecoin?.tx_n}</Text>
          </Box>
          <Box>
            <Text fontSize={"larger"} fontWeight="bold">
              Aggregated Address:
            </Text>
            <Text isTruncated maxW={"560px"} textOverflow={"ellipsis"}>
              {statecoin?.aggregated_address}
            </Text>
          </Box>

          <Box>
            <Text fontSize={"larger"} fontWeight="bold">
              Amount:
            </Text>
            <Text>{statecoin?.amount} Sats</Text>
          </Box>
          <Box>
            <Text fontSize={"larger"} fontWeight="bold">
              Time to Live (n_lock_time):
            </Text>
            <Text>{statecoin?.n_lock_time}</Text>
          </Box>
          <Box>
            <Text fontSize={"larger"} fontWeight="bold">
              Created At:
            </Text>
            <Text>{statecoin?.created_at}</Text>
          </Box>
          <Flex align="center" gap="10px">
            <Text fontSize={"larger"} fontWeight="bold">
              Confirm:
            </Text>
            {status ? (
              <FaCheckCircle color="#41c300" />
            ) : (
              <FaClock color="#fa8100" />
            )}
          </Flex>
        </Flex>
      </Flex>
    </Layout>
  );
}
