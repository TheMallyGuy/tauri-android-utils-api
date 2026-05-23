import { invoke } from "@tauri-apps/api/core";

export interface AppInfo {
  packageName: string;
  appName: string;
  isSystemApp: boolean;
  isUserApp: boolean;
  versionName: string;
  versionCode: number;
  firstInstallTime: number;
  lastUpdateTime: number;
}

export interface AppsResult {
  apps: AppInfo[];
}

/** Get ALL installed apps (system + user) */
export async function getInstalledApps(): Promise<AppInfo[]> {
  const result = await invoke<AppsResult>("plugin:android-utils|get_installed_apps");
  return result.apps;
}

/** Get only user-installed apps (no system apps) */
export async function getUserInstalledApps(): Promise<AppInfo[]> {
  const result = await invoke<AppsResult>("plugin:android-utils|get_user_installed_apps");
  return result.apps;
}