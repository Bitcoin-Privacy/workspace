import { AppApi, CoinJoinApi } from "@/apis";
import { TxStrategyEnum } from "@/dtos";
import { convertBtcToSats as convertBtcToSat, profilePath } from "@/utils";
import { useRouter } from "next/router";
import { useMemo, useState } from "react";
import { useForm } from "react-hook-form";

type CreateTxFormInput = {
  address: string;
  amount: number;
  strategy: TxStrategyEnum;
};

export const useCreateTxForm = (deriv: string) => {
  const form = useForm<CreateTxFormInput>({ defaultValues: { amount: 0 } });
  const router = useRouter();

  const [isLoading, setIsLoading] = useState<boolean>(false);

  const handleFormSubmit = useMemo(
    () =>
      form.handleSubmit(async (data: CreateTxFormInput) => {
        setIsLoading(true);
        try {
          switch (data.strategy) {
            case TxStrategyEnum.Base:
              await AppApi.createTxn(
                deriv,
                data.address,
                convertBtcToSat(data.amount),
              );
              break;
            case TxStrategyEnum.CoinJoin:
              await CoinJoinApi.register(
                deriv,
                data.address,
                convertBtcToSat(data.amount),
              );
              router.replace(profilePath(deriv, "?tab=COINJOIN"));
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
    [deriv, form, router],
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
