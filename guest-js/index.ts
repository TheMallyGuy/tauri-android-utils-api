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

export type UserInstalledAppsResult = AppInfo[];

/** Get ALL installed apps (system + user) */
export async function getInstalledApps(): Promise<AppInfo[]> {
  const result = await invoke<AppsResult>("plugin:android-utils|get_installed_apps");
  return result.apps;
}

/** Get only user-installed apps (no system apps) */
export async function getUserInstalledApps(): Promise<UserInstalledAppsResult> {
  const result = await invoke<AppsResult>("plugin:android-utils|get_user_installed_apps");
  return result.apps;
}

export interface AppBannerResult {
  /** Base64-encoded PNG image, or null if unavailable */
  data: string | null;
  mimeType: string | null;
}

/**
 * Get a TV banner or launcher icon for an app as a base64 PNG.
 * Prefers the wide TV banner (320×180) defined in the app manifest;
 * falls back to the regular launcher icon.
 */
export async function getAppTvBanner(packageName: string): Promise<AppBannerResult> {
  return await invoke<AppBannerResult>("plugin:android-utils|get_app_tv_banner", { packageName });
}

export interface WifiSignalResult {
  /** Whether the device is currently connected to a WiFi network. */
  connected: boolean;
  /** Signal strength in dBm (e.g. -55). 0 when not connected. */
  rssi: number;
  /** Discrete signal level 0–4 (4 = strongest). 0 when not connected. */
  level: number;
  /** SSID of the connected network, or empty string when not connected. */
  ssid: string;
}

export interface BluetoothStatusResult {
  /** Whether the device has Bluetooth hardware. */
  available: boolean;
  /** Whether Bluetooth is currently switched on. */
  enabled: boolean;
}

/** Get the current WiFi signal strength and connection info. */
export async function getWifiSignal(): Promise<WifiSignalResult> {
  return await invoke<WifiSignalResult>("plugin:android-utils|get_wifi_signal");
}

/** Get whether Bluetooth hardware is available and switched on. */
export async function getBluetoothStatus(): Promise<BluetoothStatusResult> {
  return await invoke<BluetoothStatusResult>("plugin:android-utils|get_bluetooth_status");
}

/** Open the Android system Settings app. */
export async function openSettings(): Promise<void> {
  await invoke("plugin:android-utils|open_settings");
}