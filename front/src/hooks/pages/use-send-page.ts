import { useQuery } from "react-query";
import { useCreateTxForm, useDeriv } from "@/hooks";

import { AppApi } from "@/apis";
import { CachePrefixKeys } from "@/consts";

export const useSendPage = () => {
  const { deriv } = useDeriv();

  const profQuery = useQuery([CachePrefixKeys.ProfileFromDeriv, deriv], () =>
    AppApi.getAccount(deriv),
  );

  const balanceQuery = useQuery(
    [CachePrefixKeys.Balance, profQuery.data?.address],
    () => AppApi.getBalance(profQuery.data!.address),
    {
      enabled: !!profQuery.data?.address,
    },
  );

  const {
    states: { form, isLoading },
    methods: { handleFormSubmit },
  } = useCreateTxForm(deriv);

  return {
    states: {
      deriv,
      form,
      isLoading,
      profQuery,
      balanceQuery,
    },
    methods: { handleFormSubmit },
  };
};
