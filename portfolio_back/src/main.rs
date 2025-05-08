pub mod models;
pub mod schema;
use chrono::NaiveDate;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::*;





pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}





pub fn create_project(conn: &mut PgConnection, name: &str,date_started: &NaiveDate, overview: Option<&str>) -> Project {
    use crate::schema::projects;

    let new_project = NewProject { name,date_started, overview };

    diesel::insert_into(projects::table)
        .values(&new_project)
        .returning(Project::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

fn main() {
    use self::schema::projects::dsl::*;

    let connection = &mut establish_connection();

    //let test_date=NaiveDate::from_ymd_opt(2025,5,1);
   // let test_overview = Some("test_overview");
   // create_project(connection, "test", &test_date.unwrap_or_else(|| {panic!()}), test_overview);



    let results = projects
      //  .filter(published.eq(true))
        .limit(5)
        .select(Project::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} projects", results.len());
    for project in results {
        println!("{}", project.name);
        println!("-----------\n");
        println!("{:?}", Some(project.overview));
    }
}
