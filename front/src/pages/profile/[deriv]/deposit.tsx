import React, { useState } from "react";
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
  Center,
  useDisclosure,
  Modal,
  ModalOverlay,
  ModalContent,
} from "@chakra-ui/react";
import { useRouter } from "next/router";
import { FaLongArrowAltRight } from "react-icons/fa";
import { Layout, NavBar } from "@/components";
import { useDepositPage } from "@/hooks/pages/use-deposit-page";
import QRCodeGenerator from "@/components/qr-code-generator";

const INPUT_WIDTH = "75%";

export default function Deposit() {
  const router = useRouter();
  const { isOpen, onOpen, onClose } = useDisclosure();

  const {
    states: { depositInfo, deriv, form, isLoading, balanceQuery },
    methods: { handleFormSubmit },
  } = useDepositPage();

  const [amount, setAmount] = useState<number>(0);

  return (
    <React.Fragment>
      <Modal blockScrollOnMount={true} isOpen={isOpen} onClose={onClose}>
        <ModalOverlay />
        <ModalContent>
          <Text>HIHIHIHIHIH</Text>
        </ModalContent>
      </Modal>
      <Layout>
        <form onSubmit={handleFormSubmit}>
          <VStack p="0px 16px" spacing="20px">
            <HStack justify="start" w="100%">
              <NavBar title={"Account " + deriv.slice(0, deriv.indexOf("/"))} />
            </HStack>
            <Text color="white" fontWeight="700" fontSize="18px">
              Deposit statecoin
            </Text>
            <VStack width="100%" maxW="500px" spacing="16px">
              <FormControl isInvalid={!!form.formState.errors.amount}>
                <VStack spacing="24px">
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
                            value: balanceQuery.data
                              ? balanceQuery.data / 10000000
                              : Number.MAX_VALUE,
                            message: "Balance is not enough",
                          },
                          min: {
                            value: 0.00000001,
                            message:
                              "Amount must larget than or equal to 1 sat",
                          },
                        })}
                      />
                      <InputRightAddon w="82px" justifyContent="center">
                        BTC
                      </InputRightAddon>
                    </InputGroup>
                  </HStack>
                  <Button
                    type="submit"
                    onClick={() => {
                      onOpen;
                      setAmount(form.getValues("amount"));
                      onClose;
                    }}
                    isLoading={isLoading}
                    isDisabled={(() => {
                      let formc = form.watch();
                      return !formc.amount;
                    })()}
                  >
                    Confirm
                  </Button>
                  {form.formState.errors.amount && (
                    <FormErrorMessage justifyContent="end">
                      {form.formState.errors.amount.message}
                    </FormErrorMessage>
                  )}
                </VStack>
              </FormControl>
            </VStack>
            {depositInfo && (
              <VStack
                bg={"gray.900"}
                borderRadius={"8px"}
                p="20px 40px"
                w="full"
                spacing="24px"
                color={"white"}
              >
                <HStack w="full" alignItems={"end"}>
                  <QRCodeGenerator
                    text={depositInfo.aggregated_address}
                    size="100px"
                  />

                  <VStack
                    w="full"
                    alignItems={"center"}
                    p="px 16px"
                    spacing="16px"
                  >
                    <Text>
                      {" "}
                      The address below is the Multisig Address between you and
                      SE
                    </Text>
                    <HStack spacing="8px" p="0px 8px">
                      <Center
                        w="30%"
                        borderRadius={"16"}
                        bg="gray.700"
                        p="10px 15px"
                      >
                        {" "}
                        {amount} BTC
                      </Center>
                      <Center>
                        <FaLongArrowAltRight size="40px" />
                      </Center>

                      <Button
                        //onClick={onCopy}
                        bgColor={"gray.700"}
                        //rightIcon={hasCopied ? <FiCheck /> : <FiCopy />}
                        borderRadius={"16"}
                      >
                        <Text
                          color={"white"}
                          isTruncated
                          maxW={"200px"}
                          p="5px"
                        >
                          {depositInfo.aggregated_address}
                        </Text>
                      </Button>
                    </HStack>
                  </VStack>
                </HStack>
                <Button
                  onClick={() => {
                    router.back();
                  }}
                >
                  Close
                </Button>
              </VStack>
            )}
          </VStack>
        </form>
      </Layout>
    </React.Fragment>
  );
}
