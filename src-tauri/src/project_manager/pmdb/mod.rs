///
/// Project Manager Database
/// data: 2024年1月20日
/// Driver: sqlite3
/// 
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::result::Error;
use super::super::lazy_vars::APP_SQLITE_PATH;

pub mod schema;
pub mod model;

use model::Project;


pub fn establish_connection() -> SqliteConnection {
    let database_url = APP_SQLITE_PATH.lock().unwrap().clone();
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_project<'a>(conn: &mut SqliteConnection, name: &'a str,
    tex_path: &'a str, bib_path: &'a str, proj_path: &'a str) -> Result<Project, Error> 
{
    use schema::projects;
    use model::NewProject;

    let new_proj = NewProject { 
        name, 
        description: "test project description",
        tex_path,
        bib_path,
        proj_path,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    diesel::insert_into(projects::table)
        .values(&new_proj)
        .execute(conn)?;
    projects::table.order(projects::id.desc()).first(conn)
}

// Read projects
pub fn get_projects(conn: &mut SqliteConnection) -> Result<Vec<Project>, Error> {
    use schema::projects::dsl::*;
    projects.load::<Project>(conn)
}

// Read project by id
pub fn get_project_by_id(conn: &mut SqliteConnection, proj_id: i32) -> Result<Project, Error> {
    use schema::projects::dsl::*;
    projects.filter(id.eq(proj_id)).first(conn)
}

// Delete project
pub fn delete_project(conn: &mut SqliteConnection, proj_id: i32) -> Result<usize, Error> {
    use schema::projects::dsl::*;
    diesel::delete(projects.filter(id.eq(proj_id))).execute(conn)
}

// create review
pub fn create_review<'a>(conn: &mut SqliteConnection, proj_id: i32, status: bool,
    reviewer: &'a str, description: &'a str) -> Result<usize, Error> 
{
    use schema::reviews;
    use model::NewReview;

    let new_review = NewReview { 
        project_id: proj_id,
        status, 
        reviewer,
        description,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    diesel::insert_into(reviews::table)
        .values(&new_review)
        .execute(conn)
}

// Read reviews by project id
pub fn get_reviews_by_proj_id(conn: &mut SqliteConnection, proj_id: i32) -> Result<Vec<model::Review>, Error> {
    use schema::reviews::dsl::*;
    reviews.filter(project_id.eq(proj_id)).load::<model::Review>(conn)
}


// update reviews by review id
pub fn update_review_by_id(conn: &mut SqliteConnection, review_id: i32, status: bool,
    reviewer: &str, description: &str) -> Result<usize, Error> 
{
    use schema::reviews::dsl::*;
    diesel::update(reviews.filter(id.eq(review_id)))
        .set((status.eq(status), reviewer.eq(reviewer), description.eq(description)))
        .execute(conn)
}

// delete review by review id
pub fn delete_review_by_id(conn: &mut SqliteConnection, review_id: i32) -> Result<usize, Error> {
    use schema::reviews::dsl::*;
    diesel::delete(reviews.filter(id.eq(review_id))).execute(conn)
}

fn texcol_run_migration(conn: &mut SqliteConnection) {
    
    let migrations = vec![
        "CREATE TABLE IF NOT EXISTS projects (id INTEGER NOT NULL PRIMARY KEY, name VARCHAR(255) NOT NULL, description TEXT NOT NULL, tex_path TEXT NOT NULL, bib_path TEXT NOT NULL, proj_path TEXT NOT NULL, created_at TIMESTAMP NOT NULL, updated_at TIMESTAMP NOT NULL);",
        "CREATE TABLE IF NOT EXISTS reviews (id INTEGER NOT NULL PRIMARY KEY, project_id INTEGER NOT NULL, status BOOLEAN NOT NULL, reviewer TEXT NOT NULL, description TEXT NOT NULL, created_at TIMESTAMP NOT NULL, updated_at TIMESTAMP NOT NULL, FOREIGN KEY (project_id) REFERENCES projects(id));"
    ];
    
    for migration in migrations {
        diesel::sql_query(migration)
            .execute(conn)
            .expect("Error running migration");
    }
}

pub fn check_sqlite_and_run_migration(database_url: &str) {
    let mut conn = SqliteConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    // check if the database is empty
    let table_count = diesel::sql_query("SELECT COUNT(*) FROM sqlite_master WHERE type = 'table'").execute(&mut conn).expect("Error check table ");
    
    if table_count == 0 {
        println!("No table found, running migration");
        texcol_run_migration(&mut conn);
    } else {
        println!("Table found, skipping migration");
    }
}