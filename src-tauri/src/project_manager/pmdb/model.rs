use diesel::prelude::*;
use super::schema::projects;
use super::schema::reviews;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub tex_path: String,
    pub bib_path: String,
    pub proj_path: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = projects)]
pub struct NewProject<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub tex_path: &'a str,
    pub bib_path: &'a str,
    pub proj_path: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = reviews)]
pub struct NewReview<'a> {
    pub project_id: i32,
    pub status: bool,
    pub reviewer: &'a str,
    pub description: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Review {
    pub id: i32,
    pub project_id: i32,
    pub status: bool,
    pub reviewer: String,
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
