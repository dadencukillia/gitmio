use std::path::Path;
use actix_web::{
    HttpResponse, Responder, get, web,
    http::header::{
        CacheControl, CacheDirective
    }
};
use chrono::Duration;
use rust_embed::Embed as RustEmbed;

#[derive(RustEmbed)]
#[folder = "client-static/"]
struct Assets;

#[get("/{_:.*}")]
async fn frontend_service(path: web::Path<String>) -> impl Responder {
    let path_str = if path.is_empty() {
        "index.html"
    } else {
        path.as_str()
    };

    match Assets::get(path_str) {
        Some(file) => {
            let cache_control = if path_str == "index.html" {
                CacheControl(vec![
                    CacheDirective::NoCache
                ])
            } else {
                CacheControl(vec![
                    CacheDirective::Public,
                    CacheDirective::MaxAge(Duration::days(3).num_seconds() as u32)
                ])
            };

            let content_type = mime_guess::from_path(Path::new(path_str))
                .first_or_text_plain()
                .to_string();

            let data = file.data;

            HttpResponse::Ok()
                .content_type(content_type)
                .insert_header(cache_control)
                .body(data)
        },
        None => {
            let content_type = mime_guess::mime::TEXT_HTML.as_ref();
            let data = Assets::get("index.html")
                .expect("Please, generate frontend bandle first")
                .data;

            HttpResponse::Ok()
                .content_type(content_type)
                .body(data)
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(frontend_service);
}
