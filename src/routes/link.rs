use actix_web::{get, web, Responder};
use webbrowser::open;

#[get("/url/{url:.*}")]
pub async fn link_route(web::Path(mut url): web::Path<String>) -> impl Responder {
    if !url.contains("://") {
        url = format!("https://{}", url)
    }

    open_url(url)
}

fn open_url(url: String) -> impl Responder {
    match open(&url) {
        Ok(_) => format!("Successfully opened url {} on your computer.", url),
        Err(_) => format!("Failed to open url {} on your computer.", url),
    }
}
