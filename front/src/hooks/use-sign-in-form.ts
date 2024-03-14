import { InitStateEnum } from "@/dtos";
import { useRouter } from "next/router";
import { useMemo, useState } from "react";
import { useForm } from "react-hook-form";

type LoginFormInput = {
  password: string;
};

export const useLoginForm = (password: string, state: InitStateEnum) => {
  const form = useForm<LoginFormInput>();
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const router = useRouter();

  const handleFormSubmit = useMemo(
    () =>
      form.handleSubmit(async (data) => {
        console.log(data, password, state);
        setIsLoading(true);
        if (data.password === password) {
          if (state === InitStateEnum.CreatedPassword)
            router.push("/seedphrase");
          else router.push("/home");
        } else {
          form.setError("password", { message: "Incorrect password" });
        }
        setIsLoading(false);
      }),
    [state],
  );
  return {
    states: {
      form,
      isHandling: isLoading,
    },
    methods: {
      handleFormSubmit,
    },
  };
};
