import { AppState } from "@/stores";
import { useHookstate } from "@hookstate/core";

export const useApp = () => {
  const appState = useHookstate(AppState);

  return {
    appState,
  };
};
