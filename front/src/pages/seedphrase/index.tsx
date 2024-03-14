import React, { useEffect } from "react";
import Head from "next/head";
import {
  Text,
  VStack,
  Button,
  HStack,
  List,
  ListItem,
  ListIcon,
  Grid,
  GridItem,
  useClipboard,
  Stack,
  Spinner,
  Center,
} from "@chakra-ui/react";
import { Layout } from "@/components/layout";
import { useRouter } from "next/router";
import { FiArrowLeft } from "react-icons/fi";
import { MdCheckCircle } from "react-icons/md";
import { TiTick } from "react-icons/ti";
import { BiCopy } from "react-icons/bi";
import { AccountApi } from "@/apis";

export default function SeedPhrase() {
  const router = useRouter();
  useEffect(() => {
    try {
      (async () => {
        const result = await AccountApi.createMasterAccount();
        setMnemonicPhrases(result.join(" "));
      })();
    } catch (e) {
      console.log("Get error", e);
    }
  }, []);

  const {
    onCopy,
    value: mnemonicPhrases,
    setValue: setMnemonicPhrases,
    hasCopied,
  } = useClipboard(" ");
  return (
    <React.Fragment>
      <Head>
        <title>Generate Seed Phrases</title>
      </Head>
      <Layout>
        <VStack p="30px 16px" textColor={"whiteAlpha.800"}>
          <HStack justify="start" w="100%" spacing={"40"}>
            <Button
              variant="unstyled"
              leftIcon={<FiArrowLeft />}
              onClick={() => router.back()}
            >
              Back
            </Button>

            <Text>Write down your secret recovery phrase</Text>
          </HStack>

          <Stack justify="start">
            <List spacing={2}>
              <Text>Tips</Text>
              <ListItem>
                <ListIcon as={MdCheckCircle} color="green.500" />
                Save in password manager
              </ListItem>
              <ListItem>
                <ListIcon as={MdCheckCircle} color="green.500" />
                Store in a safe deposit box
              </ListItem>
              <ListItem>
                <ListIcon as={MdCheckCircle} color="green.500" />
                Write down and store in multiple secret places
              </ListItem>
            </List>
          </Stack>
          {!mnemonicPhrases?.length && (
            <Center h="100%" w="100%">
              <Spinner />
            </Center>
          )}
          {mnemonicPhrases?.length && (
            <>
              <Grid
                borderWidth={"1px"}
                borderRadius={"16px"}
                templateColumns="repeat(4, 1fr)"
                gap={6}
                p={"16px"}
              >
                {mnemonicPhrases.split(" ").map((word, id) => {
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

              <Button
                onClick={onCopy}
                bgColor={"cyan.100"}
                rightIcon={hasCopied ? <TiTick /> : <BiCopy />}
                borderRadius={"16"}
              >
                <Text> Copy to clipboard</Text>
              </Button>
              <Button
                onClick={async () => {
                  router.push("home");
                }}
              >
                Next
              </Button>
            </>
          )}
        </VStack>
      </Layout>
    </React.Fragment>
  );
}
