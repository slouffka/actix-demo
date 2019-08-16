use actix_web::{web, App, HttpRequest, HttpServer, Responder};

fn hello() -> impl Responder {
    format!("Hello World!")
}

fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:3000")
    .expect("Can not bind to port 3000")
    .run()
    .unwrap();
}
