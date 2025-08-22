use diesel::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};
use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Project {
    #[serde(default)]
    pub id: i32,
    pub name: String,
    pub date_started: NaiveDate,
    pub overview: Option<String>,
    pub slug: String,
    pub image: Option<String>,
    pub chapter_descriptor: Option<String>
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Associations, Identifiable)]
#[diesel(table_name = chapters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Project))]
pub struct Chapter {
    #[serde(default)]
    pub id: i32,
    pub project_id: i32,
    pub name: String,
    pub date_started: Option<NaiveDate>,
    pub index: i32,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Associations, Identifiable)]
#[diesel(table_name = entries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Chapter))]
pub struct Entry {
    #[serde(default)]
    pub id: i32,
    pub chapter_id: i32,
    pub text: Option<String>,
    pub image: Option<String>,
    pub date: Option<NaiveDate>,
    pub index: i32
}

#[derive(Queryable, Selectable, Serialize, Identifiable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

//Insertion tables
#[derive(Insertable, Deserialize)]
#[diesel(table_name = projects)]
pub struct NewProject {
    pub name: String,
    pub date_started: NaiveDate,
    pub overview: Option<String>,
    pub slug: String,
    pub image: Option<String>,
    pub chapter_descriptor: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = chapters)]
pub struct NewChapter {
    pub project_id: i32,
    pub name: String,
    pub date_started: Option<NaiveDate>,
    pub index: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = entries)]
pub struct NewEntry {
    pub chapter_id: i32,
    pub text: Option<String>,
    pub image: Option<String>,
    pub date: Option<NaiveDate>,
    pub index: i32,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
}

// Structs for deserializing request bodies for updates
#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = projects)]
pub struct UpdateProject {
    pub name: Option<String>,
    pub date_started: Option<NaiveDate>,
    pub overview: Option<String>,
    pub slug: Option<String>,
    pub image: Option<String>,
    pub chapter_descriptor: Option<String>,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = chapters)]
pub struct UpdateChapter {
    pub name: Option<String>,
    pub date_started: Option<NaiveDate>,
    pub index: Option<i32>,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = entries)]
pub struct UpdateEntry {
    pub text: Option<String>,
    pub image: Option<String>,
    pub date: Option<NaiveDate>,
    pub index: Option<i32>,
}
