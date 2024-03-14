import { useCallback, useEffect } from "react";

import { useClipboard } from "@chakra-ui/react";
import { useRouter } from "next/router";
import { useQuery } from "react-query";

import { AccountApi, CoinJoinApi } from "@/apis";
import { CachePrefixKeys } from "@/consts";
import { b64EncodeUnicode } from "@/utils";
import { useDeriv } from "@/hooks";

export const useProfilePage = () => {
  const router = useRouter();
  const { deriv } = useDeriv();

  const profQuery = useQuery([CachePrefixKeys.ProfileFromDeriv, deriv], () =>
    AccountApi.getAccount(deriv),
  );

  const {
    value: addr,
    setValue: setAddr,
    onCopy,
    hasCopied,
  } = useClipboard("");

  useEffect(() => {
    if (profQuery.data?.address) setAddr(profQuery.data?.address);
  }, [profQuery.data?.address]);

  const listUtxoQuery = useQuery(
    [CachePrefixKeys.UTXO, addr],
    () => AccountApi.getListUtxo(addr),
    { enabled: !!addr },
  );
  const balanceQuery = useQuery(
    [CachePrefixKeys.Balance, addr],
    () => AccountApi.getBalance(addr),
    { enabled: !!addr },
  );

  const listRoomsQuery = useQuery(
    [CachePrefixKeys.ListRooms, addr],
    () => CoinJoinApi.getListRooms(deriv),
    { enabled: !!addr },
  );

  const onSendBtnClick = useCallback(() => {
    router.push(`/profile/${b64EncodeUnicode(deriv)}/send`);
  }, [deriv]);

  return {
    states: {
      deriv,
      addr,
      hasCopied,
      listUtxoQuery,
      balanceQuery,
      listRoomsQuery,
    },
    methods: {
      onCopy,
      onSendBtnClick,
    },
  };
};
