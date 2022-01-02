use actix_files::Files;
use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer};
use handlebars::Handlebars;
use std::io;

async fn index(hb: web::Data<handlebars::Handlebars<'_>>) -> actix_web::HttpResponse {
    let data = serde_json::json!({
        "project_name": "Catdex",
        "cats": [
            {
                "name": "British short hair",
                "image_path": "/static/image/british-short-hair.jpg"
            },
            {
                "name": "Persian",
                "image_path": "/static/image/persian.jpg"
            },
            {
                "name": "Ragdoll",
                "image_path": "/static/image/ragdoll.jpg"
            },
        ]
    });

    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".hbs", "./static/")
        .unwrap();
    let handlebars_ref = Data::new(handlebars);

    println!("Listing on 127.0.0.1 with port 8080");
    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
