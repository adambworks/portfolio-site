use crate::models::{self, Chapter};
use crate::schema::{self};
use diesel::prelude::*;
use crate::schema::chapters::dsl::*;

pub fn select_chapters_for_project(conn: &mut PgConnection, project_id_val: i32,) -> Result<Vec<Chapter>, diesel::result::Error> {

    return chapters
        .filter(project_id.eq(project_id_val))
        .order(index.asc()) 
        .load(conn);
}

pub fn select_chapter_by_slug_index(conn: &mut PgConnection,project_slug: &str,ch_index: i32) -> Result<Chapter,diesel::result::Error>{
    let project = schema::projects::table.filter(schema::projects::slug.eq(project_slug))
    .select(models::Project::as_select())
    .get_results(conn)?;

    return  Chapter::belonging_to(&project)
    .filter(index.eq(ch_index))
   // .select(Chapter::as_select)
    //.load(conn);
    .first(conn);
}
