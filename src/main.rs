use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[get("/")]
fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[get("/hello")]
fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Lowrd!")
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(hello)
    })
    .bind("0.0.0.0:3000")
    .unwrap()
    .run()
    .unwrap();
}
