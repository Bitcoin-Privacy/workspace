import React, { useCallback, useState } from "react";
import {
  Text,
  VStack,
  Button,
  HStack,
  Grid,
  GridItem,
  Input,
  Box,
} from "@chakra-ui/react";

import { Layout } from "@/components";
import { AppApi } from "@/apis";
import { useRouter } from "next/router";
import { useNoti } from "@/hooks";

export default function ImportSeedPhrase() {
  const router = useRouter();
  const [input, setInput] = useState<string>("");
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const noti = useNoti();
  const onNextBtnClick = useCallback(
    async (seedphrase: string) => {
      try {
        setIsLoading(true);
        await AppApi.importMaster(seedphrase.split(" "));
        router.replace("/home");
      } catch (e) {
        noti.error("Got an error", e as string);
      } finally {
        setIsLoading(false);
      }
    },
    [router, setIsLoading, noti],
  );

  return (
    <Layout title="Import Seed Phrase" header>
      <VStack p="30px 16px" textColor="whiteAlpha.800">
        <Box w="650px">
          <Text>Paste your seed phrase here:</Text>
          <Input value={input} onChange={(e) => setInput(e.target.value)} />
        </Box>
        {input.split(" ")?.length && (
          <>
            <Grid
              borderWidth={"1px"}
              borderRadius={"16px"}
              templateColumns="repeat(4, 1fr)"
              gap={6}
              p={"16px"}
            >
              {input.split(" ").map((word, id) => {
                return (
                  <GridItem key={id}>
                    <HStack
                      borderWidth={"1px"}
                      width={"120px"}
                      p={"6px"}
                      m={"4px"}
                      alignItems={"center"}
                    >
                      <Text>{id + 1}.</Text>
                      <Text> {word}</Text>
                    </HStack>
                  </GridItem>
                );
              })}
            </Grid>

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
                isLoading={isLoading}
                isDisabled={(() => {
                  return input.split(" ").length !== 12;
                })()}
                onClick={() => onNextBtnClick(input)}
              >
                Finish
              </Button>
            </HStack>
          </>
        )}
      </VStack>
    </Layout>
  );
}
