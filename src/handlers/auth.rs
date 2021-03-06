use crate::auth::{create_jwt, PrivateClaim};
use crate::database::connection::PoolType;
use crate::server_helpers::errors::ApiError;
use crate::handlers::user::UserResponse;
use actix_session::Session;
use crate::server_helpers::response::{respond_json, respond_ok};
use crate::models::user::find_by_auth;
use crate::validate::validate;
use actix_identity::Identity;
use actix_web::web::{block, Data, HttpResponse, Json};
use serde::Serialize;
use validator::Validate;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "email must be a valid email"))]
    pub email: String,

    #[validate(length(
        min = 6,
        message = "password is required and must be at least 6 characters"
    ))]
    pub password: String,
}

/// Login a user
/// Create and remember their JWT
pub async fn login(
    id: Identity,
    pool: Data<PoolType>,
    params: Json<LoginRequest>,
    session: Session
) -> Result<Json<UserResponse>, ApiError> {
    validate(&params)?;

    // Validate that the email + password matches
    let user = block(move || find_by_auth(&pool, &params.email, &params.password)).await?;

    //JWT cookie session
    // Create a JWT
    let private_claim = PrivateClaim::new(user.id, user.email.clone());
    let jwt = create_jwt(private_claim)?;
    // Remember the token
    id.remember(jwt);

    // WIZ_OPT: Random key cookie session
    //TODO: make session optional
    let sess = session.set("user_id", &user.id);
    match sess {
        Ok(_0) => (),
        Err(e) => return Err(ApiError::InternalServerError(String::from("Could not set session var")))
    }
    session.renew();
    respond_json(user.into())
}

/// Logout a user
/// Forget their user_id
pub async fn logout(id: Identity, session: Session) -> Result<HttpResponse, ApiError> {
    // WIZ_OPT: Random key cookie session
    session.clear();
    // WIZ_OPT: JWT identity
    //id.forget();
    respond_ok()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::tests::helpers::tests::get_data_pool;
    use actix_identity::Identity;
    use actix_web::{test, FromRequest};

    async fn get_identity() -> Identity {
        let (request, mut payload) =
            test::TestRequest::with_header("content-type", "application/json").to_http_parts();
        let identity = Option::<Identity>::from_request(&request, &mut payload)
            .await
            .unwrap()
            .unwrap();
        identity
    }

    async fn login_user() -> Result<Json<UserResponse>, ApiError> {
        let params = LoginRequest {
            email: "satoshi@nakamotoinstitute.org".into(),
            password: "123456".into(),
        };
        let identity = get_identity().await;
        login(identity, get_data_pool(), Json(params)).await
    }

    async fn logout_user() -> Result<HttpResponse, ApiError> {
        let identity = get_identity().await;
        logout(identity).await
    }

    #[actix_rt::test]
    async fn it_logs_a_user_in() {
        let response = login_user().await;
        assert!(response.is_ok());
    }

    #[actix_rt::test]
    async fn it_logs_a_user_out() {
        login_user().await.unwrap();
        let response = logout_user().await;
        assert!(response.is_ok());
    }
}
