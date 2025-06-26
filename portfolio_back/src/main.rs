pub mod db;
pub mod models;
mod routes;
pub mod schema;

use self::db::init_pool;

use dotenvy::dotenv;
use std::env;

//actix imports
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use actix_files::Files;




async fn spa_fallback() -> impl Responder {
    actix_files::NamedFile::open("./static/index.html")
        .unwrap()
        .use_last_modified(true)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
 
    let pool = init_pool(&database_url);
    println!("🚀 Server is starting...");
    let database_allowed_origin =  env::var("DATABASE_ALLOWED_ORGIN").expect("DATABASE_ALLOWED_ORGIN must be set");
    let bind_ip = env::var("BIND_IP").expect("BIND_IP must be set");
    HttpServer::new(move || {
       // let cors = Cors::default()
         //   .allowed_origin(&database_allowed_origin)
           // .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
           // .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
           // .supports_credentials();
          let cors = Cors::permissive();
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            
            .service(web::scope("/api")
            .wrap(cors)
                .service(self::routes::projects::list_projects)
                .service(self::routes::projects::get_project_by_slug)
                .service(Files::new("/images","./assets/images").show_files_listing())
                .service(self::routes::chapters::get_chapters_by_id)
                .service(self::routes::chapters::get_chapter_by_slug_index)
                .service(self::routes::entries::get_entries_by_id)
                .default_service(web::to(|| async {
                    HttpResponse::NotFound().body("API route not found")
                }))
            )
            .service(Files::new("/", "./static").index_file("index.html"))
          //  .default_service(web::route().to(spa_fallback))
            
    })
    .bind((bind_ip,8080))?
    .run()
    .await
}
