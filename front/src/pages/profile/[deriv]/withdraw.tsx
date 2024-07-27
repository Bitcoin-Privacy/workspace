import React from "react";
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
import { useSendPage } from "@/hooks";

import { Layout } from "@/components";
import { STATECOIN_FEE } from "@/consts";
import { useRouter } from "next/router";

const INPUT_WIDTH = "90%";

export default function Withdraw() {
  const router = useRouter();
  const {
    states: { deriv, form, isLoading },
    methods: { handleFormSubmit },
  } = useSendPage();

  return (
    <Layout header title={"Account " + deriv.slice(0, deriv.indexOf("/"))}>
      <form onSubmit={handleFormSubmit}>
        <VStack textAlign="center" p="0px 16px" spacing="20px">
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
                <Text>Transaction Details</Text>
                <Spacer />
                <Box
                  borderWidth={"1px"}
                  borderColor={"white"}
                  p="4px 8px"
                  borderRadius={"8px"}
                >
                  <Text>Fee: {STATECOIN_FEE} stats</Text>
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
                  alignSelf={"center"}
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
                  Send to this address
                </Button>
              </HStack>
            </VStack>
          </HStack>
        </VStack>
      </form>
    </Layout>
  );
}
