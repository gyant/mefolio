use actix_web::{get, web, App, HttpServer};
use askama_actix::Template;
use actix_files as fs;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> HelloTemplate {
    HelloTemplate {
        name: name.into_inner(),
    }
}

#[get("/")]
async fn index() -> IndexTemplate {
    IndexTemplate
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on port 8080");
    HttpServer::new(|| App::new().service(greet).service(index).service(fs::Files::new("/static", "./static")))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
