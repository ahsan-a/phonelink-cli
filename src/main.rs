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
            // scuffed way of http/s link and no https link
            .service(routes::link::protocol_url_link)
            .service(routes::link::protocol_link)
            .service(routes::link::no_protocol_link)
    })
    .bind(get_addr())?
    .run()
    .await
}
