use actix_web::{get, web, HttpResponse, Responder};
use crate::db::DBPool;


#[get("/list_projects")]
pub(crate) async fn list_projects(pool: web::Data<DBPool>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get DB connection");
    match crate::db::projects::get_all_projects(&mut conn) {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(_) => HttpResponse::InternalServerError().body("Error loading projects"),
    }
}

#[get("/projects/{slug}")]
pub(crate) async fn get_project_by_slug(pool: web::Data<DBPool>,path: web::Path<String>) -> impl Responder {
    let slug = path.into_inner();
    let mut conn = pool.get().expect("couldn't get DB connection");
    match crate::db::projects::find_project_by_slug(&mut conn,&slug) {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(_) => HttpResponse::InternalServerError().body("Error loading projects"),
    }
}
