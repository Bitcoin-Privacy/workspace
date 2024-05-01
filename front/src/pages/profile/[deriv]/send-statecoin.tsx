import React, { useMemo } from "react";
import Head from "next/head";
import {
  Box,
  Text,
  VStack,
  Button,
  Input,
  HStack,
  Spacer,
  InputGroup,
  InputRightAddon,
  FormControl,
  FormErrorMessage,
} from "@chakra-ui/react";
import { useRouter } from "next/router";
import { useSendPage } from "@/hooks";
import { ChakraStylesConfig, Select } from "chakra-react-select";

import { Layout, NavBar } from "@/components";
import { TxStrategyEnum } from "@/dtos";
import { useSendStateCoinPage } from "@/hooks/pages/use-send-statecoin-page";

const INPUT_WIDTH = "90%";

export default function SendStateCoin() {
  const router = useRouter();
  const {
    states: { deriv, form, isLoading, balanceQuery },
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
            >
              <VStack
                bg={"gray.800"}
                borderRadius={"8px"}
                p="16px 24px"
                w="full"
              >
                <Text> Select statecoin to send</Text>
              </VStack>

              <VStack
                w="full"
                alignItems={"start"}
                bg={"gray.800"}
                borderRadius={"8px"}
                p="16px 24px"
                spacing="16px"
              >
                <Text> Transaction Details</Text>
                <HStack w="full" justify="space-between">
                  <Text w="20%" color="white">
                    New owner pubkey:
                  </Text>
                  <Input
                    placeholder="02ed7faa45188db914f659c8c0676f66b23ab70a650b14463002be496afdd2875f"
                    defaultValue={
                      "02ed7faa45188db914f659c8c0676f66b23ab70a650b14463002be496afdd2875f"
                    }
                    w={INPUT_WIDTH}
                    color="white"
                    {...form.register("o2_pubkey", {
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
                    placeholder="e0e75e9a-07be-11ef-b8b5-a730e8877628"
                    w={INPUT_WIDTH}
                    color="white"
                    defaultValue={"e0e75e9a-07be-11ef-b8b5-a730e8877628"}
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

                <HStack w="full" justify="space-between">
                  <Text w="20%" color="white">
                    New owner auth:
                  </Text>
                  <Input
                    placeholder="5f57150ba6aea631024e02adf71738b69c76101959845ee5121e9f6fd0107e3a"
                    defaultValue={
                      "5f57150ba6aea631024e02adf71738b69c76101959845ee5121e9f6fd0107e3a"
                    }
                    w={INPUT_WIDTH}
                    color="white"
                    {...form.register("o2_authkey", {
                      required: "o2_authkey is required",
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
