use crate::{
    models::{self, Chapter, NewChapter, UpdateChapter},
    schema,
};
use diesel::prelude::*;

pub fn create_chapter(
    conn: &mut PgConnection,
    new_chapter: &NewChapter,
) -> Result<Chapter, diesel::result::Error> {
    use crate::schema::chapters::dsl::chapters;
    diesel::insert_into(chapters)
        .values(new_chapter)
        .get_result(conn)
}

pub fn select_chapters_for_project(
    conn: &mut PgConnection,
    project_id_val: i32,
) -> Result<Vec<Chapter>, diesel::result::Error> {
    use crate::schema::chapters::dsl::{chapters, index, project_id};
    chapters
        .filter(project_id.eq(project_id_val))
        .order(index.asc())
        .load(conn)
}

pub fn select_chapter_by_slug_index(
    conn: &mut PgConnection,
    project_slug: &str,
    ch_index: i32,
) -> Result<Chapter, diesel::result::Error> {
    use crate::schema::chapters::dsl::{index, chapters};
    let project: models::Project = schema::projects::table
        .filter(schema::projects::slug.eq(project_slug))
        .first(conn)?;

    Chapter::belonging_to(&project)
        .filter(index.eq(ch_index))
        .first(conn)
}

pub fn update_chapter(
    conn: &mut PgConnection,
    chapter_id: i32,
    chapter_update: &UpdateChapter,
) -> Result<Chapter, diesel::result::Error> {
    use crate::schema::chapters::dsl::chapters;
    diesel::update(chapters.find(chapter_id))
        .set(chapter_update)
        .get_result(conn)
}

pub fn delete_chapter(
    conn: &mut PgConnection,
    chapter_id: i32,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::chapters::dsl::chapters;
    diesel::delete(chapters.find(chapter_id)).execute(conn)
}
