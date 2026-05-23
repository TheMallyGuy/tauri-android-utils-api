use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingRequest {
  pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingResponse {
  pub value: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
  pub package_name: String,
  pub app_name: String,
  pub is_system_app: bool,
  pub is_user_app: bool,
  pub version_name: String,
  pub version_code: i64,
  pub first_install_time: i64,
  pub last_update_time: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppsResult {
  pub apps: Vec<AppInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBannerRequest {
  pub package_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBannerResult {
  pub data: Option<String>,
  pub mime_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WifiSignalResult {
  pub connected: bool,
  /// Signal strength in dBm (e.g. -55). 0 when not connected.
  pub rssi: i32,
  /// Discrete signal level 0–4 (strongest). 0 when not connected.
  pub level: i32,
  pub ssid: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BluetoothStatusResult {
  /// Whether the device has Bluetooth hardware.
  pub available: bool,
  /// Whether Bluetooth is currently switched on.
  pub enabled: bool,
}
