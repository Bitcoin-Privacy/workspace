import { useMemo, useState } from "react";
import { useForm } from "react-hook-form";

type LoginFormInput = {
  password: string;
};

export const useLoginForm = (onSubmit: (pw: string) => Promise<void>) => {
  const form = useForm<LoginFormInput>();
  const [isLoading, setIsLoading] = useState<boolean>(false);

  const handleFormSubmit = useMemo(
    () =>
      form.handleSubmit(async (data) => {
        setIsLoading(true);
        try {
          await onSubmit(data.password);
        } catch (e) {
          if (typeof e === "string")
            form.setError("password", {
              message: e,
            });
          else
            form.setError("password", {
              message: "Failed to submit password",
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
