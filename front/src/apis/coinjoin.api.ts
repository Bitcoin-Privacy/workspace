import { TauriConnection } from "./core";
import { RegisterResDto, RoomDto } from "@/dtos";

export const CoinJoinApi = Object.freeze({
  name(name: string): string {
    return "plugin:coinjoin|" + name;
  },
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
  async signTx(deriv: string, roomId: string): Promise<RegisterResDto> {
    return await TauriConnection.callAPI<RegisterResDto>(this.name("sign_tx"), {
      deriv,
      roomId,
    });
  },
  /* Fetching Data APIs */
  async getListRooms(deriv: string): Promise<RoomDto[]> {
    return await TauriConnection.callAPI<RoomDto[]>(this.name("get_rooms"), {
      deriv,
    });
  },
  async getTx(roomId: string): Promise<object> {
    return await TauriConnection.callAPI<object>(this.name("get_tx"), {
      roomId,
    });
  },
  async getStatus(roomId: string): Promise<object> {
    return await TauriConnection.callAPI<object>(this.name("get_status"), {
      roomId,
    });
  },
});
