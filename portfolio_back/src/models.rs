use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub date_started: NaiveDate,
    pub overview: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::chapters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Chapter {
    pub id: i32,
    pub project_id: i32,
    pub name: String,
    pub date_range: Option<pg_range::Range<NaiveDate>>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::entries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Entry {
    pub id: i32,
    pub chapter_id: i32,
    pub text: Option<String>,
    pub image: Option<String>,
    pub date: Option<NaiveDate>,
}