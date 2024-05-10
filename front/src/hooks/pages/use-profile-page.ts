import { useCallback, useEffect } from "react";

import { useClipboard } from "@chakra-ui/react";
import { useRouter } from "next/router";
import { useQuery } from "react-query";

import { AppApi, CoinJoinApi, StatechainApi } from "@/apis";
import { CachePrefixKeys } from "@/consts";
import { b64EncodeUnicode } from "@/utils";
import { useDeriv } from "@/hooks";

export const useProfilePage = () => {
  const router = useRouter();
  const { deriv } = useDeriv();

  const profQuery = useQuery([CachePrefixKeys.ProfileFromDeriv, deriv], () =>
    AppApi.getAccount(deriv),
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
    () => AppApi.getUtxos(addr),
    { enabled: !!addr },
  );
  const balanceQuery = useQuery(
    [CachePrefixKeys.Balance, addr],
    () => AppApi.getBalance(addr),
    { enabled: !!addr },
  );

  const listStatecoinsQuery = useQuery(
    [CachePrefixKeys.ListStatecoins, addr],
    () => StatechainApi.listStatecoins(deriv),
    { enabled: !!addr },
  );

  const listTransferStatecoinsQuery = useQuery(
    [CachePrefixKeys.ListTrasferStatecoins, addr],
    () => StatechainApi.listTransferStatecoins(deriv),
    { enabled: !!addr },
  );

  const listRoomsQuery = useQuery(
    [CachePrefixKeys.ListRooms, addr],
    () => CoinJoinApi.getRooms(deriv),
    { enabled: !!addr },
  );


  const onSendBtnClick = useCallback(() => {
    router.push(`/profile/${b64EncodeUnicode(deriv)}/send`);
  }, [deriv]);

  const onDepositBtnClick = useCallback(() => {
    router.push(`/profile/${b64EncodeUnicode(deriv)}/deposit`);
  }, [deriv]);

  const onWithdrawBtnClick = useCallback(() => {
    router.push(`/profile/${b64EncodeUnicode(deriv)}/withdraw`);
  }, [deriv]);

  const onSendStatecoinBtnClick = useCallback(() => {
    router.push(`/profile/${b64EncodeUnicode(deriv)}/send-statecoin`);
  }, [deriv]);

  const onReceiveStatecoinBtnClick = useCallback(() => {
    router.push(`/profile/${b64EncodeUnicode(deriv)}/receive-statecoin`);
  }, [deriv]);

  const onVerifyTransferStatecoinClick = ( deriv: String,transfer_msg : String,authkey: String) => {
    StatechainApi.verifyTransferStatecoin( deriv, transfer_msg, authkey)
  }

  return {
    states: {
      deriv,
      addr,
      hasCopied,
      listUtxoQuery,
      balanceQuery,
      listRoomsQuery,
      listStatecoinsQuery,
      listTransferStatecoinsQuery
    },
    methods: {
      onCopy,
      onSendBtnClick,
      onDepositBtnClick,
      onSendStatecoinBtnClick,
      onWithdrawBtnClick,
      onReceiveStatecoinBtnClick,
      onVerifyTransferStatecoinClick,
    },
  };
};
