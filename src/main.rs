extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpServer, Responder};
use anyhow::Result;
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

mod notifications;

#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(u32, String)>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query!("SELECT 1 as one")
        .fetch_one(db_pool.get_ref())
        .await
        .expect("Error executing query");
    format!(
        "Hello {}! id:{} one={}",
        info.1,
        info.0,
        result.one.unwrap()
    )
}

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::new(&database_url).await?;

    let host = env::var("HOST").expect("HOST environment variable is missing");
    let port = env::var("PORT").expect("PORT environment variable is missing");

    info!("Server listening on {}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(db_pool.clone())
            .service(index)
            .configure(notifications::init)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await?;

    Ok(())
}
