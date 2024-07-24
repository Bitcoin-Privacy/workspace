import React from "react";
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
  Stack,
  Spinner,
  Center,
} from "@chakra-ui/react";
import { MdCheckCircle } from "react-icons/md";
import { TiTick } from "react-icons/ti";
import { BiCopy } from "react-icons/bi";

import { Layout } from "@/components";
import { useSeedPhrasePage } from "@/hooks";

export default function SeedPhrase() {
  const {
    states: { mnemonicPhrases, hasCopied },
    methods: { onCopy, onNextBtnClick },
  } = useSeedPhrasePage();

  return (
    <Layout title="Generate Seed Phrases">
      <VStack p="30px 16px" textColor={"whiteAlpha.800"}>
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
            <Button onClick={onNextBtnClick}>Next</Button>
          </>
        )}
      </VStack>
    </Layout>
  );
}
