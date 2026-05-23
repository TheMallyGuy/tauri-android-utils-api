use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::AndroidUtils;
#[cfg(mobile)]
use mobile::AndroidUtils;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the android-utils APIs.
pub trait AndroidUtilsExt<R: Runtime> {
  fn android_utils(&self) -> &AndroidUtils<R>;
}

impl<R: Runtime, T: Manager<R>> crate::AndroidUtilsExt<R> for T {
  fn android_utils(&self) -> &AndroidUtils<R> {
    self.state::<AndroidUtils<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("android-utils")
    .invoke_handler(tauri::generate_handler![
      commands::ping,
      commands::get_installed_apps,
      commands::get_user_installed_apps,
      commands::get_app_tv_banner,
    ])
    .setup(|app, api| {
      #[cfg(mobile)]
      let android_utils = mobile::init(app, api)?;
      #[cfg(desktop)]
      let android_utils = desktop::init(app, api)?;
      app.manage(android_utils);
      Ok(())
    })
    .build()
}
