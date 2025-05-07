pub mod models;
pub mod schema;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;






pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


fn main() {
    use self::schema::projects::dsl::*;

    let connection = &mut establish_connection();
    let results = projects
        .filter(published.eq(true))
        .limit(5)
        .select(Project::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} projects", results.len());
    for project in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}
