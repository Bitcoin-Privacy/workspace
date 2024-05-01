import { CoinJoinApi, StatechainApi } from "@/apis";
import { TxStrategyEnum } from "@/dtos";
import { convertBtcToSats as convertBtcToSat } from "@/utils";
import { useMemo, useState } from "react";
import { useForm } from "react-hook-form";


type SendStatecoinFromInput = {
    o2_pubkey : string;
    o2_authkey : string;
    statechain_id : string;
}

export const useSendStateCoinForm = (derivationPath: string) => {
  const form = useForm<SendStatecoinFromInput>();
  const [isLoading, setIsLoading] = useState<boolean>(false);

  const handleFormSubmit = useMemo(
    () =>
      form.handleSubmit(async (data: SendStatecoinFromInput) => {
        setIsLoading(true);
        try {
         
          const res = await StatechainApi.sendStatecoin(
            data.o2_pubkey,
            data.o2_authkey,
            data.statechain_id
          );
          console.log("send statecoin ", res);
        
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
