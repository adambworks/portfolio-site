use crate::{
    db::loggger::log_db,
    models::{NewProject, Project, UpdateProject},
    schema::projects::dsl::projects,
};
use diesel::prelude::*;
use uuid::Uuid;

pub fn create_project(
    conn: &mut PgConnection,
    new_project: &NewProject,
) -> Result<Project, diesel::result::Error> {
    diesel::insert_into(projects)
        .values(new_project)
        .get_result(conn)
}

pub(crate) fn select_project_by_slug(
    conn: &mut PgConnection,
    project_slug: &str,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::projects::dsl::slug;
    projects.filter(slug.eq(project_slug)).first(conn)
}

pub fn update_project(
    conn: &mut PgConnection,
    project_id: i32,
    project_update: &UpdateProject,
) -> Result<Project, diesel::result::Error> {
    diesel::update(projects.find(project_id))
        .set(project_update)
        .get_result(conn)
}

pub fn delete_project(
    conn: &mut PgConnection,
    project_id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(projects.find(project_id)).execute(conn)
}

pub(crate) fn select_all_projects(
    conn: &mut PgConnection,
    trace_id: &Uuid,
) -> Result<Vec<Project>, diesel::result::Error> {
    log_db(trace_id, "select_all_projects", || {
        projects.select(Project::as_select()).load(conn)
    })
}
