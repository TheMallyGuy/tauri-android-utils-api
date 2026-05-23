use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<AndroidUtils<R>> {
  Ok(AndroidUtils(app.clone()))
}

/// Access to the android-utils APIs.
pub struct AndroidUtils<R: Runtime>(AppHandle<R>);

impl<R: Runtime> AndroidUtils<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }

  pub fn get_installed_apps(&self) -> crate::Result<AppsResult> {
    Ok(AppsResult::default())
  }

  pub fn get_user_installed_apps(&self) -> crate::Result<AppsResult> {
    Ok(AppsResult::default())
  }

  pub fn get_app_tv_banner(&self, _payload: AppBannerRequest) -> crate::Result<AppBannerResult> {
    Ok(AppBannerResult::default())
  }

  pub fn get_wifi_signal(&self) -> crate::Result<WifiSignalResult> {
    Ok(WifiSignalResult::default())
  }

  pub fn get_bluetooth_status(&self) -> crate::Result<BluetoothStatusResult> {
    Ok(BluetoothStatusResult::default())
  }

  pub fn open_settings(&self) -> crate::Result<()> {
    Ok(())
  }
}
