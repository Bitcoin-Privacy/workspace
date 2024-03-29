import { hookstate, State } from "@hookstate/core";

export type AppState = {
  loading: boolean;
  ready: boolean;
  setPassword: boolean;
  setWallet: boolean;
  logged: boolean;
};
export const AppState: State<AppState> = hookstate({
  loading: true,
  ready: false,
} as AppState);
