import { WalletApi } from "@/apis";
import { InitStateEnum } from "@/dtos";
import { AppState } from "@/stores";
import { useHookstate } from "@hookstate/core";
import { useRouter } from "next/router";
import { useEffect } from "react";

export const useInit = () => {
  const router = useRouter();
  const appState = useHookstate(AppState);

  useEffect(() => {
    (async () => {
      const initState = await WalletApi.getInitState();
      appState.merge({
        loading: false,
        ready: true,
        setPassword: initState.type !== InitStateEnum.BrandNew,
        setWallet: initState.type === InitStateEnum.CreatedWallet,
      });
    })();
  }, []);

  useEffect(() => {
    console.log("AUTO REDIRECT EXEC", appState);
    if (appState.logged.get()) return;
    const path = router.pathname;
    console.log("AUTO REDIRECT", path, appState);
    if (!["", "/"].includes(path)) {
      router.push("/");
    }
  }, [appState.logged, router.pathname]);

  return {
    appState,
  };
};
