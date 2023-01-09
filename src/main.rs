use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use std::io;

const PORT: u16 = 8080;

async fn greet(req: HttpRequest) -> impl Responder {
    let greeting = req.match_info().get("name").unwrap_or("World");
    let result = "Hello, ".to_owned() + greeting;
    HttpResponse::Ok().body(result)
}

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("localhost", PORT))?
    .run()
    .await
}
