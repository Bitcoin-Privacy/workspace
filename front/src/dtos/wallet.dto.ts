export enum InitStateEnum {
  BrandNew = "BrandNew",
  CreatedPassword = "CreatedPassword",
  CreatedWallet = "CreatedWallet",
}

export type InitState = {
  type: InitStateEnum;
  password?: string;
};

export function mapToInitState(raw: any): InitState {
  return {
    type: InitStateEnum[raw.type as keyof typeof InitStateEnum],
    password: raw.password,
  };
}

export enum TxStrategyEnum {
  Base = "Base",
  CoinJoin = "CoinJoin",
}

