use env_logger::Env;

mod db;
mod git;
mod utils;
mod web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(
        Env::default()
            .default_filter_or("info")
    );

    git::run_git_server(22).await.unwrap();
    web::run_webserver().await
}
