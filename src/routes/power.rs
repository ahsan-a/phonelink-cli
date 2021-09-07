use crate::routes::{check_password, AppState};
use actix_web::{get, web, HttpRequest, Responder};
use system_shutdown::{logout, reboot, shutdown};

#[get("/power/{power}")]
pub async fn power_route(
    web::Path(power): web::Path<String>,
    data: web::Data<AppState>,
    req: HttpRequest,
) -> impl Responder {
    if !check_password(&req, &data).await {
        return "Incorrect password.".to_string();
    }

    match power.to_lowercase().as_str() {
        "shutdown" => match shutdown() {
            Ok(_) => "Shutting down computer.".to_string(),
            Err(error) => return format!("Failed to shut down: {}", error),
        },
        "restart" => match reboot() {
            Ok(_) => "Restarting computer.".to_string(),
            Err(error) => return format!("Failed to restart: {}", error),
        },
        "logout" => match logout() {
            Ok(_) => "Logging out.".to_string(),
            Err(error) => return format!("Failed to log out: {}", error),
        },
        _ => "ERROR: Not a valid shutdown command.".to_string(),
    }
}
