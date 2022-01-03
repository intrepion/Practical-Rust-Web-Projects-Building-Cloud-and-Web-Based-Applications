#[macro_use]
extern crate diesel;

mod errors;
mod models;
mod schema;

use self::errors::*;
use self::models::*;
use self::schema::cats::dsl::*;

use actix_files::Files;
use actix_web::web::PathConfig;
use actix_web::{error, web, App, HttpResponse, HttpServer};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;
use std::io;
use validator::Validate;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(serde::Deserialize, validator_derive::Validate)]
struct CatEndpointPath {
    #[validate(range(min = 1, max = 150))]
    id: i32,
}

fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .app_data(PathConfig::default().error_handler(|_, _| UserError::ValidationError.into()))
            .route("/cats", web::get().to(cats_endpoint))
            .route("/cat/{id}", web::get().to(cat_endpoint)),
    );
}

async fn cat_endpoint(
    pool: web::Data<DbPool>,
    cat_id: web::Path<CatEndpointPath>,
) -> Result<actix_web::HttpResponse, UserError> {
    cat_id.validate().map_err(|_| UserError::ValidationError)?;

    let connection = pool.get().map_err(|_| UserError::DBPoolGetError)?;

    let query_id = cat_id.id.clone();
    let cat_data = web::block(move || cats.filter(id.eq(query_id)).first::<Cat>(&connection))
        .await
        .map_err(|e| match e {
            error::BlockingError::Error(diesel::result::Error::NotFound) => {
                UserError::NotFoundError
            }
            _ => UserError::UnexpectedError,
        })?;

    Ok(HttpResponse::Ok().json(cat_data))
}

async fn cats_endpoint(
    pool: web::Data<DbPool>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let connection = pool.get().expect("Can't get db connection from pool");
    let cats_data = web::block(move || cats.limit(100).load::<Cat>(&connection))
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    Ok(HttpResponse::Ok().json(cats_data))
}

fn setup_database() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool.")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let pool = setup_database();

    println!("Listing on 127.0.0.1 with port 8080");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(api_config)
            .service(Files::new("/", "static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_cats_endpoint_get() {
        let pool = setup_database();
        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .route("/api/cats", web::get().to(cats_endpoint)),
        )
        .await;
        let req = test::TestRequest::get().uri("/api/cats").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }
}
