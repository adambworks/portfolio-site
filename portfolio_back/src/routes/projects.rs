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
