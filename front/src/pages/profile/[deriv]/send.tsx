import React, { useMemo } from "react";
import {
  Box,
  Text,
  VStack,
  Button,
  Input,
  HStack,
  InputGroup,
  InputRightAddon,
  FormControl,
  FormErrorMessage,
  Divider,
  Spinner,
} from "@chakra-ui/react";
import { useRouter } from "next/router";
import { useSendPage } from "@/hooks";
import { Select } from "chakra-react-select";

import { Error, Layout, Loading } from "@/components";
import { TxStrategyEnum } from "@/dtos";
import { COINJOIN_FEE } from "@/consts";
import { convertBtcToSats, convertSatsToBtc } from "@/utils";
import { selectStyles } from "@/styles/components";

const INPUT_WIDTH = "75%";

type TransactionOption = {
  label: string;
  value: TxStrategyEnum;
};

export default function Send() {
  const router = useRouter();
  const {
    states: { deriv, form, isLoading, balanceQuery },
    methods: { handleFormSubmit },
  } = useSendPage();

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

  if (balanceQuery.isLoading)
    return (
      <Layout header title={"Account " + deriv.slice(0, deriv.indexOf("/"))}>
        <Loading content="Fetching your balance..." />
      </Layout>
    );

  if (!balanceQuery.data) {
    return (
      <Layout header title={"Account " + deriv.slice(0, deriv.indexOf("/"))}>
        <Error content="Failed to fetch your balance." />
      </Layout>
    );
  }

  return (
    <Layout header title={"Account " + deriv.slice(0, deriv.indexOf("/"))}>
      <form onSubmit={handleFormSubmit}>
        <VStack textAlign="center" p="0px 16px" spacing="20px">
          <Text color="white" fontWeight="700" fontSize="18px">
            Create transaction
          </Text>
          <VStack width="100%" maxW="500px" spacing="16px">
            <FormControl isInvalid={!!form.formState.errors.address}>
              <HStack w="full" justify="space-between">
                <Text color="white">Address:</Text>
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
                    color="white"
                    {...form.register("amount", {
                      required: "Amount is required",
                      pattern: {
                        value: /^[0-9]+(?:\.[0-9]{0,8})?$/,
                        message:
                          "Amount should be a floating-point number with at most 8 decimal places.",
                      },
                      max: {
                        value: convertSatsToBtc(
                          balanceQuery.data - COINJOIN_FEE,
                        ),
                        message: "Balance is not enough",
                      },
                      min: {
                        value: 0.00000001,
                        message: "Amount must larget than or equal to 1 sat",
                      },
                    })}
                  />
                  <InputRightAddon
                    w="82px"
                    justifyContent="center"
                    color="black"
                  >
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
                chakraStyles={selectStyles(INPUT_WIDTH)}
                colorScheme="purple"
                options={options}
                defaultValue={options[0]}
                onChange={(e) => {
                  if (e != null && typeof e == "object" && "value" in e) {
                    form.setValue(
                      "strategy",
                      TxStrategyEnum[e.value as keyof typeof TxStrategyEnum],
                    );
                  }
                }}
              />
            </HStack>
            <Box width="full" textColor="white">
              <HStack justify="space-between">
                <Text>Current balance:</Text>
                <Text>{`${convertSatsToBtc(balanceQuery.data)} BTC`}</Text>
              </HStack>
              <HStack justify="space-between">
                <Text>Fee:</Text>
                <Text>{convertSatsToBtc(COINJOIN_FEE)} BTC</Text>
              </HStack>
              <Divider my="10px" />
              <HStack justify="space-between">
                <Text>Spend:</Text>
                <Text>
                  {`${convertSatsToBtc(
                    convertBtcToSats(form.watch("amount")) + COINJOIN_FEE,
                  )} BTC`}
                </Text>
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
                let formc = form.watch();
                return !formc.amount || !formc.address;
              })()}
            >
              Send
            </Button>
          </HStack>
        </VStack>
      </form>
    </Layout>
  );
}
