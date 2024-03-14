import { hookstate, State } from "@hookstate/core";

export type AppState = {
  loading: boolean;
  ready: boolean;
  seedPhrase: string | undefined;
};
export const AppState: State<AppState> = hookstate({
  loading: true,
  ready: false,
} as AppState);
