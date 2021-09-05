use actix_web::{get, web, Responder};
use webbrowser;

#[get("/url/{protocol}//{url}")]
pub async fn protocol_url_link(
    web::Path((protocol, url)): web::Path<(String, String)>,
) -> impl Responder {
    open_url(format!("{}//{}", protocol, url))
}

#[get("/url/{protocol}//")]
pub async fn protocol_link(web::Path(protocol): web::Path<String>) -> impl Responder {
    open_url(format!("{}//", protocol))
}

#[get("/url/{url}")]
pub async fn no_protocol_link(web::Path(url): web::Path<String>) -> impl Responder {
    open_url(format!("{}{}", "http://", url))
}

fn open_url(url: String) -> impl Responder {
    match webbrowser::open(&url) {
        Ok(_) => format!("Successfully opened url {} on your computer.", url),
        Err(_) => format!("Failed to open url {} on your computer.", url),
    }
}
