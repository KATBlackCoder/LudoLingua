import { getVersion, getName } from "@tauri-apps/api/app";
export const useAppInfo = () => {
  async function getAppInfo() {
    const appName = await getName();
    const appVersion = await getVersion();
    return {
      name: appName,
      version: appVersion,
    };
  }
  return {
    getAppInfo,
  };
};
