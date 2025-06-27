use crate::{
    db::loggger::log_db,
    models::{NewProject, Project},
};
use chrono::NaiveDate;
use diesel::prelude::*;
use uuid::Uuid;

fn create_project(
    conn: &mut PgConnection,
    name: &str,
    date_started: &NaiveDate,
    overview: Option<&str>,
    slug: &str,
    image: Option<&str>,
    chapter_descriptor: Option<&str>,
) {
    use crate::schema::projects;

    let new_project = NewProject {
        name,
        date_started,
        overview,
        slug,
        image,
        chapter_descriptor,
    };

    diesel::insert_into(projects::table)
        .values(&new_project)
        .returning(Project::as_returning())
        .get_result(conn)
        .expect("Error saving new post");
}

pub(crate) fn select_project_by_slug(
    conn: &mut PgConnection,
    project_slug: &str,
) -> Result<Project, diesel::result::Error> {
    use crate::schema::projects::dsl::{projects, slug};
    let result = projects.filter(slug.eq(project_slug)).first(conn);
    return result;
}

fn edit_project_overview(conn: &mut PgConnection, project_id: i32, new_overview: &str) {
    use crate::schema::projects::dsl::{overview, projects};

    let project = diesel::update(projects.find(project_id))
        .set(overview.eq(new_overview))
        .returning(Project::as_returning())
        .get_result(conn)
        .unwrap();
    println!("update project overview {}", project.name);
}

pub(crate) fn select_all_projects(
    conn: &mut PgConnection,
    trace_id: &Uuid,
) -> Result<Vec<Project>, diesel::result::Error> {
    use crate::schema::projects::dsl::projects;

    let result = log_db(trace_id, "select_all_projects", || {
        projects.select(Project::as_select()).load(conn)
    });
    return result;
}
