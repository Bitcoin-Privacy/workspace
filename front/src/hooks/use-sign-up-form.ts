import { useRouter } from "next/router";
import { useMemo, useState } from "react";
import { useForm } from "react-hook-form";
import { WalletApi } from "@/apis";

type SignUpFormInput = {
  password: string;
  confirmPassword: string;
};

export const useSignUpForm = () => {
  const form = useForm<SignUpFormInput>();
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const router = useRouter();

  const handleFormSubmit = useMemo(
    () =>
      form.handleSubmit(async (data: SignUpFormInput) => {
        setIsLoading(true);
        if (data.password !== data.confirmPassword) {
          form.setError("confirmPassword", {
            message: "Do not match with password",
          });
          return;
        }

        await WalletApi.savePassword(data.password);

        router.push("seedphrase");
        setIsLoading(false);
      }),
    [],
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
