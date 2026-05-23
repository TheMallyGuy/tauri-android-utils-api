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
