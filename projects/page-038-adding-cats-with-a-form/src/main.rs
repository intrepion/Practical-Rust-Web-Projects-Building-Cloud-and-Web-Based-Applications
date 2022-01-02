#[macro_use]
extern crate diesel;

mod models;
mod schema;

use self::models::*;
use self::schema::cats::dsl::*;
use actix_files::Files;
use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use handlebars::Handlebars;
use serde::Serialize;
use std::env;
use std::io;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Serialize)]
struct IndexTemplateData {
    project_name: String,
    cats: Vec<self::models::Cat>,
}

async fn index(
    hb: web::Data<handlebars::Handlebars<'_>>,
    pool: web::Data<DbPool>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let connection = pool.get().expect("Can't get db connection from pool");
    let cats_data = web::block(move || cats.limit(100).load::<Cat>(&connection))
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    let data = IndexTemplateData {
        project_name: "Catdex".to_string(),
        cats: cats_data,
    };

    let body = hb.render("index", &data).unwrap();

    Ok(HttpResponse::Ok().body(body))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".hbs", "./static/")
        .unwrap();
    let handlebars_ref = Data::new(handlebars);

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool.");

    println!("Listing on 127.0.0.1 with port 8080");
    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .data(pool.clone())
            .service(Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
