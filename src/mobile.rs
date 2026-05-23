use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_android_utils);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<AndroidUtils<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("com.plugin.androidutils", "AndroidUtils")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_android_utils)?;
  Ok(AndroidUtils(handle))
}

/// Access to the android-utils APIs.
pub struct AndroidUtils<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> AndroidUtils<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    self
      .0
      .run_mobile_plugin("ping", payload)
      .map_err(Into::into)
  }

  pub fn get_installed_apps(&self) -> crate::Result<AppsResult> {
    self
      .0
      .run_mobile_plugin("getInstalledApps", ())
      .map_err(Into::into)
  }

  pub fn get_user_installed_apps(&self) -> crate::Result<AppsResult> {
    self
      .0
      .run_mobile_plugin("getUserInstalledApps", ())
      .map_err(Into::into)
  }

  pub fn get_app_tv_banner(&self, payload: AppBannerRequest) -> crate::Result<AppBannerResult> {
    self
      .0
      .run_mobile_plugin("getAppTvBanner", payload)
      .map_err(Into::into)
  }

  pub fn get_wifi_signal(&self) -> crate::Result<WifiSignalResult> {
    self
      .0
      .run_mobile_plugin("getWifiSignal", ())
      .map_err(Into::into)
  }

  pub fn get_bluetooth_status(&self) -> crate::Result<BluetoothStatusResult> {
    self
      .0
      .run_mobile_plugin("getBluetoothStatus", ())
      .map_err(Into::into)
  }

  pub fn open_settings(&self) -> crate::Result<()> {
    self
      .0
      .run_mobile_plugin("openSettings", ())
      .map_err(Into::into)
  }
}
