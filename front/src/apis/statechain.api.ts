import { TauriConnection } from "./core";
import { StatechainDepositResDto, StateCoinDto, StateCoinTransferDto } from "@/dtos";

export const StatechainApi = Object.freeze({
  /* Utils */
  name(name: string): string {
    return "plugin:statechain|" + name;
  },

  /* Modifiers */
  async deposit(
    deriv : string,
    amount: number,
  ): Promise<StatechainDepositResDto> {
    return await TauriConnection.callAPI<StatechainDepositResDto>(this.name("deposit"), {
      deriv,
      amount,
    });
  },

  async listStatecoins(
    deriv : string,
  ): Promise<[StateCoinDto]> {
    return await TauriConnection.callAPI<[StateCoinDto]>(this.name("list_statecoins"), {
      deriv,
    });
  },

  async sendStatecoin(
    address : string,
    statechainId : string,
  ): Promise<String> {
    return await TauriConnection.callAPI<String>(this.name("send_statecoin"), {
      address,
      statechainId,
    });
  },

  async listTransferStatecoins(
    deriv : string,
  ): Promise<[StateCoinTransferDto]> {
    return await TauriConnection.callAPI<[StateCoinTransferDto]>(this.name("list_transfer_statecoins"), {
      deriv,
    });
  },

  async genStatechainAddress(
    deriv : string,
  ): Promise<string> {
    return await TauriConnection.callAPI<string>(this.name("generate_statechain_address"), {
      deriv,
    });
  },


  async verifyTransferStatecoin(
    deriv : String, 
    transferMessage : String, 
    authkey: String,
  ) : Promise <String>{
    return await TauriConnection.callAPI<String>(this.name("verify_transfer_statecoin"), {
      deriv,
      transferMessage,
      authkey
    });
  }
});
