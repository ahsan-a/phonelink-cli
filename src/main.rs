mod config;
mod routes;

use actix_web::{get, middleware, App, HttpServer, Responder};
use preferences::{Preferences, PreferencesMap};

#[get("/")]
async fn index() -> impl Responder {
    "phonelink-rs is up and running!"
}

fn get_addr(port: String) -> String {
    println!("phonelink-rs has started up!");
    format!("127.0.0.1:{}", port)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // check if config argument is called
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && &args[1].to_lowercase() == "config" {
        config::config_menu();
        return Ok(());
    }

    config::config_check();

    let user_config =
        PreferencesMap::<String>::load(&config::APP_INFO, whoami::username()).unwrap();

    let port = match user_config.get("port") {
        Some(x) => x.to_string(),
        None => "1234".to_string(),
    };

    HttpServer::new(move || {
        App::new()
            .data(routes::AppState {
                config: PreferencesMap::<String>::load(&config::APP_INFO, whoami::username())
                    .unwrap(),
            })
            .service(index)
            .service(routes::link_route)
            .service(routes::notif)
            .service(routes::file_route)
            .wrap(middleware::Logger::default())
    })
    .bind(format!(":{}", port))?
    .bind(get_addr(port))?
    .run()
    .await
}
