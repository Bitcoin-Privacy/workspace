import { TauriConnection } from "./core";
import { RegisterResDto } from "@/dtos";

export const StatechainApi = Object.freeze({
  /* Utils */
  name(name: string): string {
    return "plugin:statechain|" + name;
  },

  /* Modifiers */
  async deposit(
    deriv: string,
    address: string,
    amount: number,
  ): Promise<RegisterResDto> {
    return await TauriConnection.callAPI<RegisterResDto>(this.name("deposit"), {
      deriv,
      address,
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
