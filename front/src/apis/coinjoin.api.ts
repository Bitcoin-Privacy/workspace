import { TauriConnection } from "./core";
import { RegisterResponseDto, RoomDto } from "@/dtos";

export const CoinJoinApi = Object.freeze({
  async register(
    deriv: string,
    address: string,
    amount: number,
  ): Promise<RegisterResponseDto> {
    return await TauriConnection.callAPI<RegisterResponseDto>("register", {
      deriv,
      address,
      amount,
    });
  },
  async signTx(deriv: string, roomId: string): Promise<RegisterResponseDto> {
    return await TauriConnection.callAPI<RegisterResponseDto>("sign_tx", {
      deriv,
      roomId,
    });
  },
  /* Fetching Data APIs */
  async getListRooms(deriv: string): Promise<RoomDto[]> {
    return await TauriConnection.callAPI<RoomDto[]>("get_rooms", {
      deriv,
    });
  },
  async getTx(roomId: string): Promise<object> {
    return await TauriConnection.callAPI<object>("get_tx", {
      roomId,
    });
  },
  async getStatus(roomId: string): Promise<object> {
    return await TauriConnection.callAPI<object>("get_status", {
      roomId,
    });
  },
});
