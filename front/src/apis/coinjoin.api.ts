import { TauriConnection } from "./core";
import { RegisterResDto, RoomDto } from "@/dtos";

export const CoinJoinApi = Object.freeze({
  /* Utils */
  name(name: string): string {
    return "plugin:coinjoin|" + name;
  },

  /* Modifiers */
  async register(
    deriv: string,
    address: string,
    amount: number,
  ): Promise<RegisterResDto> {
    return await TauriConnection.callAPI<RegisterResDto>(
      this.name("register"),
      {
        deriv,
        address,
        amount,
      },
    );
  },
  async signTxn(deriv: string, roomId: string): Promise<RegisterResDto> {
    return await TauriConnection.callAPI<RegisterResDto>(
      this.name("sign_txn"),
      {
        deriv,
        roomId,
      },
    );
  },

  /* Accessors */
  async getRooms(deriv: string): Promise<RoomDto[]> {
    return await TauriConnection.callAPI<RoomDto[]>(this.name("get_rooms"), {
      deriv,
    });
  },
  async getStatus(roomId: string): Promise<object> {
    return await TauriConnection.callAPI<object>(this.name("get_status"), {
      roomId,
    });
  },
  async getSigned(deriv: string, roomId: string): Promise<object> {
    return await TauriConnection.callAPI<object>(this.name("get_signed"), {
      deriv,
      roomId,
    });
  },
});
