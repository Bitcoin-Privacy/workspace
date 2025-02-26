import React from "react";
import { Avatar, Link } from "@chakra-ui/react";
import { Text, VStack, Button, HStack, Flex } from "@chakra-ui/react";
import { FiArrowDownLeft, FiArrowUpRight } from "react-icons/fi";
import { FaPlus } from "react-icons/fa";
import { IoMdSwap } from "react-icons/io";

import {
  Copier,
  ExplorerLink,
  ExplorerLinkType,
  Layout,
  ProfilePannel,
} from "@/components";
import { useProfilePage } from "@/hooks";

export default function ProfilePage() {
  const {
    states: { deriv, addr, balanceQuery },
    methods: {
      onSendBtnClick,
      onDepositBtnClick,
      onSendStatecoinBtnClick,
      onReceiveStatecoinBtnClick,
    },
  } = useProfilePage();

  const featureButtons = [
    {
      name: "Deposit Statecoin",
      icon: <FaPlus />,
      onClick: onDepositBtnClick,
    },
    {
      name: "Send Statecoin",
      icon: <IoMdSwap />,
      onClick: onSendStatecoinBtnClick,
    },
    {
      name: "Receive Statecoin",
      icon: <FiArrowDownLeft />,
      onClick: onReceiveStatecoinBtnClick,
    },
    {
      name: "Send",
      icon: <FiArrowUpRight />,
      onClick: onSendBtnClick,
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
                <Avatar borderRadius="full" boxSize="54px" src="/avatar.jpeg" />
                <VStack align="start" pl="5px" w="100%" flex="1">
                  <Text fontWeight="700" fontSize="20px">
                    Account {deriv.slice(0, deriv.indexOf("/"))}
                  </Text>
                  <Copier content={addr} />
                  <ExplorerLink id={addr} type={ExplorerLinkType.ADDRESS} />
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
          <ProfilePannel />
        </VStack>
      </VStack>
    </Layout>
  );
}
