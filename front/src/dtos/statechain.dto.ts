export type StateChainDto = {
  txid: string;
  address: string;
  n_locktime: number;
  value: number;
};

export type StatechainDepositResDto = {
  server_pubkey: string,
  statechain_id : string,
  signed_statechain_id: string,
};