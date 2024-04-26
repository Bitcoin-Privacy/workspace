import { TauriConnection } from "./core";
import { StatechainDepositResDto, StateCoinDto } from "@/dtos";

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
});
