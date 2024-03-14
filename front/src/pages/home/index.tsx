import React, { useMemo } from "react";
import Head from "next/head";
import { Box, Text, VStack, HStack, Avatar, Circle } from "@chakra-ui/react";
import { useRouter } from "next/router";
import { FiPlus } from "react-icons/fi";
import { Layout } from "@/components";
import { AccountApi } from "@/apis";
import { CachePrefixKeys, DEFAULT_AVATAR } from "@/consts";
import { useQuery } from "react-query";
import { derivBase64 } from "@/utils";

export default function Home() {
  const router = useRouter();

  const listProfilesQuery = useQuery([CachePrefixKeys.Profiles], () =>
    AccountApi.getListAccounts(),
  );

  const listProfiles = useMemo(() => {
    if (listProfilesQuery.data) return listProfilesQuery.data;
    else return [];
  }, [listProfilesQuery.data?.length]);

  return (
    <React.Fragment>
      <Head>
        <title>Home</title>
      </Head>
      <Layout title="Home">
        <VStack textAlign="center" p="10px 16px" h="100%">
          {/* <HStack justify="start" w="100%"> */}
          {/*   <Button */}
          {/*     variant="unstyled" */}
          {/*     color="whiteAlpha.800" */}
          {/*     onClick={() => router.back()} */}
          {/*   > */}
          {/*     Logout */}
          {/*   </Button> */}
          {/* </HStack> */}
          {/* <Text fontSize="30px" fontWeight="800" color="#ddd"> */}
          {/*   List accounts */}
          {/* </Text> */}
          <VStack
            w="100%"
            justify="stretch"
            maxW="500px"
            h="100%"
            overflowY="auto"
          >
            {listProfiles.map((prof, index) => (
              <HStack
                key={index}
                w="100%"
                borderRadius="10px"
                justify="start"
                cursor="pointer"
                p="10px"
                _hover={{ bg: "#aaa5" }}
                onClick={() => {
                  router.push(`/profile/${derivBase64(prof)}`);
                }}
              >
                <Avatar h="36px" w="36px" src={DEFAULT_AVATAR} />
                <Box textAlign="start">
                  <Text color="#fff" fontWeight="600">
                    Account {prof.account_number}
                  </Text>
                  <Text fontSize="12px" fontWeight="400" color="#aaa">
                    Address: {prof.address}
                  </Text>
                </Box>
              </HStack>
            ))}
            <HStack
              w="100%"
              borderRadius="10px"
              justify="start"
              cursor="pointer"
              p="10px"
              _hover={{ bg: "#aaa5" }}
              onClick={() => {
                router.push("/seedphrase");
              }}
            >
              <Circle size="36px" bg="white">
                <FiPlus />
              </Circle>
              <Text color="white">Create new wallet</Text>
            </HStack>
          </VStack>
        </VStack>
      </Layout>
    </React.Fragment>
  );
}
