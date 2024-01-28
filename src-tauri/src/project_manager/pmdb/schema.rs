// @generated automatically by Diesel CLI.

diesel::table! {
    projects (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        tex_path -> Text,
        bib_path -> Text,
        proj_path -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    reviews (id) {
        id -> Integer,
        project_id -> Integer,
        status -> Bool,
        reviewer -> Text,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(reviews -> projects (project_id));

diesel::allow_tables_to_appear_in_same_query!(
    projects,
    reviews,
);
