import { AppState } from "@/stores";
import { useHookstate } from "@hookstate/core";
import { useEffect } from "react";

export const useInitialize = () => {
  const appState = useHookstate(AppState);
  useEffect(() => {
    appState.merge({ loading: false, ready: true });
  }, []);

  return {
    appState,
  };
};
