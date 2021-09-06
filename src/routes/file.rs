use actix_multipart::Multipart;
use actix_web::{post, web, Responder};
use futures::{StreamExt, TryStreamExt};
use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;

#[post("/file")]
pub async fn file_route(mut payload: Multipart) -> impl Responder {
    // iterate over multipart stream
    let dir_path = "D://Users/Ahsan/Downloads/";
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();

        let filename = content_type.get_filename().unwrap();
        let filext = Path::new(filename)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap();

        let mut filepath = format!("{}{}", dir_path, sanitize_filename::sanitize(&filename));

        if std::path::Path::new(&filepath).exists() {
            let mut i = 0;

            while std::path::Path::new(&filepath).exists() {
                i += 1;
                filepath = format!(
                    "{}{} ({}).{}",
                    dir_path,
                    &filename.replace(&format!(".{}", filext), ""),
                    i,
                    filext
                );
                println!("{}", filepath);
            }
        }

        // what does this do?
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f = web::block(move || f.write_all(&data).map(|_| f))
                .await
                .unwrap();
        }
    }

    "Your file was saved successfully."
}
