import { AccountDto } from "../dtos";
import { TauriConnection } from "./core";

export const AccountApi = Object.freeze({
  /* Utils */
  name(name: string): string {
    return "plugin:account|" + name;
  },
  /* Accessors */
  async getListAccounts(): Promise<AccountDto[]> {
    const res = await TauriConnection.callAPI<AccountDto[]>(
      this.name("get_accounts"),
      {},
    );
    return res.sort((a, b) => a.account_number - b.account_number);
  },
  async getAccount(deriv: string): Promise<AccountDto> {
    return await TauriConnection.callAPI<AccountDto>(this.name("get_account"), {
      deriv,
    });
  },
  async getListUtxo(deriv: string): Promise<any[]> {
    return await TauriConnection.callAPI<any[]>(this.name("get_utxo"), {
      address: deriv,
    });
  },
  async getBalance(address: string): Promise<number> {
    return await TauriConnection.callAPI<number>(this.name("get_balance"), {
      address,
    });
  },

  /* Modifiers */
  async createMasterAccount(): Promise<string[]> {
    return await TauriConnection.callAPI<string[]>(
      this.name("create_master"),
      {},
    );
  },
  async createTx(
    deriv: string,
    address: string,
    amount: number,
  ): Promise<void> {
    return await TauriConnection.callAPI<void>(this.name("create_tx"), {
      deriv,
      address,
      amount,
    });
  },
});
