import { TauriConnection } from "./core";
import { StatechainDepositResDto } from "@/dtos";

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

  /* Accessors */
  async getStatecoins(deriv: string): Promise<any[]> {
    return await TauriConnection.callAPI<any[]>(this.name("get_statecoins"), {
      deriv,
    });
  },
});
