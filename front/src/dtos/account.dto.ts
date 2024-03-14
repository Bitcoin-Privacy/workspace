export type AccountDto = AccountIndentity & {
  address_type: "P2PKH" | "P2WPKH";
  network: "testnet";
  address: string;
};

export type AccountIndentity = {
  account_number: number;
  sub_account_number: number;
};
