import React from "react";
import Head from "next/head";

import { Box, Text, VStack, Button, HStack } from "@chakra-ui/react";
import {
  FiArrowDownLeft,
  FiArrowUpRight,
  FiCheck,
  FiCopy,
} from "react-icons/fi";

import { Layout, NavBar } from "@/components";
import { CoinJoinRoomCard } from "@/components";
import { useProfilePage } from "@/hooks";

export default function ProfilePage() {
  const {
    states: {
      deriv,
      addr,
      hasCopied,
      listUtxoQuery,
      balanceQuery,
      listRoomsQuery,
    },
    methods: { onCopy, onSendBtnClick },
  } = useProfilePage();

  const featureButtons = [
    {
      name: "Send",
      icon: <FiArrowUpRight />,
      onClick: onSendBtnClick,
    },
    {
      name: "Receive",
      icon: <FiArrowDownLeft />,
    },
  ];

  return (
    <React.Fragment>
      <Head>
        <title>Home</title>
      </Head>
      <Layout>
        <VStack textAlign="center" p="0px 16px" spacing="8px" h="100%">
          <HStack justify="start" w="100%">
            <NavBar title={"Account " + deriv.slice(0, deriv.indexOf("/"))} />
          </HStack>
          <Button
            onClick={onCopy}
            bgColor={"cyan.200"}
            rightIcon={hasCopied ? <FiCheck /> : <FiCopy />}
            borderRadius={"16"}
          >
            <Text isTruncated maxW={"120px"} p="5px">
              {addr}
            </Text>
          </Button>
          <Text fontSize="24px" fontWeight="400" color="#aaa">
            {balanceQuery.data !== undefined
              ? balanceQuery.data / 10000000
              : "-"}{" "}
            BTC
          </Text>
          <Text fontSize="16" fontWeight="200" color="#aaa">
            Balance: {balanceQuery.data ?? "-"} sats
          </Text>
          <HStack spacing="8px">
            {featureButtons.map((feature, index) => {
              return (
                <Button
                  key={index}
                  bgColor="cyan.200"
                  leftIcon={feature.icon}
                  onClick={feature.onClick}
                  fontSize="16px"
                  borderRadius="full"
                  p="10px 20px"
                >
                  {feature.name}
                </Button>
              );
            })}
          </HStack>

          <HStack alignItems={"start"} h="100%" overflowY="hidden" mb="20px">
            <Box display="flex" flexDir="column" h="100%" minW="300px">
              <Text fontSize="16" fontWeight="700" color="#aaa" mb="5px">
                CoinJoin Tx
              </Text>
              <VStack overflowY="auto" h="100%">
                {listRoomsQuery.data?.map((val, id) => (
                  <CoinJoinRoomCard key={id} data={val} deriv={deriv} />
                ))}
              </VStack>
            </Box>
            <Box display="flex" flexDir="column" h="100%" minW="300px">
              <Text fontSize="16" fontWeight="700" color="#aaa" mb="5px">
                UTXOs
              </Text>
              <VStack overflowY="auto" h="100%">
                {listUtxoQuery.data?.map((val, index) => (
                  <Box
                    key={index}
                    color="white"
                    textAlign="start"
                    maxW="300px"
                    bg="#3a3a3a"
                    p="8px 16px"
                    borderRadius="8px"
                  >
                    <Text noOfLines={1} wordBreak="break-all">
                      TxID: {val.txid}
                    </Text>
                    <Text>Vout: {val.vout}</Text>
                    <Text>Value: {val.value}</Text>
                  </Box>
                ))}
              </VStack>
            </Box>
          </HStack>
        </VStack>
      </Layout>
    </React.Fragment>
  );
}
