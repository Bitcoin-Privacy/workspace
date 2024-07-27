import { StatechainApi } from "@/apis";
import { StatechainDepositResDto } from "@/dtos";
import { convertBtcToSats } from "@/utils";
import { useMemo, useState } from "react";
import { useForm } from "react-hook-form";

type CreateDepositFormInput = {
  amount: number;
};

export const useDepositForm = (deriv: string) => {
  const form = useForm<CreateDepositFormInput>({
    defaultValues: { amount: 0 },
    criteriaMode: "all",
  });

  const [depositInfo, setDepositInfo] = useState<StatechainDepositResDto>();

  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [isError, setIsError] = useState<boolean>(false);

  const handleFormSubmit = useMemo(
    () =>
      form.handleSubmit(async (data: CreateDepositFormInput) => {
        setIsLoading(true);
        try {
          console.log("Desposit form submit API");
          const res = await StatechainApi.deposit(
            deriv,
            convertBtcToSats(data.amount),
          );
          // get the aggregated address
          console.log("Desposit form submit API response:", res);
          setDepositInfo(res);
          form.reset({ amount: 0 });
        } catch (e: any) {
          console.log("Desposit form submit API error:", e);
          form.setError("root", {
            message: e,
          });
          setIsError(true);
        } finally {
          setIsLoading(false);
        }
      }),
    [deriv, form],
  );

  return {
    states: {
      depositInfo,
      form,
      isLoading,
      isError,
    },
    methods: {
      handleFormSubmit,
      setIsError,
    },
  };
};
