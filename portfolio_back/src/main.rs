pub mod db;
pub mod models;
pub mod schema;

use self::db::{init_pool, DBPool};
use self::models::*;
use chrono::NaiveDate;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

//actix imports
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// probably move these somewhere else
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn create_project(
    conn: &mut PgConnection,
    name: &str,
    date_started: &NaiveDate,
    overview: Option<&str>,
) {
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
    use self::schema::projects::dsl::{name, projects};
    let result: Project = projects
        .filter(name.eq(project_name))
        .first(conn)
        .expect("Error loading projects");
    return result.id;
}

fn edit_project_overview(conn: &mut PgConnection, project_id: i32, new_overview: &str) {
    use self::schema::projects::dsl::{overview, projects};

    let project = diesel::update(projects.find(project_id))
        .set(overview.eq(new_overview))
        .returning(Project::as_returning())
        .get_result(conn)
        .unwrap();
    println!("update project overview {}", project.name);
}

fn get_all_projects(conn: &mut PgConnection) -> Result<Vec<Project>, diesel::result::Error> {
    //projects.load::<Project>(conn);
    //atertivative one line way

    use self::schema::projects::dsl::projects;
    let results = projects
        //  .filter(published.eq(true))
        .select(Project::as_select())
        .load(conn);
    //.expect("Error loading projects");//todo should not use expect
    return results;
}

fn main_testing_diesel() {
    let connection = &mut establish_connection();

    //let test_date=NaiveDate::from_ymd_opt(2025,5,1);
    // let test_overview = Some("test_overview");
    // create_project(connection, "test", &test_date.unwrap_or_else(|| {panic!()}), test_overview);
}

//actix land

#[get("/Hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/list_projects")]
async fn list_projects(pool: web::Data<DBPool>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get DB connection");
    match get_all_projects(&mut conn) {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(_) => HttpResponse::InternalServerError().body("Error loading projects"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = init_pool(&database_url);

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::scope("api")
                .service(list_projects)
                .service(hello)
                .service(echo)
                .route("/hey", web::get().to(manual_hello)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
