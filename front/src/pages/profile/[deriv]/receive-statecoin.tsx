import { StatechainApi } from "@/apis";
import { Layout } from "@/components";
import QRCodeGenerator from "@/components/qr-code-generator";
import { useDeriv } from "@/hooks";

import { Button, HStack, Text, useClipboard, VStack } from "@chakra-ui/react";
import { useRouter } from "next/router";
import React, { useState } from "react";
import { FiCheck, FiCopy } from "react-icons/fi";

export default function ReceiveStatecoin() {
  const { deriv } = useDeriv();
  const router = useRouter();
  const { onCopy, value, setValue, hasCopied } = useClipboard("");
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [isDisable, setIsDisable] = useState<boolean>(false);
  return (
    <Layout header title={"Account " + deriv.slice(0, deriv.indexOf("/"))}>
      <VStack p="8px 16px" color="white" spacing="60px">
        <Text fontWeight="700" fontSize="24px">
          Generate statechain address to receive statecoin
        </Text>
        {value && (
          <HStack spacing="8px" h="20px">
            <QRCodeGenerator size="100px" text={value} />
            <Button
              onClick={onCopy}
              bgColor="cyan.100"
              rightIcon={hasCopied ? <FiCheck /> : <FiCopy />}
              borderRadius="8"
              p="2px 8px"
            >
              <Text isTruncated maxW="320px">
                {value}
              </Text>
            </Button>
          </HStack>
        )}

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
            isDisabled={isDisable}
            onClick={async () => {
              setIsLoading(true);
              let address = await StatechainApi.genStatechainAddress(deriv);
              console.log("address", address);
              setValue(() => address);
              setIsLoading(false);
              setIsDisable(true);
            }}
          >
            Generate Keys
          </Button>
        </HStack>
      </VStack>
    </Layout>
  );
}
