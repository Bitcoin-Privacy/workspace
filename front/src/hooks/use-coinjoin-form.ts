import { useMemo, useState } from "react";
import { useForm } from "react-hook-form";

import { AppApi, CoinJoinApi } from "@/apis";
import { TxStrategyEnum } from "@/dtos";
import { convertBtcToSats as convertBtcToSat } from "@/utils";

type CreateTxFormInput = {
  address: string;
  amount: number;
  strategy: TxStrategyEnum;
};

export const useCreateTxForm = (derivationPath: string) => {
  const form = useForm<CreateTxFormInput>();
  const [isLoading, setIsLoading] = useState<boolean>(false);

  const handleFormSubmit = useMemo(
    () =>
      form.handleSubmit(async (data: CreateTxFormInput) => {
        setIsLoading(true);
        try {
          switch (data.strategy) {
            case TxStrategyEnum.Base:
              await AppApi.createTxn(
                derivationPath,
                data.address,
                convertBtcToSat(data.amount),
              );
              break;
            case TxStrategyEnum.CoinJoin:
              await CoinJoinApi.register(
                derivationPath,
                data.address,
                convertBtcToSat(data.amount),
              );
              break;
            default:
              throw "Not support yet";
          }
          form.reset({ address: "", amount: 0, strategy: TxStrategyEnum.Base });
        } catch (e) {
        } finally {
          setIsLoading(false);
        }
      }),
    [derivationPath, form],
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
