import React, { useMemo } from "react";
import {
  Box,
  Text,
  VStack,
  Button,
  Input,
  HStack,
  Spacer,
  Flex,
} from "@chakra-ui/react";
import { useRouter } from "next/router";
import { useSendPage } from "@/hooks";
import { ChakraStylesConfig } from "chakra-react-select";

import { Layout, NavBar } from "@/components";

const INPUT_WIDTH = "90%";

export default function Withdraw() {
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
    <Layout>
      <form onSubmit={handleFormSubmit}>
        <VStack textAlign="center" p="0px 16px" spacing="20px">
          <HStack justify="start" w="100%">
            <NavBar title={"Account " + deriv.slice(0, deriv.indexOf("/"))} />
          </HStack>
          <Text color="white" fontWeight="700" fontSize="18px">
            Withdraw Statecoin
          </Text>

          <HStack
            alignItems={"flex-start"}
            color={"white"}
            p="0px 8px"
            w="full"
          >
            <VStack bg={"gray.800"} borderRadius={"8px"} p="16px 24px" w="full">
              <Text> Select statecoin to withdraw</Text>
            </VStack>

            <VStack
              w="full"
              alignItems={"start"}
              bg={"gray.800"}
              borderRadius={"8px"}
              p="16px 24px"
              spacing="16px"
            >
              <Flex w="100%" alignItems={"center"}>
                <Text> Transaction Details</Text>
                <Spacer />
                <Box
                  borderWidth={"1px"}
                  borderColor={"white"}
                  p="4px 8px"
                  borderRadius={"8px"}
                >
                  <Text> Fee : 10000</Text>
                </Box>
              </Flex>

              <HStack w="full" justify="space-between">
                <Text w="20%" color="white">
                  Address:
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

              <Button alignSelf={"center"}> Send to this address</Button>
            </VStack>
          </HStack>
        </VStack>
      </form>
    </Layout>
  );
}
