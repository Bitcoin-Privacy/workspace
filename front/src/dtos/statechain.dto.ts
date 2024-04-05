export type StateChainDto = {
  txid: string;
  address: string;
  n_locktime: number;
  value: number;
};

export type StatechainDepositResDto = {
  aggregated_pubkey : string,
  aggregated_address : string
};