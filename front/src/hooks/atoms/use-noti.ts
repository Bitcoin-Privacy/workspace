import { useToast } from "@chakra-ui/react";
import { useCallback } from "react";

export const useNoti = () => {
  const toast = useToast();
  const success = useCallback(
    (m: string, d: string | undefined = undefined) => {
      toast({
        title: m,
        description: d,
        status: "success",
        position: "top",
        isClosable: true,
        duration: 2000,
      });
    },
    [toast],
  );

  const error = useCallback(
    (m: string, d: string | undefined = undefined) => {
      toast({
        title: m,
        description: d,
        status: "error",
        position: "top",
        isClosable: true,
        duration: 2000,
      });
    },
    [toast],
  );

  return {
    success,
    error,
  };
};
