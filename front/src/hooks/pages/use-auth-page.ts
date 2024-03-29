import { WalletApi } from "@/apis";
import { useEffect, useCallback } from "react";
import { useApp } from "..";
import { useRouter } from "next/router";

export const useAuthPage = () => {
  const router = useRouter();
  const { appState } = useApp();

  useEffect(() => {
    if (!appState.logged.get()) return;
    if (appState.setWallet.get()) router.push("/home");
    router.push("/seedphrase");
  }, [appState.logged.get()]);

  const onSignin = useCallback(
    async (pw: string) => {
      const result = await WalletApi.signin(pw);
      if (result) {
        appState.merge({ logged: true });
      } else {
        throw "The password is incorrect";
      }
    },
    [appState.setWallet],
  );

  const onSignup = useCallback(async (pw: string) => {
    await WalletApi.savePassword(pw);
    appState.merge({ logged: true });
  }, []);

  return {
    state: {
      setPassword: appState.setPassword,
    },
    method: { onSignin, onSignup },
  };
};
