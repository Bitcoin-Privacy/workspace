import { Layout, NavBar } from "@/components";
import {
  Button,
  HStack,
  Modal,
  ModalBody,
  ModalCloseButton,
  ModalContent,
  ModalFooter,
  ModalHeader,
  ModalOverlay,
  Text,
  useClipboard,
  useDisclosure,
  VStack,
} from "@chakra-ui/react";
import Head from "next/head";
import { useRouter } from "next/router";
import React from "react";
import { FiCheck, FiCopy } from "react-icons/fi";

export default function ReceiveStatecoin() {
  const router = useRouter();
  const { isOpen, onOpen, onClose } = useDisclosure();

  const { onCopy, value, setValue, hasCopied } = useClipboard("");
  return (
    <React.Fragment>
      <Head>Receive Statecoin</Head>
      <Layout>
        <VStack p="0px 16px" color="white" spacing="16px">
          <NavBar title={"Account "} />
          <Text fontWeight="700" fontSize="18px">
            Receive Statecoin
          </Text>

          <Text> Press the button below to generate the keys</Text>
          <Button onClick={onOpen} color="white" bg="gray.700">
            Generate Keys
          </Button>
          <HStack spacing="8px">
            <Text>Owner key: </Text>
            <Button
              onClick={onCopy}
              bgColor={"cyan.100"}
              rightIcon={hasCopied ? <FiCheck /> : <FiCopy />}
              borderRadius={"8"}
              p="2px 8px"
            >
              <Text isTruncated maxW={"160px"}>
                asdfasdfasdfasdfasdfasdfadfasdf
              </Text>
            </Button>
          </HStack>

          <HStack spacing="8px">
            <Text>Authen key:</Text>
            <Button
              onClick={onCopy}
              bgColor={"cyan.100"}
              rightIcon={hasCopied ? <FiCheck /> : <FiCopy />}
              borderRadius={"8"}
              p="2px 8px"
            >
              <Text isTruncated maxW={"160px"}>
                asdfasdfasdfasdfasdfasdfadfasdf
              </Text>
            </Button>
          </HStack>
        </VStack>

        <Modal isOpen={isOpen} onClose={onClose}>
          <ModalOverlay />
          <ModalContent>
            <ModalHeader>Keys are successfully generated</ModalHeader>
            <ModalCloseButton />
            <ModalBody>
              <Text>
                Please send those keys to the whom you want to receive the
                statecoin{" "}
              </Text>
            </ModalBody>

            <ModalFooter>
              <Button colorScheme="blue" mr={3} onClick={onClose}>
                Close
              </Button>
            </ModalFooter>
          </ModalContent>
        </Modal>
      </Layout>
    </React.Fragment>
  );
}
