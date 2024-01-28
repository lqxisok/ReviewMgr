use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub tex_path: String,
  pub bib_path: String,
  pub proj_path: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

// for review 
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Review {
  pub id: i32,
  pub project_id: i32,
  pub status: bool,
  pub reviewer: String,
  pub description: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

// for create review
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateReviewArgs {
  pub proj_id: i32,
  pub status: bool,
  pub reviewer: String,
  pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectIdArgs {
    pub id: i32,
}


#[derive(Serialize, Deserialize)]
pub struct ReviewFromProjectIdArgs {
    pub proj_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteReviewIdArgs {
    pub review_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectContentSaveArgs {
    pub id: i32,
    pub content: String,
}


#[derive(Serialize, Deserialize)]
pub struct CreateProjectArgs<'a> {
  pub project_name:  &'a str,
  pub tex_path: &'a str,
  pub bib_tex_path: &'a str,
  pub dir_path: &'a str,
}


#[derive(Serialize, Deserialize)]
pub struct SleepArgs {
    pub ms: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ReviewWindowArgs {
    pub proj_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct WindowNameArgs {
    pub window_name: String,
}


