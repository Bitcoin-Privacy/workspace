import React from "react";
import Head from "next/head";
import { Spacer, Box, Link } from "@chakra-ui/react";
import { Text, VStack, Button, HStack, Image, Flex } from "@chakra-ui/react";
import {
  FiArrowDownLeft,
  FiArrowUpRight,
  FiCheck,
  FiCopy,
} from "react-icons/fi";
import { TiMinus } from "react-icons/ti";
import { FaPlus } from "react-icons/fa";
import { IoMdSwap } from "react-icons/io";
import { Layout, NavBar, ProfilePannel } from "@/components";
import { useProfilePage } from "@/hooks";

export default function ProfilePage() {
  const {
    states: { deriv, addr, hasCopied, balanceQuery },
    methods: {
      router,
      onCopy,
      onSendBtnClick,
      onDepositBtnClick,
      onSendStatecoinBtnClick,
      onWithdrawBtnClick,
      onReceiveStatecoinBtnClick,
    },
  } = useProfilePage();

  const featureButtons = [
    {
      name: "Deposit",
      icon: <FaPlus />,
      onClick: onDepositBtnClick,
    },
    {
      name: "Withdraw",
      icon: <TiMinus />,
      onClick: onWithdrawBtnClick,
    },
    {
      name: "Send Statecoin",
      icon: <IoMdSwap />,
      onClick: onSendStatecoinBtnClick,
    },
    {
      name: "Send",
      icon: <FiArrowUpRight />,
      onClick: onSendBtnClick,
    },
    {
      name: "Receive",
      icon: <FiArrowDownLeft />,
      onClick: onReceiveStatecoinBtnClick,
    },
  ];

  return (
    <React.Fragment>
      <Head>
        <title>Home</title>
      </Head>

      <Layout>
        <NavBar title={"Account " + deriv.slice(0, deriv.indexOf("/"))} />
        <VStack spacing="8px" h="100vh" w="100%" p="20px 16px">
          <VStack spacing="36px" w="90%">
            <VStack
              justifyContent={"center"}
              id="control_box"
              bg={"gray.900"}
              borderRadius={"8px"}
              p="20px 16px"
              spacing="80px"
              w="full"
            >
              <Flex w="full" alignItems={"center"} justifyContent={"center"}>
                <HStack>
                  <Link
                    href={`https://blockstream.info/testnet/address/${addr}`}
                    rel="noopener noreferrer"
                    target="_blank"
                  >
                    <Image
                      borderRadius="full"
                      boxSize="50px"
                      src="https://bit.ly/dan-abramov"
                    />
                  </Link>
                  <Button
                    onClick={onCopy}
                    bgColor={"cyan.200"}
                    rightIcon={hasCopied ? <FiCheck /> : <FiCopy />}
                    borderRadius={"16"}
                  >
                    <Text isTruncated maxW={"320px"} p="5px">
                      {addr}
                    </Text>
                  </Button>
                </HStack>
                <Spacer />

                <VStack
                  bg={"gray.600"}
                  fontSize="24px"
                  fontWeight="200"
                  textColor="white"
                  p="8px 16px"
                  borderRadius={"8px"}
                >
                  <Text>
                    {balanceQuery.data !== undefined ? balanceQuery.data : "-"}{" "}
                    Sats
                  </Text>
                  <Text fontSize="16px"> 0 Statecoin in the wallet</Text>
                </VStack>
              </Flex>

              <HStack
                direction={{ base: "column", md: "row" }}
                spacing={{ base: 4, md: 2 }}
                wrap="wrap"
              >
                {featureButtons.map((feature, index) => {
                  return (
                    <Button
                      key={index}
                      bgColor="cyan.200"
                      leftIcon={feature.icon}
                      onClick={feature.onClick}
                      fontSize="16px"
                      borderRadius="full"
                      p="8px 16px"
                    >
                      {feature.name}
                    </Button>
                  );
                })}
              </HStack>
            </VStack>

            <HStack
              w="100%"
              color="white"
              justifyContent={"space-between"}
              direction={{ base: "column", md: "row" }}
              spacing={{ base: 16, md: 4 }}
              wrap="wrap"
              p="0px 24px"
            >
              <Box bg="gray.900" p="12px 12px" borderRadius="8px">
                <Text> Connect to server</Text>
              </Box>
              <Box bg="gray.900" p="12px 12px" borderRadius="8px">
                <Text> Connect to CoinJoin</Text>
              </Box>
              <Box
                bg="gray.900"
                p="12px 12px"
                borderRadius="8px"
                onClick={router.reload}
              >
                <Text> Connect to Statecoin</Text>
              </Box>
            </HStack>
            <ProfilePannel />
          </VStack>
        </VStack>
      </Layout>
    </React.Fragment>
  );
}
