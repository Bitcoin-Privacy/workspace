import React, { useEffect, useState } from "react";
import {
  Box,
  Text,
  VStack,
  Button,
  HStack,
  Tooltip,
  Flex,
} from "@chakra-ui/react";
import { useStatecoinDetail } from "@/hooks/atoms/use-statecoin-detail";
import Head from "next/head";
import { Layout, NavBar } from "@/components";
import QRCodeGenerator from "@/components/qr-code-generator";
import { AppApi, StatechainApi } from "@/apis";
import { StatecoinDetailDto } from "@/dtos";
import { open } from "@tauri-apps/api/shell";
import { FaCheckCircle, FaClock } from "react-icons/fa";
export default function StatecoinDetail() {
  const { deriv, statechainId } = useStatecoinDetail();
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

  return (
    <React.Fragment>
      <Head>
        <title> Detail Page</title>
      </Head>
      <Layout>
        <NavBar title={"Detail Page"} />
        <HStack justifyContent={"space-around"} px="16px" w={"full"}>
          <VStack
            spacing={"12px"}
            p={"10px 10px"}
            alignItems={"center"}
            height={"full"}
          >
            <QRCodeGenerator text={statecoin?.funding_txid} size="200px" />
            <Tooltip label="View transaction onchain" placement="auto-end">
              <Button
                colorScheme="blue"
                variant="ghost"
                isTruncated
                maxW="200px"
                onClick={() => {
                  open(
                    `https://blockstream.info/testnet/tx/${statecoin?.funding_txid}`
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
              w={"full"}
              colorScheme="red"
              onClick={async () => {
                let res = await StatechainApi.withdrawStatecoin(
                  statecoin?.statechain_id as string,
                  deriv
                );
                console.log("withdraw ", res);
              }}
            >
              Withdraw
            </Button>
          </VStack>

          <VStack spacing={6} alignItems="flex-start" p={6}>
            <Box>
              <Text fontWeight="bold">Statechain ID:</Text>
              <Text isTruncated maxW="400px">
                {statecoin?.statechain_id}
              </Text>
            </Box>
            <Box>
              <Text fontWeight="bold">Deposit transaction ID:</Text>
              <Text isTruncated maxW="400px">
                {statecoin?.funding_txid}
              </Text>
            </Box>
            <Box>
              <Text fontWeight="bold">Transaction Number (tx_n):</Text>
              <Text>{statecoin?.tx_n}</Text>
            </Box>
            <Box>
              <Text fontWeight="bold">Aggregated Address:</Text>
              <Text isTruncated maxW={"560px"} textOverflow={"ellipsis"}>
                {statecoin?.aggregated_address}
              </Text>
            </Box>

            <Box>
              <Text fontWeight="bold">Amount:</Text>
              <Text>{statecoin?.amount} Sats</Text>
            </Box>
            <Box>
              <Text fontWeight="bold">Time to Live (n_lock_time):</Text>
              <Text>{statecoin?.n_lock_time}</Text>
            </Box>
            <Box>
              <Text fontWeight="bold">Created At:</Text>
              <Text>{statecoin?.created_at}</Text>
            </Box>
            <Flex align="center" gap="10px">
              <Text fontWeight="bold">Confirm:</Text>
              {status ? (
                <FaCheckCircle color="#41c300" />
              ) : (
                <FaClock color="#fa8100" />
              )}
            </Flex>
          </VStack>
        </HStack>
      </Layout>
    </React.Fragment>
  );
}
