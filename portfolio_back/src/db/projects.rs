use crate::models::{Project,NewProject};
use chrono::NaiveDate;
use diesel::prelude::*;


fn create_project(conn: &mut PgConnection, name: &str, date_started: &NaiveDate, overview: Option<&str>) {
    use crate::schema::projects;

    let new_project = NewProject {
        name,
        date_started,
        overview,
    };

    diesel::insert_into(projects::table)
        .values(&new_project)
        .returning(Project::as_returning())
        .get_result(conn)
        .expect("Error saving new post");
}

fn find_project_id(conn: &mut PgConnection, project_name: &str) -> i32 {
    use crate::schema::projects::dsl::{name, projects};
    let result: Project = projects
        .filter(name.eq(project_name))
        .first(conn)
        .expect("Error loading projects");
    return result.id;
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

pub(crate) fn get_all_projects(conn: &mut PgConnection) -> Result<Vec<Project>, diesel::result::Error> {
    //projects.load::<Project>(conn);
    //atertivative one line way

    use crate::schema::projects::dsl::projects;
    let results = projects
        //  .filter(published.eq(true))
        .select(Project::as_select())
        .load(conn);
    //.expect("Error loading projects");//todo should not use expect
    return results;
}


