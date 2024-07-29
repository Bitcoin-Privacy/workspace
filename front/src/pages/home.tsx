import React, { useMemo } from "react";
import { Box, Text, VStack, HStack, Avatar } from "@chakra-ui/react";
import { useRouter } from "next/router";
import { useQuery } from "react-query";

import { Layout } from "@/components";
import { AppApi } from "@/apis";
import { CachePrefixKeys } from "@/consts";
import { derivBase64 } from "@/utils";

export default function Home() {
  const router = useRouter();

  const listProfilesQuery = useQuery([CachePrefixKeys.Profiles], () =>
    AppApi.getAccounts(),
  );

  const listProfiles = useMemo(() => {
    if (listProfilesQuery.data) return listProfilesQuery.data;
    else return [];
  }, [listProfilesQuery.data?.length]);

  return (
    <Layout header title="Home">
      <VStack textAlign="center" p="10px 16px" h="100%">
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
              <Avatar h="42px" w="42px" src="/avatar.jpeg" />

              <Box textAlign="start">
                <Text color="#fff" fontWeight="600" fontSize="16px">
                  Account {prof.account_number}
                </Text>
                <Text fontSize="14px" fontWeight="400" color="#aaa">
                  Address: {prof.address}
                </Text>
              </Box>
            </HStack>
          ))}
          {/* <HStack */}
          {/*   w="100%" */}
          {/*   borderRadius="10px" */}
          {/*   justify="start" */}
          {/*   cursor="pointer" */}
          {/*   p="10px" */}
          {/*   _hover={{ bg: "#aaa5" }} */}
          {/*   onClick={() => { */}
          {/*     router.push("/seedphrase"); */}
          {/*   }} */}
          {/* > */}
          {/*   <Circle size="36px" bg="white"> */}
          {/*     <FiPlus /> */}
          {/*   </Circle> */}
          {/*   <Text color="white">Create new wallet</Text> */}
          {/* </HStack> */}
        </VStack>
      </VStack>
    </Layout>
  );
}
