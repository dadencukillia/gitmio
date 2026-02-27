use actix_web::web;

mod controllers;
mod dto;
mod services;

mod rate_limit;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .wrap(rate_limit::middleware())
    );
}
