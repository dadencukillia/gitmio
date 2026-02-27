use actix_web::{
    middleware::Logger,
    App, HttpServer
};

mod frontend;
mod api_v1;

pub async fn run_webserver() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(api_v1::config)
            .configure(frontend::config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
