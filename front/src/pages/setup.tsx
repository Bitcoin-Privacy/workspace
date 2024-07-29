import React from "react";
import { Text, VStack, Button, Box } from "@chakra-ui/react";

import { Layout } from "@/components";
import { useRouter } from "next/router";

export default function Setup() {
  const router = useRouter();

  return (
    <Layout title="Setup your wallet" header>
      <VStack p="30px 16px" textColor="whiteAlpha.800" spacing="8px">
        <Text fontSize="24px">Create or restore your wallet!</Text>
        <Box textAlign="left" w="300px" fontSize="14px">
          <Text>
            - &ldquo;Create a new wallet&rdquo; option will generate a new seed
            phrase
          </Text>
          <Text>
            - &ldquo;Restore existing wallet&rdquo; option will let you to
            import your seed phrase
          </Text>
        </Box>
        <Button w="300px" onClick={() => router.push("/seedphrase/generate")}>
          Create a new wallet
        </Button>
        <Button w="300px" onClick={() => router.push("/seedphrase/import")}>
          Restore existing wallet
        </Button>
      </VStack>
    </Layout>
  );
}
