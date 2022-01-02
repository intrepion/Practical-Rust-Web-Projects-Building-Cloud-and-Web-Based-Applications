use actix_files::{Files, NamedFile};
use actix_web::{web, App, HttpServer};
use std::io;

async fn index() -> actix_web::Result<actix_files::NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("Listing on 127.0.0.1 with port 8080");
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
