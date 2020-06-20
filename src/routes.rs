//! Place all Actix routes here, multiple route configs can be used and
//! combined.

use crate::handlers::{
    auth::{login, logout},
    health::get_health,
    user::{create_user, delete_user, get_user, get_users, update_user},
};
use crate::middleware::auth::Auth as AuthMiddleware;
use actix_files::Files;
use actix_web::web;
use actix_ratelimit::RedisStore;
use crate::auth::get_ip_rate_limiter;
use crate::config::CONFIG;

pub fn routes(cfg: &mut web::ServiceConfig) {
    //Rate limiting
    let store = RedisStore::connect("redis://".to_string() + &CONFIG.redis_url);
    cfg
        // Healthcheck
        .route("/health", web::get().to(get_health))
        // /api/v1 routes
        .service(
            web::scope("/api/v1")
                // Lock down routes with AUTH Middleware
                .wrap(AuthMiddleware)
                .wrap(get_ip_rate_limiter(&store))
                // AUTH routes
                .service(
                    web::scope("/auth")
                        .route("/login", web::post().to(login))
                        .route("/logout", web::post().to(logout)),
                )
                // USER routes
                .service(
                    web::scope("/user")
                        .route("/{id}", web::get().to(get_user))
                        .route("/{id}", web::put().to(update_user))
                        .route("/{id}", web::delete().to(delete_user))
                        .route("", web::get().to(get_users))
                        .route("", web::post().to(create_user)),
                ),
        )
        .service(
            web::scope("/api/ext/v1")
                .wrap(get_ip_rate_limiter(&store))
                .route("/login", web::post().to(login))
        )
        // Serve secure static files from the static-private folder
        .service(
            web::scope("/secure")
                .wrap(AuthMiddleware)
                .wrap(get_ip_rate_limiter(&store))
                .service(
                Files::new("", "./static-secure")
                    .index_file("index.html")
                    .use_last_modified(true),
            ),
        )
        // Serve public static files from the static folder
        .service(
            web::scope("").default_service(
                Files::new("", "./static")
                    .index_file("index.html")
                    .use_last_modified(true),
            ),
        );
}
