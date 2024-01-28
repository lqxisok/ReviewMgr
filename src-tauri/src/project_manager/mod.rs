mod pmdb;

use core::num;
use std::fs::read_to_string;
use std::path::Path;

use pmdb::establish_connection;
use pmdb::create_project;
use pmdb::get_projects;
use ropey::Rope;

use self::pmdb::model::Project;

use super::texcol_preprocess::latex_to_html_string;
use super::texcol_preprocess::regenerate_all_tex_content;
use super::texcol_preprocess::file_utils::{read_file, change_CRLF_to_LF};
use super::texcol_preprocess::{process_bib_file, tex_rope_to_html};

#[tauri::command(rename_all = "snake_case")]
pub fn texcol_create_project(project_name: &str, tex_path: &str, bib_tex_path: &str, dir_path: &str) -> String {
    let mut conn = establish_connection();

    // read the tex file and 
    let tex_path_obj = Path::new(tex_path);
    let input = read_file(tex_path_obj);
    let input = change_CRLF_to_LF(input.unwrap());
    let result = regenerate_all_tex_content(input.as_str());

    // write a new tex file
    let new_tex_path = dir_path.to_owned() + "/" + project_name + ".tex";
    let new_tex_path_obj = Path::new(new_tex_path.as_str());
    let _ = std::fs::write(new_tex_path_obj, result.as_bytes());

    let bib_tex_path_obj = Path::new(bib_tex_path);
    let bib_src = read_file(bib_tex_path_obj);
    let bib_src = change_CRLF_to_LF(bib_src.unwrap());

    // write a new bib file
    let new_bib_tex_path = dir_path.to_owned() + "/" + project_name + ".bib";
    let new_bib_tex_path_obj = Path::new(new_bib_tex_path.as_str());
    let _ = std::fs::write(new_bib_tex_path_obj, bib_src.as_bytes());

    let proj = create_project(&mut conn, project_name, new_tex_path.as_str(), new_bib_tex_path.as_str(), dir_path).expect("Error creating project");

    println!("proj: {:?}", proj);
    format!("Project created!")
}

#[tauri::command(rename_all = "snake_case")]
pub fn texcol_get_all_projects() -> Vec<Project> {
    let mut conn = establish_connection();
    let projects = get_projects(&mut conn).expect("Error getting projects");
    projects
}

// get project info by id
#[tauri::command(rename_all = "snake_case")]
pub fn texcol_get_project_by_id(id: i32) -> Project {
    let mut conn = establish_connection();
    let project = pmdb::get_project_by_id(&mut conn, id).expect("Error getting project");
    project
}

// delete project by id
#[tauri::command(rename_all = "snake_case")]
pub fn texcol_delete_project_by_id(id: i32) -> bool {
    let mut conn = establish_connection();
    let num_deleted = pmdb::delete_project(&mut conn, id).expect("Error deleting project");
    if num_deleted == 1 {
        true
    } else {
        false
    }
}

// according to the id to open the folder
#[tauri::command(rename_all = "snake_case")]
pub fn texcol_open_project_folder_by_id(id: i32) -> bool {
    let mut conn = establish_connection();
    let project = pmdb::get_project_by_id(&mut conn, id).expect("Error getting project");
    let dir_path = project.proj_path;
    let dir_path_str = dir_path.as_str();
    let _output = std::process::Command::new("explorer")
        .arg(dir_path_str).status();
    let path = Path::new(dir_path_str);
    let path_exists = path.exists();
    path_exists
}

// convert the project tex file to html string
#[tauri::command(rename_all = "snake_case")]
pub fn telcol_convert_tex_to_html_by_id(id: i32) -> String {
    let mut conn = establish_connection();
    let project = pmdb::get_project_by_id(&mut conn, id).expect("Error getting project");
    let tex_path = project.tex_path;
    let tex_path_str = tex_path.as_str();
    let bib_tex_path = project.bib_path;
    let bib_path_str = bib_tex_path.as_str();

    let bib_string = read_to_string(bib_path_str).unwrap();
    let bib_source = process_bib_file(&bib_string, Path::new(bib_path_str));

    let tex_rope = Rope::from_str(&read_to_string(tex_path_str).unwrap());
    let html_string = tex_rope_to_html(&tex_rope, bib_source, tex_path_str);
    html_string
}

// read the raw tex content by id
#[tauri::command(rename_all = "snake_case")]
pub fn texcol_read_raw_tex_content_by_id(id: i32) -> String {
    let mut conn: diesel::prelude::SqliteConnection = establish_connection();
    let project = pmdb::get_project_by_id(&mut conn, id).expect("Error getting project");
    let tex_path = project.tex_path;
    let tex_path_str = tex_path.as_str();
    let tex_string = read_to_string(tex_path_str).unwrap();
    tex_string
}

// update the raw tex content by id
#[tauri::command(rename_all = "snake_case")]
pub fn texcol_update_raw_tex_content_by_id(id: i32, content: &str) -> String {
    let mut conn: diesel::prelude::SqliteConnection = establish_connection();
    let project = pmdb::get_project_by_id(&mut conn, id).expect("Error getting project");
    let tex_path = project.tex_path;
    let tex_path_str = tex_path.as_str();
    let _ = std::fs::write(tex_path_str, content.as_bytes());
    format!("Raw tex content updated!")
}

// create review for the project
#[tauri::command(rename_all = "snake_case")]
pub fn texcol_create_review_for_project(proj_id: i32, status: bool, reviewer: &str, description: &str) -> String {
    let mut conn = establish_connection();
    let num_created = pmdb::create_review(&mut conn, proj_id, status, reviewer, description).expect("Error creating review");
    format!("{} reviews created", num_created)
}


// get all reviews for the project
#[tauri::command(rename_all = "snake_case")]
pub fn texcol_get_all_reviews_for_project(proj_id: i32) -> Vec<pmdb::model::Review> {
    let mut conn = establish_connection();
    let reviews = pmdb::get_reviews_by_proj_id(&mut conn, proj_id).expect("Error getting reviews");
    reviews
}

// update review by id
#[tauri::command(rename_all = "snake_case")]
pub fn texcol_update_review_by_id(review_id: i32, status: bool, reviewer: &str, description: &str) -> String {
    let mut conn = establish_connection();
    let num_updated = pmdb::update_review_by_id(&mut conn, review_id, status, reviewer, description).expect("Error updating review");
    format!("{} reviews updated", num_updated)
}

// delete review by id
#[tauri::command(rename_all = "snake_case")]
pub fn texcol_delete_review_by_id(review_id: i32) -> String {
    let mut conn = establish_connection();
    let num_deleted = pmdb::delete_review_by_id(&mut conn, review_id).expect("Error deleting review");
    format!("{} review deleted", num_deleted)
}