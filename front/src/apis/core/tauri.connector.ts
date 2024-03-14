export const TauriConnection = Object.freeze({
  async callAPI<T>(name: string, body?: Record<string, unknown>): Promise<T> {
    const tauriApi = await import("@tauri-apps/api");
    try {
      const res = await tauriApi.invoke<T>(name, body);
      console.log(
        `[TAURI] ${name}: ${JSON.stringify(body)}\n > Res: ${JSON.stringify(
          res,
        )}`,
      );
      return res;
    } catch (e) {
      console.log(
        `[TAURI] ${name}: ${JSON.stringify(body)}\n > Err: ${JSON.stringify(
          e,
        )}`,
      );
      throw e;
    }
  },
});
