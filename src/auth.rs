use crate::config::CONFIG;
use crate::server_helpers::errors::ApiError;
use actix_redis::RedisSession;
use argon2rs::argon2i_simple;
use chrono::{Duration, Utc};
use std::time::Duration as TimeDuration;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PrivateClaim {
    pub user_id: Uuid,
    pub email: String,
    exp: i64,
}

impl PrivateClaim {
    pub fn new(user_id: Uuid, email: String) -> Self {
        Self {
            user_id,
            email,
            exp: (Utc::now() + Duration::hours(CONFIG.jwt_expiration)).timestamp(),
        }
    }
}

/// Create a json web token (JWT)
pub fn create_jwt(private_claim: PrivateClaim) -> Result<String, ApiError> {
    let encoding_key = EncodingKey::from_secret(&CONFIG.jwt_key.as_ref());
    encode(
        &Header::default(),
        &private_claim,
        &encoding_key,
    )
    .map_err(|e| ApiError::CannotEncodeJwtToken(e.to_string()))
}

/// Decode a json web token (JWT)
pub fn decode_jwt(token: &str) -> Result<PrivateClaim, ApiError> {
    let decoding_key = DecodingKey::from_secret(&CONFIG.jwt_key.as_ref());
    decode::<PrivateClaim>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| ApiError::CannotDecodeJwtToken(e.to_string()))
}

/// Encrypt a password
///
/// Uses the argon2i algorithm.
/// auth_salt is environment-configured.
pub fn hash(password: &str, salt: &String) -> String {
    let masked = mask_str(&salt, &CONFIG.auth_salt);
    argon2i_simple(&password, &masked)
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

// WIZ_OPT: IP rate limiter
use actix_ratelimit::{RateLimiter, RedisStoreActor, RedisStore};
use actix::Addr;
pub fn get_ip_rate_limiter(store: &Addr<RedisStore>) -> RateLimiter<RedisStoreActor> {
    RateLimiter::new(
        RedisStoreActor::from(store.clone()).start())
        .with_interval(TimeDuration::from_secs(30))
        .with_max_requests(200)
        .with_identifier(|req| {
            let identity = RequestIdentity::get_identity(req).unwrap_or("".to_string());
            let connection_info = req.connection_info();

            log::info!(target: "actix_web","{}  -----{:?}   {:?}","LIMITER" , identity,connection_info);
            let remote = connection_info.remote().unwrap_or("");
            Ok(identity + remote)
        })
}

// WIZ_OPT: use Redis as identity( and session) storage
use actix_web::cookie::SameSite;
/// Gets the session service for injection into an Actix app
pub fn get_session_service() -> RedisSession {
    let time_out = CONFIG.session_timeout as u16;
    let same_site = get_same_site();
    RedisSession::new(&CONFIG.redis_url, &CONFIG.session_key.as_ref())
        .cookie_name(&CONFIG.session_name)
        .ttl(time_out*60)
        .cookie_secure(CONFIG.session_secure)
        .cookie_same_site(same_site)
}

use actix_identity::{CookieIdentityPolicy, IdentityService, IdentityPolicy, RequestIdentity};
/// Gets the identidy service for injection into an Actix app
pub fn get_identity_service<T: IdentityPolicy>(policy: T) -> IdentityService<impl IdentityPolicy> {
    IdentityService::new(policy )
}

// WIZ_OPT: use cookies as identity storage
pub fn get_cookie_policy() -> CookieIdentityPolicy {
    let same_site = get_same_site();
    CookieIdentityPolicy::new(&CONFIG.session_key.as_ref())
        .name("id-".to_string() + &CONFIG.session_name)
        .max_age_time(chrono::Duration::minutes(CONFIG.session_timeout))
        .secure(CONFIG.session_secure)
        .same_site(same_site)
}


fn get_same_site() -> SameSite {
    let same_site : SameSite;
    match (&CONFIG.session_samesite.to_uppercase()).as_str() {
        "STRICT" => same_site = SameSite::Strict,
        "NONE" => same_site = SameSite::None,
        _ => same_site = SameSite::Lax,
    }
    same_site
}

fn mask_str(str: &String, mask : &String) -> String{
    let mut strb = str.clone().into_bytes();
    let maskb = mask.clone().into_bytes();
    let str_len = strb.len();
    let mask_len = maskb.len();
    let mut i = 0;
    let mut m = 0;
    while i < str_len{
        if m >= mask_len {
            m = 0;
        }
        strb[i] = (strb[i].wrapping_add(maskb[m])) % 128;
        i += 1;
        m+= 1;
    }
    return String::from_utf8(strb).unwrap();
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;
    static EMAIL: &str = "test@test.com";

    #[test]
    fn it_hashes_a_password() {
        let password = "password";
        let salt1 = thread_rng().sample_iter(&Alphanumeric).take(32).collect::<String>();
        let hashed = hash(password, &salt1);
        assert_ne!(password, hashed);
    }

    #[test]
    fn it_matches_2_hashed_passwords() {
        let password = "password";
        let salt = thread_rng().sample_iter(&Alphanumeric).take(32).collect::<String>();
        let hashed = hash(password, &salt);
        let hashed_again = hash(password, &salt);
        assert_eq!(hashed, hashed_again);
    }

    #[test]
    fn it_creates_a_jwt() {
        let private_claim = PrivateClaim::new(Uuid::new_v4(), EMAIL.into());
        let jwt = create_jwt(private_claim);
        assert!(jwt.is_ok());
    }

    #[test]
    fn it_decodes_a_jwt() {
        let private_claim = PrivateClaim::new(Uuid::new_v4(), EMAIL.into());
        let jwt = create_jwt(private_claim.clone()).unwrap();
        let decoded = decode_jwt(&jwt).unwrap();
        assert_eq!(private_claim, decoded);
    }


    #[test]
    fn it_masks_a_string() {
        let salt1 = "salt1salt1salt1".to_string();
        let mask = "mask52632".to_string();
        let masked = mask_str(&salt1, &mask);
        assert_ne!(masked, "".to_string());
    }
}
