import { useQuery } from "react-query";
import { useDeriv } from "@/hooks";

import { AppApi } from "@/apis";
import { CachePrefixKeys } from "@/consts";
import { useDepositForm } from "../use-deposit-form";

export const useDepositPage = () => {
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
    states: {  depositInfo, form, isLoading,  isError },
    methods: { handleFormSubmit ,setIsError},
  } = useDepositForm(deriv);

  return {
    states: {
      depositInfo,
      deriv,
      form,
      isLoading,
      isError,
      profQuery,
      balanceQuery,
    },
    methods: { handleFormSubmit ,setIsError},
  };
};
