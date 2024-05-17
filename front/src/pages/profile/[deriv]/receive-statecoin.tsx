import { StatechainApi } from "@/apis";
import { Layout, NavBar } from "@/components";
import QRCodeGenerator from "@/components/qr-code-generator";
import { useDeriv } from "@/hooks";

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
import React, { useState } from "react";
import { FiCheck, FiCopy } from "react-icons/fi";

export default function ReceiveStatecoin() {
  const router = useRouter();
  const { deriv } = useDeriv();
  const { onCopy, value, setValue, hasCopied } = useClipboard("");
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [isDisable, setIsDisable] = useState<boolean>(false);
  return (
    <React.Fragment>
      <Head>Receive Statecoin</Head>
      <Layout>
        <NavBar title={"Receive statecoin "} />
        <VStack p="8px 16px" color="white" spacing="60px">
          <Text fontWeight="700" fontSize="24px">
            Generate statechain address to receive
          </Text>
          {value && (
            <HStack spacing="8px" h="20px">
              <QRCodeGenerator size="100px" text={value} />
              <Button
                onClick={onCopy}
                bgColor={"cyan.100"}
                rightIcon={hasCopied ? <FiCheck /> : <FiCopy />}
                borderRadius={"8"}
                p="2px 8px"
              >
                <Text isTruncated maxW={"320px"}>
                  {value}
                </Text>
              </Button>
            </HStack>
          )}
          
          <Button
            isLoading={isLoading}
            isDisabled={isDisable}
            onClick={async () => {
              setIsLoading(true);
              let address = await StatechainApi.genStatechainAddress(deriv);
              console.log("address", address);
              setValue(() => address);
              setIsLoading(false);
              setIsDisable(true);
            }}
            color="white"
            bg="gray.700"
          >
            Generate Keys
          </Button>
        </VStack>
      </Layout>
    </React.Fragment>
  );
}
