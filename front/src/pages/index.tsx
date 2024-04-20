"client";

import React, { useMemo } from "react";
import { Text, VStack } from "@chakra-ui/react";
import { useAuthPage } from "@/hooks";
import { Layout, SignIn, SignUp } from "@/components";

function AuthPage() {
  const {
    state: { setPassword },
    method: { onSignin, onSignup },
  } = useAuthPage();

  const authForm = useMemo(() => {
    if (setPassword.get()) return <SignIn onSubmit={onSignin} />;
    else return <SignUp onSubmit={onSignup} />;
  }, [setPassword]);

  return (
    <Layout showHeader={false}>
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
  );
}

export default AuthPage;
