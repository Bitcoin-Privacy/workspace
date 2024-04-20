"client";

import React, { useMemo } from "react";
import { Center, Text } from "@chakra-ui/react";
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
  }, [setPassword, onSignin, onSignup]);

  return (
    <Layout showHeader={false}>
      <Center flexDir="column" h="100%">
        <Text fontSize="30px" fontWeight="800" color="#ddd">
          Bitcoin Wallet
        </Text>
        <Text fontSize="14px" fontWeight="500" m="0 0 15px" color="#aaa">
          The most Privacy
        </Text>
        {authForm}
      </Center>
    </Layout>
  );
}

export default AuthPage;
