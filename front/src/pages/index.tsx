"client";

import React, { useMemo } from "react";
import Head from "next/head";
import { Text, VStack } from "@chakra-ui/react";
import { useAuthPage } from "@/hooks";
import { Layout, SignIn, SignUp } from "@/components";

function AuthPage() {
  const {
    states: { password, state },
    methods: { },
  } = useAuthPage();

  const authForm = useMemo(() => {
    if (password !== null && state != null)
      return <SignIn state={state} password={password} />;
    else return <SignUp />;
  }, [password, state]);

  return (
    <React.Fragment>
      <Head>
        <title>Home</title>
      </Head>
      <Layout>
        <VStack minH="100vh" justify="center" align="center">
          <Text fontSize="30px" fontWeight="800" color="#ddd">
            Bitcoin Wallet
          </Text>
          <Text fontSize="12px" fontWeight="400" m="0 0 15px" color="#aaa">
            The most Privacy
          </Text>
          {authForm}
        </VStack>
      </Layout>
    </React.Fragment>
  );
}

export default AuthPage;
