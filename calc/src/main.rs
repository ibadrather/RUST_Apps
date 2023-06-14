// Web microservice for calculating mathemtical operations

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("This is a calculator micro-service!")
}

#[get("/add/{a}/{b}")]
async fn add(info: web::Path<(i32, i32)>) -> impl Responder {
    let result = calc::add(info.0, info.1);
    HttpResponse::Ok().body(format!("{} + {} = {}", info.0, info.1, result))
}

#[get("/subtract/{a}/{b}")]
async fn subtract(info: web::Path<(i32, i32)>) -> impl Responder {
    let result = calc::subtract(info.0, info.1);
    HttpResponse::Ok().body(format!("{} - {} = {}", info.0, info.1, result))
}

#[get("/multiply/{a}/{b}")]
async fn multiply(info: web::Path<(i32, i32)>) -> impl Responder {
    let result = calc::multiply(info.0, info.1);
    HttpResponse::Ok().body(format!("{} * {} = {}", info.0, info.1, result))
}

#[get("/divide/{a}/{b}")]
async fn divide(info: web::Path<(i32, i32)>) -> impl Responder {
    let result = calc::divide(info.0, info.1);
    HttpResponse::Ok().body(format!("{} / {} = {}", info.0, info.1, result))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(add)
            .service(subtract)
            .service(multiply)
            .service(divide)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}