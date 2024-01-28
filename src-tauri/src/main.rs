// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file_manager;
mod project_manager;
mod window_manager;
mod lazy_vars;
mod texcol_preprocess;

use file_manager::{texcol_select_file, texcol_select_dir, texcol_select_bib_file};
use project_manager::texcol_create_project;
use project_manager::texcol_get_all_projects;
use project_manager::{texcol_get_project_by_id, texcol_delete_project_by_id,
     texcol_open_project_folder_by_id, telcol_convert_tex_to_html_by_id, texcol_read_raw_tex_content_by_id};
use project_manager::{texcol_create_review_for_project, texcol_get_all_reviews_for_project,
    texcol_update_review_by_id, texcol_delete_review_by_id, texcol_update_raw_tex_content_by_id};
use window_manager::{create_project_window, close_create_project_window,
     check_create_project_window_status,texcol_sleep_ms, create_review_window, close_review_window};
use window_manager::texcol_app_dir;

use window_manager::{check_window_status_by_name, check_review_window_status};


fn main() {

    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            texcol_create_project, texcol_select_bib_file,
            texcol_select_file, texcol_select_dir, texcol_get_all_projects,
            create_project_window, texcol_app_dir, close_create_project_window,
            check_create_project_window_status, texcol_sleep_ms,
            texcol_get_project_by_id, texcol_delete_project_by_id,
            texcol_open_project_folder_by_id, telcol_convert_tex_to_html_by_id,
            texcol_create_review_for_project, texcol_get_all_reviews_for_project,
            texcol_update_review_by_id, texcol_delete_review_by_id,
            texcol_read_raw_tex_content_by_id, texcol_update_raw_tex_content_by_id,
            create_review_window, close_review_window,
            check_window_status_by_name, check_review_window_status
            ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|_, _| {});
}


