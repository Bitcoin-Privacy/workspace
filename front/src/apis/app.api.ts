import { TauriConnection } from "./core";
import { InitState, mapToInitState, AccountDto } from "@/dtos";

export const AppApi = Object.freeze({
  /* Utils */
  name(name: string): string {
    return "plugin:app|" + name;
  },

  /* Modifiers */
  async signUp(password: string): Promise<void> {
    return TauriConnection.callAPI<void>(this.name("signup"), {
      password,
    });
  },
  async signIn(password: string): Promise<boolean> {
    return TauriConnection.callAPI<boolean>(this.name("signin"), { password });
  },
  async createMaster(): Promise<string[]> {
    return await TauriConnection.callAPI<string[]>(
      this.name("create_master"),
      {},
    );
  },
  async createTxn(
    deriv: string,
    receiver: string,
    amount: number,
  ): Promise<void> {
    return await TauriConnection.callAPI<void>(this.name("create_txn"), {
      deriv,
      receiver,
      amount,
    });
  },

  /* Accessors */
  async getInitState(): Promise<InitState> {
    const res = await TauriConnection.callAPI<object>(
      this.name("get_init_state"),
      {},
    );
    return mapToInitState(res);
  },
  async getAccounts(): Promise<AccountDto[]> {
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
  async getUtxos(deriv: string): Promise<any[]> {
    return await TauriConnection.callAPI<any[]>(this.name("get_utxos"), {
      address: deriv,
    });
  },
  async getStatus(txid: string): Promise<boolean> {
    return await TauriConnection.callAPI<boolean>(this.name("get_status"), {
      txid,
    });
  },
  async getBalance(address: string): Promise<number> {
    return await TauriConnection.callAPI<number>(this.name("get_balance"), {
      address,
    });
  },
});
