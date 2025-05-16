pub mod db;
pub mod models;
mod routes;
pub mod schema;

use self::db::init_pool;

use dotenvy::dotenv;
use std::env;

//actix imports
use actix_web::{web, App, HttpServer};




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = init_pool(&database_url);

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::scope("api")
                .service(self::routes::projects::list_projects)
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
