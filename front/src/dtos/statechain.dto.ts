export type StateCoinDto = {
  statechain_id : string,
  amount : number, 
  n_lock_time : number,
};

export type StatechainDepositResDto = {
  aggregated_address : string,
};

export type StateCoinTransferDto = {
  auth_key : String, 
  transfer_message: String, 
}


export type StatecoinDetailDto = {
  statechain_id : string, 
  aggregated_address : string,
  amount : number,
  tx_n : number,
  n_lock_time : number, 
  bk_tx : string,
  funding_txid : string,
  created_at : string
} 


