import { StatechainApi } from "@/apis";
import { convertBtcToSats } from "@/utils";
import { useMemo, useState } from "react";
import { useForm } from "react-hook-form";

type CreateDepositFormInput = {
  amount: number;
};

export const useDepositForm = (derivationPath: string) => {
  const form = useForm<CreateDepositFormInput>();

  const [aggAddress,setAggAddress] = useState<string>("");

  const [isLoading, setIsLoading] = useState<boolean>(false);

  const handleFormSubmit = useMemo(
    () =>
      form.handleSubmit(async (data: CreateDepositFormInput) => {
        setIsLoading(true);
        try {
          console.log("send deposit");
          const res = await StatechainApi.deposit(
            derivationPath,
            convertBtcToSats(data.amount),
          );
          console.log("api response ",res);
          setAggAddress(res.aggregated_address)
          const deposit_tx = await StatechainApi.createDepositTx(derivationPath, convertBtcToSats(data.amount),res.aggregated_address);
          console.log("deposit tx: ", deposit_tx);
          form.reset({ amount: 0 });
        } catch (e) {
        } finally {
          setIsLoading(false);
        }
      }),
    [derivationPath],
  );

  return {
    states: {
      aggAddress,
      form,
      isLoading,
    },
    methods: {
      handleFormSubmit,
    },
  };
};
