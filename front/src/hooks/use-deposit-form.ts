import { StatechainApi } from "@/apis";
import { convertBtcToSats } from "@/utils";
import { useMemo, useState } from "react";
import { useForm } from "react-hook-form";

type CreateDepositFormInput = {
  amount: number;
};

export const useDepositForm = (derivationPath: string) => {
  const form = useForm<CreateDepositFormInput>();

  const [isLoading, setIsLoading] = useState<boolean>(false);

  const handleFormSubmit = useMemo(
    () =>
      form.handleSubmit(async (data: CreateDepositFormInput) => {
        setIsLoading(true);
        try {
          console.log("send deposit");
          await StatechainApi.deposit(
            convertBtcToSats(data.amount),
          );

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
      form,
      isLoading,
    },
    methods: {
      handleFormSubmit,
    },
  };
};
