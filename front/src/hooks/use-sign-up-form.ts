import { useMemo, useState } from "react";
import { useForm } from "react-hook-form";

type SignUpFormInput = {
  password: string;
  confirmPassword: string;
};

export const useSignUpForm = (onSignup: (pw: string) => Promise<void>) => {
  const form = useForm<SignUpFormInput>();
  const [isLoading, setIsLoading] = useState<boolean>(false);

  const handleFormSubmit = useMemo(
    () =>
      form.handleSubmit(async (data: SignUpFormInput) => {
        setIsLoading(true);
        try {
          if (data.password !== data.confirmPassword) {
            form.setError("confirmPassword", {
              message: "Do not match with password",
            });
            return;
          }
          await onSignup(data.password);
        } catch (e) {
          console.error(e);
          form.setError("password", {
            message: "Failed to set password",
          });
        } finally {
          setIsLoading(false);
        }
      }),
    [],
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
