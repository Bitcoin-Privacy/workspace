import React, { useState } from "react";
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
  Box,
  Link,
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalBody,
  ModalFooter,
  useDisclosure,
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
import { StateChainCard } from "@/components/statechain-card";
import { StatechainApi } from "@/apis";

export default function ProfilePage() {
  const {
    states: {
      deriv,
      addr,
      hasCopied,
      listUtxoQuery,
      balanceQuery,
      listRoomsQuery,
      listTransferStatecoinsQuery,
      listStatecoinsQuery,
    },
    methods: {
      router,
      onCopy,
      onSendBtnClick,
      onDepositBtnClick,
      onSendStatecoinBtnClick,
      onWithdrawBtnClick,
      onReceiveStatecoinBtnClick,
      //onVerifyTransferStatecoinClick,
      onDetailButtonClick,
    },
  } = useProfilePage();

  const [isVerifying, setIsVerifying] = useState<boolean>(false);
  const [verifyError, setVerifyError] = useState<string>();
  const { isOpen, onOpen, onClose } = useDisclosure();
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
                    {" "}
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

            <Tabs isFitted variant="unstyled" w="100%">
              <TabList>
                <Tab fontSize="18px" fontWeight="200" color="#aaa">
                  Statechain
                </Tab>
                <Tab fontSize="18px" fontWeight="200" color="#aaa">
                  Statechain transfer
                </Tab>
                <Tab fontSize="18px" fontWeight="200" color="#aaa">
                  UTXO
                </Tab>
                <Tab fontSize="18px" fontWeight="200" color="#aaa">
                  CoinJoin
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
                  <VStack h="100%" w="100%">
                    {listStatecoinsQuery.data?.map((val, index) => (
                      <StateChainCard val={val} key={index} deriv={deriv} />
                    ))}
                  </VStack>
                </TabPanel>
                <TabPanel>
                  <VStack h="100%" w="100%">
                    {listTransferStatecoinsQuery.data?.map((val, index) => (
                      <HStack
                        color="white"
                        textAlign="start"
                        w="90%"
                        maxW="90%"
                        bg="#3a3a3a"
                        p="8px 16px"
                        borderRadius="8px"
                        dir="row"
                        alignItems={"center"}
                        spacing="8px"
                      >
                        <Image
                          borderRadius="full"
                          boxSize="50px"
                          src="https://i.ibb.co/R91rN3Q/statechain.png"
                        />
                        <Flex w="full" alignItems={"center"}>
                          <VStack alignItems={"flex-start"} spacing="8px">
                            {/* <Text
                              isTruncated
                              maxW={"120px"}
                              fontSize={"16"}
                              fontWeight={"400"}
                            >
                              Id : {val.statechain_id}
                            </Text> */}
                            <Text
                              isTruncated
                              maxW={"160px"}
                              fontSize={"16"}
                              fontWeight={"400"}
                            >
                              Authkey : {val.auth_key}
                            </Text>
                          </VStack>

                          <Spacer />

                          <VStack alignItems={"end"} spacing={"8px"} w="100%">
                            <Button
                              // onClick={() =>
                              //   onVerifyTransferStatecoinClick(
                              //     deriv,
                              //     val.transfer_message,
                              //     val.auth_key
                              //   )
                              // }
                              onClick={async () => {
                                setIsVerifying(true);
                                try {
                                  let res =
                                    await StatechainApi.verifyTransferStatecoin(
                                      deriv,
                                      val.transfer_message,
                                      val.auth_key
                                    );
                                  console.log("verify statecoin :", res);
                                } catch (e: any) {
                                  console.error(
                                    "Error when verify statecoin:",
                                    e
                                  );
                                  setVerifyError(e);
                                } finally {
                                  setIsVerifying(false);
                                }
                              }}
                            >
                              {" "}
                              Verify
                            </Button>
                          </VStack>
                        </Flex>
                      </HStack>
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
                  {listRoomsQuery.data?.map((val, id) => (
                    <CoinJoinRoomCard key={id} data={val} deriv={deriv} />
                  ))}
                </TabPanel>
              </TabPanels>
            </Tabs>
          </VStack>
        </VStack>

        <Modal
          closeOnOverlayClick={false}
          isOpen={!!verifyError}
          onClose={onClose}
        >
          <ModalOverlay />
          <ModalContent>
            <ModalHeader>ERROR !!!!</ModalHeader>

            <ModalBody pb={6}>{verifyError}</ModalBody>
            <ModalFooter>
              <Button
                colorScheme="red"
                onClick={() => {
                  onClose();
                  setVerifyError(undefined);
                }}
              >
                Cancel
              </Button>
            </ModalFooter>
          </ModalContent>
        </Modal>
      </Layout>
    </React.Fragment>
  );
}
