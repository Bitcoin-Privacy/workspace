import React from "react";
import Head from "next/head";
import {
  Tabs,
  TabList,
  TabPanels,
  Tab,
  TabPanel,
  TabIndicator,
  Spacer,
  Grid,
  GridItem,
} from "@chakra-ui/react";
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
import { Layout, NavBar } from "@/components";
import { CoinJoinRoomCard } from "@/components";
import { useProfilePage } from "@/hooks";
import { UTXOCard } from "@/components/utxo-card";

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
    methods: {
      onCopy,
      onSendBtnClick,
      onDepositBtnClick,
      onSendStatecoinBtnClick,
      onWithdrawBtnClick,
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
    },
  ];

  return (
    <React.Fragment>
      <Head>
        <title>Home</title>
      </Head>
      <Layout>
        <VStack
          textAlign="center"
          spacing="8px"
          h="100%"
          overflowY="scroll"
          p="20px 16px"
        >
          <NavBar title={"Account " + deriv.slice(0, deriv.indexOf("/"))} />
          <VStack spacing="36px" w="90%">
            <VStack
              h="100%"
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
                  <Image
                    borderRadius="full"
                    boxSize="50px"
                    src="https://bit.ly/dan-abramov"
                  />
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
                    {balanceQuery.data !== undefined
                      ? balanceQuery.data / 10000000
                      : "-"}{" "}
                    BTC
                  </Text>
                  <Text fontSize="16px"> 0 Statecoin in the wallet</Text>
                </VStack>
              </Flex>

              <HStack px="2px">
                {featureButtons.map((feature, index) => {
                  return (
                    <GridItem>
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
                    </GridItem>
                  );
                })}
              </HStack>
            </VStack>

            <Grid templateColumns="repeat(3, 1fr)" gap={24}>
              <GridItem w="100%" bg="gray.900" p="6px 6px" borderRadius="8px">
                <Text fontSize="16px" fontWeight="200" color="#aaa">
                  {" "}
                  Connected to Server
                </Text>
              </GridItem>
              <GridItem w="100%" bg="gray.900" p="6px 6px" borderRadius="8px">
                <Text fontSize="16px" fontWeight="200" color="#aaa">
                  {" "}
                  Connected to CoinJoin
                </Text>
              </GridItem>
              <GridItem w="100%" bg="gray.900" p="6px 6px" borderRadius="8px">
                <Text fontSize="16px" fontWeight="200" color="#aaa">
                  {" "}
                  Connected to bitcoin
                </Text>
              </GridItem>
            </Grid>

            <Tabs isFitted variant="unstyled" w="100%">
              <TabList>
                <Tab fontSize="18px" fontWeight="200" color="#aaa">
                  UTXO
                </Tab>
                <Tab fontSize="18px" fontWeight="200" color="#aaa">
                  CoinJoin
                </Tab>
                <Tab fontSize="18px" fontWeight="200" color="#aaa">
                  Statechain
                </Tab>
              </TabList>
              <TabIndicator
                mt="-1.5px"
                height="2px"
                bg="cyan.200"
                borderRadius="1px"
              />
              <TabPanels>
                <TabPanel>
                  <VStack overflowY="scroll" h="100%" w="100%">
                    {listUtxoQuery.data?.map((val, index) => (
                      <UTXOCard index={index} val={val} />
                    ))}
                  </VStack>
                </TabPanel>
                <TabPanel>
                  <VStack overflowY="scroll" h="100%">
                    {listRoomsQuery.data?.map((val, id) => (
                      <CoinJoinRoomCard key={id} data={val} deriv={deriv} />
                    ))}
                  </VStack>
                </TabPanel>
                <TabPanel>
                  <Text fontSize="12px" fontWeight="200" color="#aaa">
                    Statechain
                  </Text>
                </TabPanel>
              </TabPanels>
            </Tabs>
          </VStack>
        </VStack>
      </Layout>
    </React.Fragment>
  );
}
