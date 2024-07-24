import React from "react";
import { Box, Link } from "@chakra-ui/react";
import { Text, VStack, Button, HStack, Image, Flex } from "@chakra-ui/react";
import { FiArrowDownLeft, FiArrowUpRight } from "react-icons/fi";
import { TiMinus } from "react-icons/ti";
import { FaPlus } from "react-icons/fa";
import { IoMdSwap } from "react-icons/io";

import { Copier, Layout, ProfilePannel } from "@/components";
import { useProfilePage } from "@/hooks";

export default function ProfilePage() {
  const {
    states: { deriv, addr, balanceQuery },
    methods: {
      router,
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
    <Layout header back>
      <VStack spacing="8px" h="100vh" w="100%" p="20px 16px">
        <VStack spacing="36px" w="90%">
          <VStack
            justifyContent="center"
            id="control_box"
            bg="gray.900"
            borderRadius="8px"
            p="20px 16px"
            spacing="80px"
            w="full"
          >
            <Flex w="full" alignItems="start" justifyContent="space-between">
              <HStack w="100%" alignItems="start" flex="1">
                <Image borderRadius="full" boxSize="54px" src="/avatar.jpeg" />
                <VStack align="start" pl="5px" w="100%" flex="1">
                  <Text fontWeight="700" fontSize="20px">
                    Account {deriv.slice(0, deriv.indexOf("/"))}
                  </Text>
                  <Copier content={addr} />
                  <Link
                    display="block"
                    href={`https://blockstream.info/testnet/address/${addr}`}
                    rel="noopener noreferrer"
                    target="_blank"
                  >
                    View on explorer
                  </Link>
                </VStack>
              </HStack>
              <VStack
                bg="gray.600"
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
              justify="center"
              w="100%"
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
  );
}
