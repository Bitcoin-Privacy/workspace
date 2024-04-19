import { CoinJoinApi } from "@/apis";
import { TxStrategyEnum } from "@/dtos";
import { convertBtcToSats as convertBtcToSat } from "@/utils";
import { useMemo, useState } from "react";
import { useForm } from "react-hook-form";


type SendStatecoinFromInput = {
    o2_address : string;
    o2_authkey : string;
}

export const useSendStateCoinForm = (derivationPath: string) => {
  const form = useForm<SendStatecoinFromInput>();
  const [isLoading, setIsLoading] = useState<boolean>(false);

  const handleFormSubmit = useMemo(
    () =>
      form.handleSubmit(async (data: SendStatecoinFromInput) => {
        setIsLoading(true);
        try {
         
        
        } catch (e) {
        } finally {
          setIsLoading(false);
        }
      }),
    [derivationPath],
  );

  return {
    states: {
      form,
      isLoading,
    },
    methods: {
      handleFormSubmit,
    },
  };
};
