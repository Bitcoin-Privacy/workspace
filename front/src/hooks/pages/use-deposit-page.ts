import { useQuery } from "react-query";
import { useDeriv } from "@/hooks";

import { AccountApi } from "@/apis";
import { CachePrefixKeys } from "@/consts";
import { useDepositForm } from "../use-deposit-form";

export const useDepositPage = () => {
    const {deriv} = useDeriv();
    const profQuery = useQuery([CachePrefixKeys.ProfileFromDeriv, deriv], () =>
    AccountApi.getAccount(deriv),
  );

    const balanceQuery = useQuery(
        [CachePrefixKeys.Balance, profQuery.data?.address],
        () => AccountApi.getBalance(profQuery.data!.address),
        {
        enabled: !!profQuery.data?.address,
        },
    );

    const {
        states: { form, isLoading },
        methods: { handleFormSubmit },
      } = useDepositForm(deriv);

    return {
        states: {
          deriv, form, isLoading, profQuery, balanceQuery
        },
        methods: { handleFormSubmit},
      };

}