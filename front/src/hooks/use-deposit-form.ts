import { StatechainApi } from "@/apis";
import { StatechainDepositResDto } from "@/dtos";
import { convertBtcToSats } from "@/utils";
import { useMemo, useState } from "react";
import { useForm } from "react-hook-form";

type CreateDepositFormInput = {
  amount: number;
};

export const  useDepositForm = (derivationPath: string) => {
  const form = useForm<CreateDepositFormInput>({
    criteriaMode: "all",
  });

  const [depositInfo,setDepositInfo] = useState<StatechainDepositResDto>();
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [isError,setIsError] = useState<boolean>(false);

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
          // get the aggregated address
          console.log("api response nee",res);
          setDepositInfo(res);
          form.reset({ amount: 0 });
        } catch (e :any ){
          console.log("api response error", e );
          form.setError('root', {
            message: e
          });
          setIsError(true)
        } finally {
          setIsLoading(false);
        }
      }),
    [derivationPath],
  );

  return {
    states: {
      depositInfo,
      form,
      isLoading,
      isError
    },
    methods: {
      handleFormSubmit,
      setIsError
    },
  };
};
