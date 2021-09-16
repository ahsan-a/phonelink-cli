use crate::routes::{check_password, AppState};
use actix_web::{get, web, HttpRequest, Responder};
use webbrowser::open;

#[get("/url/{url:.*}")]
pub async fn link_route(
    web::Path(mut url): web::Path<String>,
    data: web::Data<AppState>,
    req: HttpRequest,
) -> impl Responder {
    if !check_password(&req, &data).await {
        return "Incorrect password.".to_string();
    }

    if !url.contains("://") {
        url = format!("https://{}", url)
    }

    match open(&url) {
        Ok(_) => format!("Successfully opened url {} on your computer.", url),
        Err(e) => format!("Failed to open url {} on your computer: {}", url, e),
    }
}
