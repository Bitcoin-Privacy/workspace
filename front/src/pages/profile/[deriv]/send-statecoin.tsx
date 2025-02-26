import React from "react";
import {
  Text,
  VStack,
  Button,
  Input,
  HStack,
  Modal,
  ModalHeader,
  ModalBody,
  ModalFooter,
  ModalOverlay,
  ModalContent,
  useDisclosure,
} from "@chakra-ui/react";

import { Layout } from "@/components";
import { useSendStateCoinPage } from "@/hooks/pages/use-send-statecoin-page";
import { StatecoinToSendCard } from "@/components/statecoin-to-send-card";
import { useRouter } from "next/router";

const INPUT_WIDTH = "90%";

export default function SendStateCoin() {
  const {
    states: { deriv, form, isLoading, listStatecoinsQuery, isError },
    methods: { handleFormSubmit, setIsError },
  } = useSendStateCoinPage();
  const { isOpen, onOpen, onClose } = useDisclosure();
  const router = useRouter();

  return (
    <Layout header title={"Account " + deriv.slice(0, deriv.indexOf("/"))}>
      <Modal closeOnOverlayClick={false} isOpen={isError} onClose={onClose}>
        <ModalOverlay />
        <ModalContent>
          <ModalHeader>ERROR !!!!</ModalHeader>

          <ModalBody pb={6}>{form.formState.errors.root?.message}</ModalBody>
          <ModalFooter>
            <Button
              colorScheme="red"
              //onClick={onClose}
              onClick={() => {
                setIsError(false);

                onClose();
              }}
            >
              Cancel
            </Button>
          </ModalFooter>
        </ModalContent>
      </Modal>
      <form onSubmit={handleFormSubmit}>
        <VStack textAlign="center" p="0px 16px" spacing="20px">
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
                  })}
                />
              </HStack>

              <HStack>
                <Button
                  p="10px 50px"
                  w="180px"
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
                  w="180px"
                  borderRadius="full"
                  isLoading={isLoading}
                  isDisabled={
                    !form.watch("address") || !form.watch("statechain_id")
                  }
                  type="submit"
                >
                  Send Statecoin
                </Button>
              </HStack>
            </VStack>
          </HStack>
        </VStack>
      </form>
    </Layout>
  );
}
