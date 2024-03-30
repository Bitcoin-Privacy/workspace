import { useEffect } from "react";

import { useClipboard } from "@chakra-ui/react";
import { useRouter } from "next/router";

import { AppApi } from "@/apis";

export const useSeedPhrasePage = () => {
  const router = useRouter();
  useEffect(() => {
    try {
      (async () => {
        const result = await AppApi.createMaster();
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
