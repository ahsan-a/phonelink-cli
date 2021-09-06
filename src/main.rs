mod routes;

use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    "phonelink-rs is up and running!"
}

fn get_addr() -> &'static str {
    println!("phonelink-rs has started up!");
    "127.0.0.1:1234"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(routes::link_route)
            .service(routes::notif)
            .service(routes::file_route)
    })
    .bind(":1234")?
    .bind(get_addr())?
    .run()
    .await
}
