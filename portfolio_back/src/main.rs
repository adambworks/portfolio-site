pub mod db;
pub mod models;
mod routes;
pub mod schema;

use self::db::init_pool;

use dotenvy::dotenv;
use std::env;

//actix imports
use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer,Responder};
use actix_files::Files;



fn print_static_files() {
    match std::fs::read_dir("./static") {
        Ok(entries) => {
            println!("Contents of ./static:");
            for entry in entries {
                match entry {
                    Ok(entry) => println!(" - {:?}", entry.path()),
                    Err(e) => println!(" - Error reading entry: {}", e),
                }
            }
        }
        Err(e) => {
            println!("Failed to read ./static: {}", e);
        }
    }
}

async fn spa_fallback() -> impl Responder {
    actix_files::NamedFile::open("./static/index.html")
        .unwrap()
        .use_last_modified(true)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Current working directory: {:?}", std::env::current_dir()?);

    println!("database_url is: {}", database_url);
    println!("Current working directory: {:?}", std::env::current_dir()?);
    let pool = init_pool(&database_url);
    println!("🚀 Server is starting...");
    print_static_files();
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("*")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
            .supports_credentials();
        App::new()
           // .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("api")
                .service(self::routes::projects::list_projects)
                .service(self::routes::projects::get_project_by_slug)
                .service(Files::new("/images","./static/images").show_files_listing())
                .service(self::routes::chapters::get_chapters_by_id)
                .service(self::routes::chapters::get_chapter_by_slug_index)
                .service(self::routes::entries::get_entries_by_id)
            )
            .service(Files::new("/", "./static").index_file("index.html"))
            .default_service(web::route().to(spa_fallback))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
