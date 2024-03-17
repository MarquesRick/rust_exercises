mod model;
mod schema;
mod services;

use actix_web::{middleware::Logger, web, App, HttpServer};

use dotenv::dotenv;

use sqlx::{
    database,
    postgres::{PgPool, PgPoolOptions},
    Pool, Postgres,
};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server is running! 🚀");

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info")
    }

    dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must to be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection DB resolved! 🚀");
            pool
        }
        Err(error) => {
            println!("Failed to connect to database 🚨: {:?}", error);
            std::process::exit(1)
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(services::config)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
