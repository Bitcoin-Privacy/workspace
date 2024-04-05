import { TauriConnection } from "./core";
import { StatechainDepositResDto } from "@/dtos";

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

  /* Accessors */

  // async signTx(deriv: string, roomId: string): Promise<RegisterResponseDto> {
  //   return await TauriConnection.callAPI<RegisterResponseDto>("sign_tx", {
  //     deriv,
  //     roomId,
  //   });
  // },
  // /* Fetching Data APIs */
  // async getListRooms(deriv: string): Promise<RoomDto[]> {
  //   return await TauriConnection.callAPI<RoomDto[]>("get_rooms", {
  //     deriv,
  //   });
  // },
  // async getTx(roomId: string): Promise<object> {
  //   return await TauriConnection.callAPI<object>("get_tx", {
  //     roomId,
  //   });
  // },
  // async getStatus(roomId: string): Promise<object> {
  //   return await TauriConnection.callAPI<object>("get_status", {
  //     roomId,
  //   });
  // },
});
