#[macro_use]
extern crate diesel;

mod models;
mod schema;

use self::models::*;
use self::schema::cats::dsl::*;

use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;
use std::io;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn cats_endpoint(pool: web::Data<DbPool>) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let connection = pool.get().expect("Can't get db connection from pool");
    let cats_data = web::block(move || {
        cats.limit(100).load::<Cat>(&connection)
    })
    .await
    .map_err(|_| HttpResponse::InternalServerError().finish())?;

    Ok(HttpResponse::Ok().json(cats_data))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool.");

    println!("Listing on 127.0.0.1 with port 8080");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(
                web::scope("/api")
                    .route("/cats", web::get().to(cats_endpoint)),
            )
            .service(Files::new("/", "static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
