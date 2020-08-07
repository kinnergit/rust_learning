mod my_json;

use crate::my_json::Person;
use actix_web::{get, post, web, App, HttpServer, Responder};

#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[post("/json")]
async fn json(_: web::Path<()>) -> impl Responder {
    let p = Person::new();

    match p.to_json() {
        Ok(json) => format!("{}", json),
        Err(e) => format!("{}", e),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(json))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
