import { StatechainApi } from "@/apis";
import { useMemo, useState } from "react";
import { useForm } from "react-hook-form";
import { useNoti } from "./atoms";
import { useRouter } from "next/router";
import { profilePath } from "@/utils";

type SendStatecoinFromInput = {
  address: string;
  statechain_id: string;
};

export const useSendStateCoinForm = (deriv: string) => {
  const form = useForm<SendStatecoinFromInput>();
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [isError, setIsError] = useState<boolean>(false);
  const noti = useNoti();
  const router = useRouter();

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
          noti.success("Send successfully!");
          router.replace(profilePath(deriv, "?tab=STATECHAIN"));
        } catch (e: any) {
          console.log("Send Statecoin form submit API error:", e);
          noti.error("Got an error!", e);
        } finally {
          setIsLoading(false);
        }
      }),
    [deriv, form, noti, router],
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
