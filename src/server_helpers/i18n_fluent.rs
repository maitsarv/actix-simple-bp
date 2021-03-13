use std::{error::Error, fmt, fs};
use actix_web::{dev::Payload, FromRequest, HttpRequest, ResponseError};
use fluent_templates::{FluentLoader, static_loader};
use futures::future::{ok, Ready};
use actix_web::http::HeaderValue;

static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US"
    };
}

const ACCEPT_LANG: &'static str = "Accept-Language";

pub fn get_supported_languages()
{

}

/// A request guard to get the right locale.
pub struct I18n {
    /// The language of the current request.
    pub lang: String,
}



#[derive(Debug)]
pub struct MissingTranslationsError(String);

impl fmt::Display for MissingTranslationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not find translations for {}", self.0)
    }
}

impl Error for MissingTranslationsError {
    fn description(&self) -> &str {
        "Could not find translations"
    }
}

impl ResponseError for MissingTranslationsError {
    // this defaults to an empty InternalServerError response
}

#[derive(Debug)]
pub struct MissingStateError;

impl fmt::Display for MissingStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not retrieve state")
    }
}

impl Error for MissingStateError {
    fn description(&self) -> &str {
        "Could not retrieve state"
    }
}

impl ResponseError for MissingStateError {
    // this defaults to an empty InternalServerError response
}

impl FromRequest for I18n {
    type Config = ();
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let lang = req
            .headers()
            .get(ACCEPT_LANG)
            .unwrap_or(&HeaderValue::from_static("en"))
            .to_str().unwrap().to_owned();
        ok(I18n {
            lang,
        })
    }
}
