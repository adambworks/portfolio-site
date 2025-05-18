use actix_web::{get, web, HttpResponse, Responder};
use crate::db::{chapters::{select_chapter_by_slug_index, select_chapters_for_project}, DBPool};


#[get("/projects/{id}/chapters")]
async fn get_chapters_by_id(pool: web::Data<DBPool>, path: web::Path<i32> ) -> impl Responder {
    let id: i32 = path.into_inner();
    let mut conn = pool.get().expect("couldn't get DB connection");
    match select_chapters_for_project(&mut conn, id) {
        Ok(chapters) => HttpResponse::Ok().json(chapters),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/projects/{slug}/chapter/{index}")]
async fn get_chapter_by_slug_index(pool: web::Data<DBPool>, path: web::Path<(String,i32)> ) -> impl Responder {
    let(project_slug,ch_index) = path.into_inner();
        let mut conn = pool.get().expect("couldn't get DB connection");
    match select_chapter_by_slug_index(&mut conn, &project_slug, ch_index) {
        Ok(chapter) => HttpResponse::Ok().json(chapter),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
