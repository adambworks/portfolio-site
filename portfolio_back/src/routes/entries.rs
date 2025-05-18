use actix_web::{get, web, HttpResponse, Responder};
use crate::db::{entries::select_entries, DBPool};


#[get("/chapters/{id}/entries")]
async fn get_entries_by_id(pool: web::Data<DBPool>, path: web::Path<i32>, ) -> impl Responder {
    let id = path.into_inner();
    let mut conn = pool.get().expect("couldn't get DB connection");
    match select_entries(&mut conn,id){
        Ok(entries) => HttpResponse::Ok().json(entries),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
    