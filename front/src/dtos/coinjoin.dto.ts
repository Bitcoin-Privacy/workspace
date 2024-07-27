export type RegisterResDto = {
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
  txid: string | undefined;
  created_at: number;
  updated_at: number;
};

export type UtxoDto = {
  txid: string;
  vout: number;
  value: number;
  status: UtxoStatus;
};

export type UtxoStatus = {
  confirmed: boolean;
};

export type GetSignedResponse = {
  status: number;
};
