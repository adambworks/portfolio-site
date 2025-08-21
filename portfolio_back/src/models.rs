use diesel::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};
use crate::schema::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize,Identifiable)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub date_started: NaiveDate,
    pub overview: Option<String>,
    pub slug: String,
    pub image: Option<String>,
    pub chapter_descriptor: Option<String>
}

#[derive(Queryable, Selectable, Serialize, Associations,Identifiable)]
#[diesel(table_name = chapters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Project))]
pub struct Chapter {
    pub id: i32,
    pub project_id: i32,
    pub name: String,
    pub date_started: Option<NaiveDate>,
    pub index: i32,
}

#[derive(Queryable, Selectable, Serialize, Associations,Identifiable)]
#[diesel(table_name = entries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Chapter))]
pub struct Entry {
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
#[derive(Insertable)]
#[diesel(table_name = projects)]
pub struct NewProject<'a>{
    pub name: &'a str,
    pub date_started: &'a NaiveDate,
    pub overview: Option<&'a str>,
    pub slug: &'a str,
    pub image: Option<&'a str>,
    pub chapter_descriptor: Option<&'a str>

}

#[derive(Insertable)]
#[diesel(table_name = chapters)]
pub struct NewChapter<'a> {
    pub project_id: i32,
    pub name: &'a str,
    pub date_started: Option<NaiveDate>,
    pub index: &'a i32,
}

#[derive(Insertable)]
#[diesel(table_name = entries)]
pub struct NewEntry<'a> {
    pub chapter_id: i32,
    pub text: &'a str,
    pub image: Option<&'a str>,
    pub date: Option<NaiveDate>,
    pub index: &'a i32
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
}
