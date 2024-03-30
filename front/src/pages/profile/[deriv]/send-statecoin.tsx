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

const INPUT_WIDTH = "90%";

export default function SendStateCoin() {
  const router = useRouter();
  const {
    states: { deriv, form, isLoading, balanceQuery },
    methods: { handleFormSubmit },
  } = useSendPage();

  const chakraStyles: ChakraStylesConfig = useMemo(
    () => ({
      menuList: (provided) => ({
        ...provided,
        // ...bgThemeListSearch,
      }),
      menu: (provided) => ({
        ...provided,
        // ...bgThemeListSearch,
      }),
      inputContainer: (provided) => ({
        ...provided,
        fontSize: "14px",
        color: "white",
        textAlign: "start",
      }),
      dropdownIndicator: (provided) => ({
        ...provided,
        w: "80px",
      }),
      control: (provided) => ({
        ...provided,
        background: "transparent",
        fontSize: "12px",
        color: "textSloganHomepage",
      }),
      container: (provided) => ({
        ...provided,
        width: INPUT_WIDTH,
      }),
      singleValue: (provided) => ({
        ...provided,
        fontSize: "14px",
        color: "white",
        textAlign: "start",
      }),
      placeholder: (provided) => ({
        ...provided,
        color: "#a6a6a6",
        fontSize: "14px",
        textAlign: "start",
      }),
    }),
    [],
  );

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
                    Statechain Address:
                  </Text>
                  <Input
                    placeholder="tb1qtperkdhmm9hesga45wzzdzks6rrtejtp2uec40"
                    w={INPUT_WIDTH}
                    color="white"
                    {...form.register("address", {
                      required: "Receiver address is required",
                      pattern: {
                        value: /^(tb1)[a-z0-9]{39,59}$/,
                        message:
                          "Addess should follow P2WPKH format, other type is not supported yet.",
                      },
                    })}
                  />
                </HStack>
                <HStack w="full" justify="space-between">
                  <Text w="20%" color="white">
                    Authen Address:
                  </Text>
                  <Input
                    placeholder="tb1qtperkdhmm9hesga45wzzdzks6rrtejtp2uec40"
                    w={INPUT_WIDTH}
                    color="white"
                    {...form.register("address", {
                      required: "Authen address is required",
                      pattern: {
                        value: /^(tb1)[a-z0-9]{39,59}$/,
                        message:
                          "Addess should follow P2WPKH format, other type is not supported yet.",
                      },
                    })}
                  />
                </HStack>
                <Button alignSelf={"center"}> Send Statecoin</Button>
              </VStack>
            </HStack>
          </VStack>
        </form>
      </Layout>
    </React.Fragment>
  );
}
