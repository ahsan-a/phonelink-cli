use actix_web::{get, HttpRequest, Responder};
use notify_rust::Notification;

#[get("/notification")]
pub async fn notif(req: HttpRequest) -> impl Responder {
    let title = match req.headers().get("title") {
        None => return "An error occurred.",
        Some(x) => x.to_str().unwrap(),
    };

    match Notification::new()
        .summary(title)
        .body(match req.headers().get("body") {
            None => "",
            Some(y) => y.to_str().unwrap(),
        })
        .appname("phonelink-rs")
        .auto_icon()
        .show()
    {
        Ok(_) => return "Succesfully sent a notification to your computer.",
        Err(_) => return "An error occurred while sending a notification to your computer.",
    };
}
