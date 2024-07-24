import { TauriConnection } from "./core";
import {
  StatechainDepositResDto,
  StatecoinDetailDto,
  StateCoinDto,
  StateCoinTransferDto,
} from "@/dtos";

export const StatechainApi = Object.freeze({
  /* Utils */
  name(name: string): string {
    return "plugin:statechain|" + name;
  },

  /* Modifiers */
  async deposit(
    deriv: string,
    amount: number,
  ): Promise<StatechainDepositResDto> {
    return await TauriConnection.callAPI<StatechainDepositResDto>(
      this.name("deposit"),
      {
        deriv,
        amount,
      },
    );
  },

  async listStatecoins(deriv: string): Promise<StateCoinDto[]> {
    return await TauriConnection.callAPI<StateCoinDto[]>(
      this.name("list_statecoins"),
      {
        deriv,
      },
    );
  },

  async sendStatecoin(address: string, statechainId: string): Promise<String> {
    return await TauriConnection.callAPI<String>(this.name("send_statecoin"), {
      address,
      statechainId,
    });
  },

  async listTransferStatecoins(deriv: string): Promise<StateCoinTransferDto[]> {
    return await TauriConnection.callAPI<StateCoinTransferDto[]>(
      this.name("list_transfer_statecoins"),
      {
        deriv,
      },
    );
  },

  async genStatechainAddress(deriv: string): Promise<string> {
    return await TauriConnection.callAPI<string>(
      this.name("generate_statechain_address"),
      {
        deriv,
      },
    );
  },

  async verifyTransferStatecoin(
    deriv: String,
    transferMessage: String,
    authkey: String,
  ): Promise<String> {
    return await TauriConnection.callAPI<String>(
      this.name("verify_transfer_statecoin"),
      {
        deriv,
        transferMessage,
        authkey,
      },
    );
  },

  async getStatecoinDetailById(
    statechainId: string,
  ): Promise<StatecoinDetailDto> {
    return await TauriConnection.callAPI<StatecoinDetailDto>(
      this.name("get_statecoin_detail_by_id"),
      {
        statechainId,
      },
    );
  },

  async withdrawStatecoin(
    statechainId: string,
    deriv: string,
  ): Promise<object> {
    return await TauriConnection.callAPI<object>(
      this.name("withdraw_statecoin"),
      {
        statechainId,
        deriv,
      },
    );
  },
});
