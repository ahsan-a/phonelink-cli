use crate::routes;
use actix_web::{web, HttpRequest};

pub async fn check_password(req: &HttpRequest, data: &web::Data<routes::AppState>) -> bool {
    let enabled: bool = data.config.get("enable_password").unwrap().parse().unwrap();
    let password = data.config.get("password").unwrap();

    if !enabled {
        return true;
    }

    let header = match req.headers().get("password") {
        Some(x) => x.to_str().unwrap().to_string(),
        None => return false,
    };

    if password == &header {
        return true;
    }
    return false;
}
