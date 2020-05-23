use actix_web::cookie::{Cookie, CookieJar, Key, SameSite};
use std::time::Duration;
use actix_web::dev::{ServiceResponse, ServiceRequest};
use actix_web::error::{Error, Result};
use futures::future::{ok, Ready};
use actix_redis::RedisSession;
use actix_session::UserSession;
use actix_identity::IdentityPolicy;

pub struct RedisSessionPolicy();

impl RedisSessionPolicy {
    pub fn new() -> RedisSessionPolicy{
        RedisSessionPolicy()
    }
}


impl IdentityPolicy for RedisSessionPolicy {
    type Future = Ready<Result<Option<String>, Error>>;
    type ResponseFuture = Ready<Result<(), Error>>;

    fn from_request(&self, req: &mut ServiceRequest) -> Self::Future {
        let session = req.get_session();
        let identity = session.get::<String>("user_id").unwrap_or(None);
        ok({ identity })
    }

    fn to_response<B>(
        &self,
        id: Option<String>,
        changed: bool,
        res: &mut ServiceResponse<B>,
    ) -> Self::ResponseFuture {
        ok(())
    }
}