import React from "react";
import Head from "next/head";
import { Text, VStack, Button, Input, HStack } from "@chakra-ui/react";

import { Layout, NavBar } from "@/components";
import { useSendStateCoinPage } from "@/hooks/pages/use-send-statecoin-page";
import { StatecoinToSendCard } from "@/components/statecoin-to-send-card";

const INPUT_WIDTH = "90%";

export default function SendStateCoin() {
  const {
    states: { deriv, form, isLoading, listStatecoinsQuery },
    methods: { handleFormSubmit },
  } = useSendStateCoinPage();

  return (
    <React.Fragment>
      <Head>
        <title>Send Statecoin</title>
      </Head>
      <Layout>
        <form onSubmit={handleFormSubmit}>
          <VStack textAlign="center" p="0px 16px" spacing="20px">
            <HStack justify="start" w="100%">
              <NavBar title={"Account " + deriv.slice(0, deriv.indexOf("/"))} />
            </HStack>
            <Text color="white" fontWeight="700" fontSize="18px">
              Send Statecoin
            </Text>

            <HStack
              alignItems={"flex-start"}
              color={"white"}
              p="0px 8px"
              w="full"
              spacing={"16px"}
              justifyContent={"space-between"}
            >
              <VStack h="100%" w="50%" px={"16px"}>
                {listStatecoinsQuery.data?.map((val, index) => (
                  //<StateChainCard val={val} key={index} deriv={deriv} />
                  <StatecoinToSendCard val={val} key={index} deriv={deriv} />
                ))}
              </VStack>
              <VStack
                alignItems={"start"}
                bg={"gray.800"}
                borderRadius={"8px"}
                p="16px 24px"
                spacing="16px"
                w="50%"
              >
                <Text> Transaction Details</Text>
                <HStack w="full" justify="space-between">
                  <Text w="20%" color="white">
                    Statechain address
                  </Text>
                  <Input
                    w={INPUT_WIDTH}
                    color="white"
                    {...form.register("address", {
                      required: "Receiver address is required",
                      // pattern: {
                      //   value: /^(tb1)[a-z0-9]{39,59}$/,
                      //   message:
                      //     "Addess should follow P2WPKH format, other type is not supported yet.",
                      // },
                    })}
                  />
                </HStack>

                <HStack w="full" justify="space-between">
                  <Text w="20%" color="white">
                    Statechain ID
                  </Text>
                  <Input
                    w={INPUT_WIDTH}
                    color="white"
                    {...form.register("statechain_id", {
                      required: "statechain_id is required",
                      // pattern: {
                      //   value: /^(tb1)[a-z0-9]{39,59}$/,
                      //   message:
                      //     "Addess should follow P2WPKH format, other type is not supported yet.",
                      // },
                    })}
                  />
                </HStack>

                <Button
                  alignSelf={"center"}
                  type="submit"
                  isLoading={isLoading}
                >
                  {" "}
                  Send Statecoin
                </Button>
              </VStack>
            </HStack>
          </VStack>
        </form>
      </Layout>
    </React.Fragment>
  );
}
