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

const INPUT_WIDTH = "75%";

type TransactionOption = {
  label: string;
  value: TxStrategyEnum;
};

export default function Send() {
  const router = useRouter();
  const {
    states: {
      deriv, form, isLoading, balanceQuery
    },
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

  const options: TransactionOption[] = useMemo(
    () => [
      {
        label: "Standard",
        value: TxStrategyEnum.Base,
      },
      {
        label: "Coin Join Protocol",
        value: TxStrategyEnum.CoinJoin,
      },
    ],
    [],
  );

  return (
    <React.Fragment>
      <Head>
        <title>Send Bitcoin</title>
      </Head>
      <Layout>
        <form onSubmit={handleFormSubmit}>
          <VStack textAlign="center" p="0px 16px" spacing="20px">
            <HStack justify="start" w="100%">
              <NavBar title={"Account " + deriv.slice(0, deriv.indexOf("/"))} />
            </HStack>
            <Text color="white" fontWeight="700" fontSize="18px">
              Create transaction
            </Text>
            <VStack width="100%" maxW="500px" spacing="16px" >
              <FormControl isInvalid={!!form.formState.errors.address} >
                <HStack w="full" justify="space-between">
                  <Text color="white">Address:</Text>
                  <Input
                    placeholder="tb1qtperkdhmm9hesga45wzzdzks6rrtejtp2uec40"
                    w={INPUT_WIDTH}
                    color="white"
                    {...form.register("address", { required: "Receiver address is required", pattern: { value: /^(tb1)[a-z0-9]{39,59}$/, message: "Addess should follow P2WPKH format, other type is not supported yet." } })}
                  />
                </HStack>
                {form.formState.errors.address && (
                  <FormErrorMessage justifyContent="end">
                    {form.formState.errors.address.message}
                  </FormErrorMessage>
                )}
              </FormControl>
              <FormControl isInvalid={!!form.formState.errors.amount}>
                <HStack w="full" justify="space-between">
                  <Text color={"white"}>Amount:</Text>
                  <InputGroup w={INPUT_WIDTH}>
                    <Input
                      placeholder="0.12"
                      color={"white"}
                      {...form.register("amount", {
                        required: "Amount is required",
                        pattern: { value: /^[0-9]+(?:\.[0-9]{0,8})?$/, message: "Amount should be a floating-point number with at most 8 decimal places." },
                        max: { value: balanceQuery.data ? balanceQuery.data / 10000000 : Number.MAX_VALUE, message: "Balance is not enough" },
                        min: { value: 0.00000001, message: "Amount must larget than or equal to 1 sat" }
                      })}
                    />
                    <InputRightAddon w="82px" justifyContent="center">
                      BTC
                    </InputRightAddon>
                  </InputGroup>
                </HStack>
                {form.formState.errors.amount && (
                  <FormErrorMessage justifyContent="end">
                    {form.formState.errors.amount.message}
                  </FormErrorMessage>
                )}
              </FormControl>
              <HStack w="full" justify="space-between">
                <Text color="white">Strategy:</Text>
                <Select
                  chakraStyles={chakraStyles}
                  colorScheme="purple"
                  options={options}
                  defaultValue={options[0]}
                  onChange={(e) => {
                    if (e != null && typeof e == "object" && "value" in e) {
                      form.setValue("strategy", TxStrategyEnum[e.value as keyof typeof TxStrategyEnum]);
                    }
                  }}
                />
              </HStack>
              <Box width={"full"}>
                <HStack>
                  <Text color={"white"}>Current balance</Text>
                  <Spacer />
                  <Text color={"white"}>
                    {balanceQuery.data !== undefined
                      ? balanceQuery.data / 10000000
                      : "-"}{" "}
                    BTC
                  </Text>
                </HStack>
                <HStack>
                  <Text color={"white"}>Gas</Text>
                  <Spacer />
                  <Text color={"white"}>0.0003 BTC</Text>
                </HStack>
                <HStack>
                  <Text color={"white"}>Likely in 30 seconds</Text>
                  <Spacer />
                  <Text color={"white"}>Max fee: BTC</Text>
                </HStack>
              </Box>
            </VStack>

            <HStack marginY={"36px"}>
              <Button
                p="10px 50px"
                borderRadius="full"
                colorScheme="blackAlpha"
                flex="1"
                onClick={() => {
                  router.back();
                }}
                isDisabled={isLoading}
              >
                Cancel
              </Button>
              <Button
                p="10px 50px"
                borderRadius="full"
                flex="1"
                type="submit"
                isLoading={isLoading}
                isDisabled={(() => {
                  let formc = form.watch()
                  return !formc.amount || !formc.address
                })()}
              >
                Send
              </Button>
            </HStack>
          </VStack>
        </form>
      </Layout>
    </React.Fragment>
  );
}
