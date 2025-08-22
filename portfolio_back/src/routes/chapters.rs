use crate::{
    db::{chapters, DBPool},
    models::{NewChapter, UpdateChapter},
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[get("/projects/{id}/chapters")]
pub async fn get_chapters_by_id(pool: web::Data<DBPool>, path: web::Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();
    let mut conn = pool.get().expect("couldn't get DB connection");
    match chapters::select_chapters_for_project(&mut conn, id) {
        Ok(chapters) => HttpResponse::Ok().json(chapters),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/projects/{slug}/chapter/{index}")]
pub async fn get_chapter_by_slug_index(
    pool: web::Data<DBPool>,
    path: web::Path<(String, i32)>,
) -> impl Responder {
    let (project_slug, ch_index) = path.into_inner();
    let mut conn = pool.get().expect("couldn't get DB connection");
    match chapters::select_chapter_by_slug_index(&mut conn, &project_slug, ch_index) {
        Ok(chapter) => HttpResponse::Ok().json(chapter),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/chapters")]
pub async fn create_chapter(
    pool: web::Data<DBPool>,
    new_chapter_data: web::Json<NewChapter>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get DB connection");
    let new_chapter = new_chapter_data.into_inner();

    match chapters::create_chapter(&mut conn, &new_chapter) {
        Ok(chapter) => HttpResponse::Ok().json(chapter),
        Err(_) => HttpResponse::InternalServerError().body("Error creating chapter"),
    }
}

#[put("/chapters/{id}")]
pub async fn update_chapter(
    pool: web::Data<DBPool>,
    path: web::Path<i32>,
    chapter_update: web::Json<UpdateChapter>,
) -> impl Responder {
    let id = path.into_inner();
    let mut conn = pool.get().expect("couldn't get DB connection");
    match chapters::update_chapter(&mut conn, id, &chapter_update) {
        Ok(chapter) => HttpResponse::Ok().json(chapter),
        Err(_) => HttpResponse::InternalServerError().body("Error updating chapter"),
    }
}

#[delete("/chapters/{id}")]
pub async fn delete_chapter(
    pool: web::Data<DBPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    let mut conn = pool.get().expect("couldn't get DB connection");
    match chapters::delete_chapter(&mut conn, id) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Error deleting chapter"),
    }
}
