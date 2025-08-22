use crate::{
    db::{entries, DBPool},
    models::{NewEntry, UpdateEntry},
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[get("/chapters/{id}/entries")]
pub async fn get_entries_by_id(pool: web::Data<DBPool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let mut conn = pool.get().expect("couldn't get DB connection");
    match entries::select_entries(&mut conn, id) {
        Ok(entries) => HttpResponse::Ok().json(entries),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/entries")]
pub async fn create_entry(
    pool: web::Data<DBPool>,
    new_entry_data: web::Json<NewEntry>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get DB connection");
    let new_entry = new_entry_data.into_inner();

    match entries::create_entry(&mut conn, &new_entry) {
        Ok(entry) => HttpResponse::Ok().json(entry),
        Err(_) => HttpResponse::InternalServerError().body("Error creating entry"),
    }
}

#[put("/entries/{id}")]
pub async fn update_entry(
    pool: web::Data<DBPool>,
    path: web::Path<i32>,
    entry_update: web::Json<UpdateEntry>,
) -> impl Responder {
    let id = path.into_inner();
    let mut conn = pool.get().expect("couldn't get DB connection");
    match entries::update_entry(&mut conn, id, &entry_update) {
        Ok(entry) => HttpResponse::Ok().json(entry),
        Err(_) => HttpResponse::InternalServerError().body("Error updating entry"),
    }
}

#[delete("/entries/{id}")]
pub async fn delete_entry(
    pool: web::Data<DBPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    let mut conn = pool.get().expect("couldn't get DB connection");
    match entries::delete_entry(&mut conn, id) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Error deleting entry"),
    }
}