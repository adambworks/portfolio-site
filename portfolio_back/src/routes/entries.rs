use crate::{
    db::{entries, DBPool},
    models::{NewEntry, UpdateEntry},
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures_util::stream::StreamExt;
use std::io::Write;
use std::fs;
use uuid::Uuid;

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

#[post("/upload")]
pub async fn upload_image(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
    let mut filename = String::new();
    let dir_path = "./assets/images";
    fs::create_dir_all(dir_path).map_err(|e| {
        eprintln!("Error creating directory: {:?}", e);
        actix_web::error::ErrorInternalServerError("Could not create directory for upload")
    })?;

    while let Some(item) = payload.next().await {
        let mut field = item?;
        if let Some(content_disposition) = field.content_disposition() {
            let name = content_disposition.get_name().unwrap_or_default();
    
            if name == "image" {
                let original_filename = content_disposition.get_filename().unwrap_or("file");
                let extension = std::path::Path::new(original_filename)
                    .extension()
                    .and_then(std::ffi::OsStr::to_str)
                    .unwrap_or("");
                filename = format!("{}.{}", Uuid::new_v4(), extension);
                let filepath = format!("{}/{}", dir_path, filename);
                let mut f = web::block(|| std::fs::File::create(filepath)).await??;
    
                while let Some(chunk) = field.next().await {
                    let data = chunk?.to_vec();
                    f = web::block(move || f.write_all(&data).map(|_| f)).await??;
                }
            }
        }
    }
    Ok(HttpResponse::Ok().json(filename))
}