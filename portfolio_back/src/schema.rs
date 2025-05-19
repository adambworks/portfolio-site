// @generated automatically by Diesel CLI.

diesel::table! {
    chapters (id) {
        id -> Int4,
        project_id -> Int4,
        name -> Text,
        date_started -> Nullable<Date>,
        index -> Int4,
    }
}

diesel::table! {
    entries (id) {
        id -> Int4,
        chapter_id -> Int4,
        text -> Nullable<Text>,
        image -> Nullable<Text>,
        date -> Nullable<Date>,
        index -> Int4,
    }
}

diesel::table! {
    projects (id) {
        id -> Int4,
        name -> Text,
        date_started -> Date,
        overview -> Nullable<Text>,
        slug -> Text,
        image -> Nullable<Text>,
        chapter_descriptor -> Nullable<Text>,
    }
}

diesel::joinable!(chapters -> projects (project_id));
diesel::joinable!(entries -> chapters (chapter_id));

diesel::allow_tables_to_appear_in_same_query!(
    chapters,
    entries,
    projects,
);
