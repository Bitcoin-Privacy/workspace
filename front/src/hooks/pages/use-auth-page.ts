import { WalletApi } from "@/apis";
import { InitStateEnum } from "@/dtos";
import { useEffect, useState } from "react";

export const useAuthPage = () => {
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [password, setPassword] = useState<string | null>(null);
  const [state, setState] = useState<InitStateEnum | null>(null);

  useEffect(() => {
    (async () => {
      try {
        const initState = await WalletApi.getInitState();
        setPassword(initState.password ?? null);
        setState(initState.type);
      } catch (e) {
        console.log("Get init state error:", e);
      } finally {
        setIsLoading(false);
      }
    })();
  }, []);

  return {
    states: {
      isLoading,
      password,
      state,
    },
    methods: {},
  };
};
