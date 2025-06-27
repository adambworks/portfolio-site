use crate::{db::DBPool, routes::logger::log_api};
use actix_web::{get, web, HttpResponse, Responder};
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

    return match crate::db::projects::select_all_projects(&mut conn, &trace_id) {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(_) => HttpResponse::InternalServerError().body("Error loading projects"),
    };
}

#[get("/projects/{slug}")]
pub(crate) async fn get_project_by_slug(
    pool: web::Data<DBPool>,
    path: web::Path<String>,
) -> impl Responder {
    let slug = path.into_inner();
    let mut conn = pool.get().expect("couldn't get DB connection");
    match crate::db::projects::select_project_by_slug(&mut conn, &slug) {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(_) => HttpResponse::InternalServerError().body("Error loading projects"),
    }
}
