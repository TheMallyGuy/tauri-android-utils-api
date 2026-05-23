const COMMANDS: &[&str] = &["ping", "get_installed_apps", "get_user_installed_apps", "get_app_tv_banner", "get_wifi_signal", "get_bluetooth_status", "open_settings"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}
