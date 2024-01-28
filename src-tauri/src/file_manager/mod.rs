use native_dialog::{FileDialog};
use std::path::PathBuf;


#[tauri::command(rename_all = "snake_case")]
pub fn texcol_select_file() -> String {
    let path = FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("Tex File", &["tex"])
        .show_open_single_file()
        .unwrap();
    let path: PathBuf = match path {
        Some(path) => path,
        None => PathBuf::from(""),
    };
    
    let path = path.to_str().unwrap().to_string();
    path
}

#[tauri::command(rename_all = "snake_case")]
pub fn texcol_select_bib_file() -> String {
    let path = FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("Bib File", &["bib"])
        .show_open_single_file()
        .unwrap();
    let path: PathBuf = match path {
        Some(path) => path,
        None => PathBuf::from(""),
    };
    
    let path = path.to_str().unwrap().to_string();
    path
}



#[tauri::command(rename_all = "snake_case")]
pub fn texcol_select_dir() -> String {
    let path = FileDialog::new()
        .set_location("~/Desktop")
        .show_open_single_dir()
        .unwrap();
    let path: PathBuf = match path {
        Some(path) => path,
        None => PathBuf::from(""),
    };
    let path = path.to_str().unwrap().to_string();
    path
}