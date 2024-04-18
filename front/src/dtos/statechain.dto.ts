export type StateChainDto = {
  txid: string;
  address: string;
  n_locktime: number;
  value: number;
};

export type StatechainDepositResDto = {
  aggregated_address : string,
  deposit_tx_hex : string,
};