use chrono::format;
use diesel::sqlite::SqliteConnection;
use tauri::Manager;

use crate::project_manager::pmdb::check_sqlite_and_run_migration;

use super::lazy_vars::APP_SQLITE_PATH;
use super::project_manager::pmdb;
use std::path::{Path, PathBuf};


// for creating reviews
#[tauri::command(rename_all = "snake_case")]
pub async fn create_review_window(proj_id: i32, handle: tauri::AppHandle) {
  let url_str = format!("review/{}/create", proj_id);
  println!("url_str: {}", url_str);
  let _create_review_window = tauri::WindowBuilder::new(
    &handle,
    "create_review", /* the unique window label */
    tauri::WindowUrl::App(PathBuf::from(url_str))
  ).title("Create Review").center().inner_size(400.0, 600.0).build().unwrap();
}

// for close review window
#[tauri::command(rename_all = "snake_case")]
pub async fn close_review_window(handle: tauri::AppHandle) {
  let window = handle.get_window("create_review").unwrap();
  window.close().unwrap();
}


// for creating projects 
#[tauri::command(rename_all = "snake_case")]
pub async fn create_project_window(handle: tauri::AppHandle) {
  let _create_project_window = tauri::WindowBuilder::new(
    &handle,
    "create_project", /* the unique window label */
    // tauri::WindowUrl::External("http://localhost:1420/project/create".parse().unwrap())
    tauri::WindowUrl::App(PathBuf::from("project/create"))
  ).title("Create Project").center().inner_size(400.0, 600.0).build().unwrap();
}

#[tauri::command(rename_all = "snake_case")]
pub async fn close_create_project_window(handle: tauri::AppHandle) {
  let window = handle.get_window("create_project").unwrap();
  window.close().unwrap();
}

#[tauri::command(rename_all = "snake_case")]
pub async fn check_window_status_by_name(handle: tauri::AppHandle, window_name: String) -> bool {
  let window = handle.get_window(&window_name);
  if window.is_none() {
    return false;
  } else {
    let window = window.unwrap();
    let result = match window.is_visible() {
      Ok(result) => result,
      Err(_) => false,
    };
    return result
  }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn check_create_project_window_status(handle: tauri::AppHandle) -> bool {
  let window = handle.get_window("create_project");
  if window.is_none() {
    return false;
  } else {
    let window = window.unwrap();
    let result = match window.is_visible() {
      Ok(result) => result,
      Err(_) => false,
        
    };
    return result
  }
}

// check window status
#[tauri::command(rename_all = "snake_case")]
pub async fn check_review_window_status(handle: tauri::AppHandle) -> bool {
  let window = handle.get_window("create_review");
  if window.is_none() {
    return false;
  } else {
    let window = window.unwrap();
    let result = match window.is_visible() {
      Ok(result) => result,
      Err(_) => false,
        
    };
    return result
  }
}

// return app dir
#[tauri::command(rename_all = "snake_case")]
pub fn texcol_app_dir(handle: tauri::AppHandle) -> bool {
  let resource_path = handle.path_resolver().resource_dir().unwrap();
  let resource_path = resource_path.to_str().unwrap();
  let mut sqllite_path = APP_SQLITE_PATH.lock().unwrap();
  *sqllite_path = format!("{}\\texcol_db.sqlite3", resource_path).to_string();
  
  check_sqlite_and_run_migration(&(*sqllite_path.as_str()));

  let path = Path::new(resource_path);
  if path.exists() {
    true
  } else {
    false
  }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn texcol_sleep_ms(ms: u64) {
  std::thread::sleep(std::time::Duration::from_millis(ms));
}
