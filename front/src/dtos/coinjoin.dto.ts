export type RegisterResponseDto = {
  room: string;
  signed_blined_output: string;
};

export type RoomDto = {
  id: string;
  base_amount: number;
  no_peer: number;
  status: number;
  due1: number;
  due2: number;
  created_at: number;
  updated_at: number;
};

export type UtxoDto = {
  txid : string;
  vout : number;
  value : number;
}


export type StateChainDto = {
  txid : string;
  Address : string;
  n_locktime : number;
  value : number;
}