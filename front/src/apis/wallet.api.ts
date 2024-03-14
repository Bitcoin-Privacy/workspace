import { TauriConnection } from "./core";
import { InitState, mapToInitState } from "@/dtos";

export const WalletApi = Object.freeze({
  async getInitState(): Promise<InitState> {
    const res = await TauriConnection.callAPI<object>("get_init_state", {});
    return mapToInitState(res);
  },
  async savePassword(password: string): Promise<void> {
    return TauriConnection.callAPI<void>("save_password", { password });
  },
});
