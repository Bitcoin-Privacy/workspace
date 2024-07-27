import { StatechainApi } from "@/apis";
import { useMemo, useState } from "react";
import { useForm } from "react-hook-form";

type SendStatecoinFromInput = {
  address: string;
  statechain_id: string;
};

export const useSendStateCoinForm = (deriv: string) => {
  const form = useForm<SendStatecoinFromInput>();
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [isError, setIsError] = useState<boolean>(false);

  const handleFormSubmit = useMemo(
    () =>
      form.handleSubmit(async (data: SendStatecoinFromInput) => {
        setIsLoading(true);
        try {
          console.log("Send Statecoin form submit API", deriv);
          const res = await StatechainApi.sendStatecoin(
            data.address,
            data.statechain_id,
          );
          console.log("Send Statecoin form submit API response:", res);
        } catch (e: any) {
          console.log("Send Statecoin form submit API error:", e);
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
