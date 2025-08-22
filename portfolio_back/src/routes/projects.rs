use crate::{
    db::{projects, DBPool},
    models::{NewProject, UpdateProject},
    routes::logger::log_api,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use uuid::Uuid;

#[get("/list_projects")]
pub(crate) async fn list_projects(pool: web::Data<DBPool>) -> impl Responder {
    let trace_id = Uuid::new_v4();
    log_api(&trace_id, "GET {/list projects}");
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(e) => {
            log::error!("DB pool error: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    match projects::select_all_projects(&mut conn, &trace_id) {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(_) => HttpResponse::InternalServerError().body("Error loading projects"),
    }
}

#[get("/projects/{slug}")]
pub(crate) async fn get_project_by_slug(
    pool: web::Data<DBPool>,
    path: web::Path<String>,
) -> impl Responder {
    let slug = path.into_inner();
    let mut conn = pool.get().expect("couldn't get DB connection");
    match projects::select_project_by_slug(&mut conn, &slug) {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(_) => HttpResponse::InternalServerError().body("Error loading projects"),
    }
}

#[post("/projects")]
pub(crate) async fn create_project(
    pool: web::Data<DBPool>,
    new_project_data: web::Json<NewProject>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get DB connection");
    let new_project = new_project_data.into_inner();

    match projects::create_project(&mut conn, &new_project) {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(_) => HttpResponse::InternalServerError().body("Error creating project"),
    }
}

#[put("/projects/{id}")]
pub(crate) async fn update_project(
    pool: web::Data<DBPool>,
    path: web::Path<i32>,
    project_update: web::Json<UpdateProject>,
) -> impl Responder {
    let id = path.into_inner();
    let mut conn = pool.get().expect("couldn't get DB connection");
    match projects::update_project(&mut conn, id, &project_update) {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(_) => HttpResponse::InternalServerError().body("Error updating project"),
    }
}

#[delete("/projects/{id}")]
pub(crate) async fn delete_project(
    pool: web::Data<DBPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    let mut conn = pool.get().expect("couldn't get DB connection");
    match projects::delete_project(&mut conn, id) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Error deleting project"),
    }
}
