import { useEffect } from "react";

import { useClipboard } from "@chakra-ui/react";
import { useRouter } from "next/router";

import { AccountApi } from "@/apis";

export const useSeedPhrasePage = () => {
  const router = useRouter();
  useEffect(() => {
    try {
      (async () => {
        const result = await AccountApi.createMasterAccount();
        setMnemonicPhrases(result.join(" "));
      })();
    } catch (e) {
      console.log("Get error", e);
    }
  }, []);

  const {
    onCopy,
    value: mnemonicPhrases,
    setValue: setMnemonicPhrases,
    hasCopied,
  } = useClipboard(" ");

  const onNextBtnClick = () => {
    router.push("/home");
  };

  return {
    states: {
      mnemonicPhrases,
      hasCopied,
    },
    methods: {
      onCopy,
      onNextBtnClick,
    },
  };
};
