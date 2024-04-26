import React from "react";
import {
  Tabs,
  TabList,
  TabPanels,
  Tab,
  TabPanel,
  TabIndicator,
  Box,
} from "@chakra-ui/react";
import { Text, VStack, Button, HStack, Image, Flex } from "@chakra-ui/react";
import {
  FiArrowDownLeft,
  FiArrowLeft,
  FiArrowUpRight,
  FiCheck,
  FiCopy,
} from "react-icons/fi";
import { TiMinus } from "react-icons/ti";
import { FaPlus } from "react-icons/fa";
import { IoMdSwap } from "react-icons/io";
import { Layout } from "@/components";
import { CoinJoinRoomCard } from "@/components";
import { useProfilePage } from "@/hooks";
import { UTXOCard } from "@/components/utxo-card";
import { StateChainDto } from "@/dtos/statechain.dto";
import { StateChainCard } from "@/components/statechain-card";
import { useRouter } from "next/router";

export default function ProfilePage() {
  const {
    states: {
      deriv,
      addr,
      hasCopied,
      listUtxoQuery,
      balanceQuery,
      listRoomsQuery,
      listStatecoinsQuery,
    },
    methods: {
      onCopy,
      onSendBtnClick,
      onDepositBtnClick,
      onSendStatecoinBtnClick,
      onWithdrawBtnClick,
      onReceiveStatecoinBtnClick,
    },
  } = useProfilePage();

  const mockStatechain: StateChainDto[] = [
    {
      txid: "asoo4545o4o5ji4oj5oi4j54j54h;lkfs",
      address: "4545taasdfasdfasdfsddsdddasdf",
      n_locktime: 34234234234234,
      value: 0.0001,
    },
    {
      txid: "asoo4545o4o5jisj5oi4j54j54h;lkfs",
      address: "4545taasdfasdasdsdsasdfasdfasdf",
      n_locktime: 34234234234234,
      value: 0.0001,
    },
    {
      txid: "asoo4545o4o5ji4s5oi4j54j54h;lkfs",
      address: "4545taasdfasdsdsdsasdfasdf",
      n_locktime: 34234234234234,
      value: 0.0001,
    },
  ];

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
  const router = useRouter();

  return (
    <Layout
      title={
        <Button
          variant="link"
          leftIcon={<FiArrowLeft />}
          color="#a6a6a6"
          onClick={() => router.back()}
          pos="relative"
        >
          Back
        </Button>
      }
    >
      <VStack spacing="36px" w="100%">
        <VStack
          justifyContent={"center"}
          id="control_box"
          bg={"gray.900"}
          borderRadius={"8px"}
          p="20px 16px"
          spacing="80px"
          w="full"
        >
          <Flex w="full" alignItems="center" justifyContent="space-between">
            <HStack>
              <Image
                alt=""
                borderRadius="full"
                boxSize="50px"
                src="https://bit.ly/dan-abramov"
              />
              <Text>{"Account " + deriv.slice(0, deriv.indexOf("/"))}</Text>
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
                  ? balanceQuery.data / 100000000
                  : "-"}{" "}
                BTC
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
          <Box bg="gray.900" p="12px 12px" borderRadius="8px">
            <Text> Connect to Statecoin</Text>
          </Box>
        </HStack>

        <Tabs isFitted variant="unstyled" w="100%">
          <TabList>
            <Tab fontSize="18px" fontWeight="200" color="#aaa">
              Statechain
            </Tab>
            <Tab fontSize="18px" fontWeight="200" color="#aaa">
              UTXO
            </Tab>
            <Tab fontSize="18px" fontWeight="200" color="#aaa">
              CoinJoin
            </Tab>
          </TabList>
          <TabIndicator height="2px" bg="cyan.200" borderRadius="1px" />
          <TabPanels>
            <TabPanel>
              <VStack h="100%" w="100%">
                {listStatecoinsQuery.data?.map((val, index) => (
                  <StateChainCard val={val} key={index} />
                ))}
              </VStack>
            </TabPanel>
            <TabPanel>
              <VStack overflowY="scroll" h="100%">
                {listUtxoQuery.data?.map((val, index) => (
                  <UTXOCard key={index} val={val} />
                ))}
              </VStack>
            </TabPanel>
            <TabPanel>
              <Text fontSize="12px" fontWeight="200" color="#aaa">
                <VStack h="100%" w="100%" spacing="8px">
                  {listRoomsQuery.data?.map((val, id) => (
                    <CoinJoinRoomCard key={id} data={val} deriv={deriv} />
                  ))}
                </VStack>
              </Text>
            </TabPanel>
          </TabPanels>
        </Tabs>
      </VStack>
    </Layout>
  );
}
