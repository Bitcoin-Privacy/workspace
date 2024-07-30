import React from "react";
import {
  Text,
  VStack,
  Button,
  Input,
  HStack,
  InputGroup,
  InputRightAddon,
  FormControl,
  FormErrorMessage,
  useDisclosure,
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalBody,
  ModalFooter,
  Box,
  Divider,
} from "@chakra-ui/react";
import { useRouter } from "next/router";
import { Error, Layout, Loading } from "@/components";
import { useDepositPage } from "@/hooks/pages/use-deposit-page";
import { convertBtcToSats, convertSatsToBtc } from "@/utils";
import { STATECOIN_FEE, STATECOIN_MIN } from "@/consts";

const INPUT_WIDTH = "75%";

export default function Deposit() {
  const router = useRouter();
  const { onClose } = useDisclosure();

  const {
    states: { deriv, form, isLoading, balanceQuery, isError },
    methods: { handleFormSubmit, setIsError },
  } = useDepositPage();

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
        <Modal closeOnOverlayClick={false} isOpen={isError} onClose={onClose}>
          <ModalOverlay />
          <ModalContent>
            <ModalHeader>ERROR !!!!</ModalHeader>

            <ModalBody pb={6}>{form.formState.errors.root?.message}</ModalBody>
            <ModalFooter>
              <Button
                colorScheme="red"
                onClick={() => {
                  onClose;
                  setIsError(false);
                }}
              >
                Cancel
              </Button>
            </ModalFooter>
          </ModalContent>
        </Modal>
        <VStack p="0px 16px" spacing="20px">
          <Text color="white" fontWeight="700" fontSize="18px">
            Deposit statecoin
          </Text>
          <VStack width="100%" maxW="500px" spacing="16px">
            <VStack spacing="24px">
              <FormControl isInvalid={!!form.formState.errors.amount}>
                <HStack w="full" justify="space-between">
                  <Text color={"white"}>Amount:</Text>
                  <InputGroup w={INPUT_WIDTH}>
                    <Input
                      placeholder="0.12"
                      color={"white"}
                      {...form.register("amount", {
                        required: "Amount is required",
                        pattern: {
                          value: /^[0-9]+(?:\.[0-9]{0,8})?$/,
                          message:
                            "Amount should be a floating-point number with at most 8 decimal places.",
                        },
                        max: {
                          value: convertSatsToBtc(
                            balanceQuery.data - STATECOIN_FEE,
                          ),
                          message: "Balance is not enough",
                        },
                        min: {
                          value: convertSatsToBtc(STATECOIN_MIN),
                          message: `Amount must larget than or equal to ${STATECOIN_MIN} sats`,
                        },
                      })}
                    />
                    <InputRightAddon
                      w="82px"
                      justifyContent="center"
                      textColor="black"
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
              <Box width="full" textColor="white">
                <HStack justify="space-between">
                  <Text>Current balance:</Text>
                  <Text>{`${convertSatsToBtc(balanceQuery.data)} BTC`}</Text>
                </HStack>
                <HStack justify="space-between">
                  <Text>Fee:</Text>
                  <Text>{convertSatsToBtc(STATECOIN_FEE)} BTC</Text>
                </HStack>
                <Divider my="10px" />
                <HStack justify="space-between">
                  <Text>Spend:</Text>
                  <Text>
                    {`${convertSatsToBtc(
                      convertBtcToSats(form.watch("amount")) + STATECOIN_FEE,
                    )} BTC`}
                  </Text>
                </HStack>
              </Box>

              <HStack>
                <Button
                  p="10px 50px"
                  borderRadius="full"
                  w="180px"
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
                  w="180px"
                  flex="1"
                  type="submit"
                  isLoading={isLoading}
                  isDisabled={(() => {
                    let formc = form.watch();
                    return !formc.amount;
                  })()}
                >
                  Confirm
                </Button>
              </HStack>
            </VStack>
          </VStack>
        </VStack>
      </form>
    </Layout>
  );
}
