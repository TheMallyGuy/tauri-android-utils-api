const COMMANDS: &[&str] = &["ping", "get_installed_apps", "get_user_installed_apps", "get_app_tv_banner"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}
