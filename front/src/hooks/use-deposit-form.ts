import { StatechainApi } from "@/apis";
import { StatechainDepositResDto } from "@/dtos";
import { convertBtcToSats, profilePath } from "@/utils";
import { useMemo, useState } from "react";
import { useForm } from "react-hook-form";
import { useNoti } from ".";
import { useRouter } from "next/router";

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

  const noti = useNoti();
  const router = useRouter();

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
          noti.success(
            "Deposit successfully",
            `Sent ${data.amount} BTC to the multisig address between you and SE`,
          );
          router.replace(profilePath(deriv, "?tab=STATECHAIN"));
        } catch (e: any) {
          console.log("Desposit form submit API error:", e);
          form.setError("root", {
            message: e,
          });
          setIsError(true);
          noti.error("Got an error", e);
        } finally {
          setIsLoading(false);
        }
      }),
    [deriv, form, noti, router],
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
