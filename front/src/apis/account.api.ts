import { AccountDto } from "../dtos";
import { TauriConnection } from "./core";

export const AccountApi = Object.freeze({
    /* Fetching Data APIs */
    async getListAccounts(): Promise<AccountDto[]> {
        const res = await TauriConnection.callAPI<AccountDto[]>("get_accounts", {});
        return res.sort((a, b) => a.account_number - b.account_number);
    },
    async getAccount(deriv: string): Promise<AccountDto> {
        return await TauriConnection.callAPI<AccountDto>("get_account", {
            deriv,
        });
    },
    async getListUtxo(deriv: string): Promise<any[]> {
        return await TauriConnection.callAPI<any[]>("get_utxo", { address: deriv });
    },
    async getBalance(address: string): Promise<number> {
        return await TauriConnection.callAPI<number>("get_balance", { address });
    },

    /* Upsert Data APIs */
    async createMasterAccount(): Promise<string[]> {
        return await TauriConnection.callAPI<string[]>("create_master", {});
    },
    async createTx(
        deriv: string,
        address: string,
        amount: number,
    ): Promise<void> {
        return await TauriConnection.callAPI<void>("create_tx", {
            deriv,
            address,
            amount,
        });
    },
});
