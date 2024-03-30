import { useCallback } from "react";
import { useApp } from ".";
import { useRouter } from "next/router";

export const useLogout = () => {
  const router = useRouter();
  const { appState } = useApp();

  const logout = useCallback(() => {
    appState.merge({ logged: false });
  }, []);

  return {
    method: { logout },
  };
};
