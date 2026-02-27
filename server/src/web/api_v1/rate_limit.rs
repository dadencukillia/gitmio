use std::time::Duration;
use actix_extensible_rate_limit::{RateLimiter, backend::{SimpleInputFunctionBuilder, memory::InMemoryBackend}};

pub fn middleware() -> RateLimiter<
    InMemoryBackend,
    actix_extensible_rate_limit::backend::SimpleOutput,
    impl Fn(&actix_web::dev::ServiceRequest) -> std::future::Ready<Result<actix_extensible_rate_limit::backend::SimpleInput, actix_web::Error>> + 'static
> {
    let backend = InMemoryBackend::builder().build();

    let input = SimpleInputFunctionBuilder::new(Duration::from_secs(60), 100)
        .peer_ip_key()
        .build();

    let middleware = RateLimiter::builder(backend, input)
        .add_headers()
        .build();

    middleware
}
