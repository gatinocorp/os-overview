// Impede que uma janela de console extra apareça no Windows em modo release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gio::prelude::*;
use serde::Serialize;


#[tauri::command]
fn greet(name: &str) -> String {
  format!("Olá, {}! Você foi cumprimentado pelo Rust!", name)
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![
        greet,
        launch_app,
        list_installed_apps,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


// ------------------------------------------------------------------------------------------------------------------ //


#[derive(Debug, Serialize)]
struct AppInfo {
  id: Option<String>,
  name: String,
  description: Option<String>,
  executable: Option<String>,
  icon: Option<String>,
}

#[tauri::command]
fn list_installed_apps() -> Vec<AppInfo> {
  let apps = gio::AppInfo::all();
  let mut app_list: Vec<AppInfo> = apps
    .iter()
    .map(|app| {
      let icon_name = app.icon().map(|gicon| match gicon.downcast::<gio::ThemedIcon>() {
        Ok(themed_icon) => themed_icon.names().join(", "),
        Err(_) => "generic-icon".to_string(),
      });
      let executable_string = app.executable().to_string_lossy().into_owned();
      AppInfo {
        id: app.id().map(|id| id.to_string()),
        name: app.display_name().to_string(),
        description: app.description().map(|d| d.to_string()),
        executable: Some(executable_string),
        icon: icon_name,
      }
    })
    .collect();
  app_list.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
  app_list
}

#[tauri::command]
async fn launch_app(app_id: &str) -> Result<(), String> {
  if let Some(app_info) = gio::DesktopAppInfo::new(app_id) {
    match app_info.launch(&[], None::<&gio::AppLaunchContext>) {
      Ok(_) => Ok(()),
      Err(e) => Err(format!("Falha ao abrir a aplicação: {}", e)),
    }
  } else {
    Err(format!("Aplicação com ID '{}' não encontrada.", app_id))
  }
}