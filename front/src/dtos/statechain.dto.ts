export type StateCoinDto = {
  statechain_id : string,
  aggregated_address : string,
  amount : number, 
  funding_txid: string,
  funding_vout : number, 
  n_lock_time : number,
};

export type StatechainDepositResDto = {
  aggregated_address : string,
  deposit_tx_hex : string,
};

export type StateCoinTransferDto = {
  auth_key : String, 
  transfer_message: String, 
}
