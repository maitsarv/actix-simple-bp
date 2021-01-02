//! Spin up a HTTPServer

use crate::auth::{get_session_service, get_identity_service};
use crate::server_helpers::cache::add_cache;
use crate::server_helpers::state::new_state;
use crate::config::CONFIG;
use crate::database::connection::add_pool;
use crate::routes::routes;
use futures::future;
use actix_cors::Cors;
use actix_web::web;
use actix_web::{middleware::Logger, App, HttpServer};
use listenfd::ListenFd;
use actix_web_middleware_redirect_https::RedirectHTTPS;
use crate::config::tls;
use crate::middleware::redis_identity::RedisSessionPolicy;

pub async fn server() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // Create the application state
    // String is used here, but it can be anything
    // Invoke in hanlders using data: AppState<'_, String>
    let data = new_state::<String>();

    let secure_addr = format!("{}:{}", CONFIG.server, CONFIG.secure_port);
    let unsecure_addr = format!("{}:{}", CONFIG.server, CONFIG.port);

    let tlsconfig = tls::load_ssl_keys();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .configure(add_cache)
            .wrap(Cors::new().allowed_methods(vec!["GET", "POST"]).supports_credentials().finish())
            .wrap(Logger::default())
            .wrap(get_identity_service(RedisSessionPolicy::new()))
            .wrap(get_session_service())
            .configure(add_pool)
            .app_data(data.clone())
            .configure(routes)
    });

    let mut server_unsecure = HttpServer::new(move || {
        App::new()
            .wrap(RedirectHTTPS::with_replacements(&[((":".to_string()+&CONFIG.port).to_owned(), (":".to_string()+&CONFIG.secure_port).to_owned())]))
            .route("/", web::get().to(|| web::HttpResponse::Ok()
                .content_type("text/plain")
                .body("Always HTTPS!")))

    });

    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen_rustls(l,tlsconfig)?
    } else {
        server.bind_rustls(&secure_addr,tlsconfig)?
    };
    server_unsecure = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server_unsecure.listen(l)?
    } else {
        server_unsecure.bind(&unsecure_addr)?
    };
    server_unsecure = server_unsecure.workers(1);

    let s1_future = server.run();
    let s2_future = server_unsecure.run();
    future::try_join(s1_future, s2_future).await?;
    Ok(())
}
