use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::AndroidUtilsExt;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.android_utils().ping(payload)
}

#[command]
pub(crate) async fn get_installed_apps<R: Runtime>(
    app: AppHandle<R>,
) -> Result<AppsResult> {
    app.android_utils().get_installed_apps()
}

#[command]
pub(crate) async fn get_user_installed_apps<R: Runtime>(
    app: AppHandle<R>,
) -> Result<AppsResult> {
    app.android_utils().get_user_installed_apps()
}

#[command]
pub(crate) async fn get_app_tv_banner<R: Runtime>(
    app: AppHandle<R>,
    payload: AppBannerRequest,
) -> Result<AppBannerResult> {
    app.android_utils().get_app_tv_banner(payload)
}

#[command]
pub(crate) async fn get_wifi_signal<R: Runtime>(
    app: AppHandle<R>,
) -> Result<WifiSignalResult> {
    app.android_utils().get_wifi_signal()
}

#[command]
pub(crate) async fn get_bluetooth_status<R: Runtime>(
    app: AppHandle<R>,
) -> Result<BluetoothStatusResult> {
    app.android_utils().get_bluetooth_status()
}

#[command]
pub(crate) async fn open_settings<R: Runtime>(
    app: AppHandle<R>,
) -> Result<()> {
    app.android_utils().open_settings()
}
