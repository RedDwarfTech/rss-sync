use actix_web::web;

pub async fn health() -> String {
    "OK".to_string()
}

pub async fn liveness() -> String {
    "OK".to_string()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/rss/health")
            .route("/healthz", web::get().to(health))
            .route("/liveness", web::get().to(liveness)),
    );
}