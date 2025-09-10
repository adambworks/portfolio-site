// Modules for database, models, routes, and schema
pub mod db;
pub mod models;
mod routes;
pub mod schema;

// Use statements for database pool initialization
use self::db::init_pool;

// Use statements for environment variables
use dotenvy::dotenv;
use std::env;

// Use statements for Actix web framework
use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer, Responder};
use actix_files::Files;
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::routes::auth_middleware::validator;

/// Fallback for Single Page Application (SPA)
/// This function serves the `index.html` file for any route that is not matched by other handlers.
/// This is essential for SPAs where routing is handled on the client-side.
async fn spa_fallback() -> impl Responder {
    actix_files::NamedFile::open("./static/index.html")
        .unwrap()
        .use_last_modified(true)
}

/// Main function for the Actix web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init();

    // Load environment variables from .env file
    dotenv().ok();

    // Get the database URL from environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
 
    // Initialize the database connection pool
    let pool = init_pool(&database_url);
    println!("🚀 Server is starting...");
    // Get the allowed origin for CORS from environment variables
    let database_allowed_origin =  env::var("DATABASE_ALLOWED_ORGIN").expect("DATABASE_ALLOWED_ORGIN must be set");
    // Get the IP to bind the server to from environment variables
    let bind_ip = env::var("BIND_IP").expect("BIND_IP must be set");

    // Create and configure the HTTP server
    HttpServer::new(move || {
        // Create the authentication middleware
        let auth_middleware = HttpAuthentication::bearer(validator);

        App::new()
            // Configure Cross-Origin Resource Sharing (CORS)
            .wrap(
                Cors::default()
                    .allowed_origin(&database_allowed_origin)
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
                    .supports_credentials()
            )
            // Enable request logging
            .wrap(Logger::default())
            // Add the database pool to the application data
            .app_data(web::Data::new(pool.clone()))
            // Define the admin API scope, protected by authentication
              .service(web::scope("/api/admin")
                .wrap(auth_middleware)
                .service(routes::projects::create_project)
                .service(routes::projects::update_project)
                .service(routes::projects::delete_project)
                .service(routes::chapters::create_chapter)
                .service(routes::chapters::update_chapter)
                .service(routes::chapters::delete_chapter)
                .service(routes::entries::create_entry)
                .service(routes::entries::update_entry)
                .service(routes::entries::delete_entry)
                .service(routes::entries::upload_image)
            )
            // Define the public API scope
            .service(web::scope("/api")
                .service(routes::projects::list_projects)
                .service(routes::projects::get_project_by_slug)
                .service(Files::new("/images","./assets/images"))
                .service(routes::chapters::get_chapters_by_id)
                .service(routes::chapters::get_chapter_by_slug_index)
                .service(routes::entries::get_entries_by_id)
                //.route("/register", web::post().to(routes::auth::register))
                .route("/login", web::post().to(routes::auth::login))
            )
          
            // Set the default service to the SPA fallback
            .default_service(web::route().to(spa_fallback))
            // Serve static files from the "./static" directory
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind((bind_ip, 8080))?
    // Run the server
    .run()
    .await
}
