import React, { FC } from "react";
import {
  Tabs,
  TabList,
  TabPanels,
  Tab,
  TabPanel,
  TabIndicator,
  Spacer,
} from "@chakra-ui/react";
import { Text, VStack, Button, HStack, Image, Flex } from "@chakra-ui/react";
import { CoinJoinRoomCard } from "@/components";
import { useProfilePage } from "@/hooks";
import { UTXOCard } from "@/components/utxo-card";
import { ListStateChain } from "@/components/profile/list-statechain";

interface IProfilePanel {}
export const ProfilePannel: FC<IProfilePanel> = (props) => {
  const {
    states: {
      deriv,
      listUtxoQuery,
      listRoomsQuery,
      listTransferStatecoinsQuery,
      listStatecoinsQuery,
    },
    methods: { onVerifyTransferStatecoinClick },
  } = useProfilePage();
  return (
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
      <TabIndicator mt="-1.5px" height="2px" bg="cyan.200" borderRadius="1px" />
      <TabPanels>
        <TabPanel>
          <ListStateChain
            isLoading={listStatecoinsQuery.isLoading}
            isError={listStatecoinsQuery.isError}
            deriv={deriv}
            data={listStatecoinsQuery.data ?? []}
          />
        </TabPanel>
        <TabPanel>
          <VStack h="100%" w="100%">
            {listTransferStatecoinsQuery.data?.map((val, index) => (
              <HStack
                key={index}
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
                      onClick={() =>
                        onVerifyTransferStatecoinClick(
                          deriv,
                          val.transfer_message,
                          val.auth_key,
                        )
                      }
                    >
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
          <VStack>
            {listRoomsQuery.data?.map((val, id) => (
              <CoinJoinRoomCard key={id} data={val} deriv={deriv} />
            ))}
          </VStack>
        </TabPanel>
      </TabPanels>
    </Tabs>
  );
};
