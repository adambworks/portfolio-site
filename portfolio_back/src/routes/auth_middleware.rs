use actix_web::Error;
use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::headers::www_authenticate::bearer::Bearer;
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::routes::auth::{Claims, JWT_SECRET};

pub async fn validator(
    req: ServiceRequest,
    auth: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    log::info!("Validator is being called");
    let token = auth.token();
    let decoding_key = DecodingKey::from_secret(&JWT_SECRET);
    let validation = Validation::default();

    match decode::<Claims>(token, &decoding_key, &validation) {
        Ok(_claims) => Ok(req),
        Err(e) => {
            log::info!("JWT Decode Error: {}", e);
            let challenge = Bearer::default();
            let err = AuthenticationError::new(challenge);
            Err((err.into(), req))
        }
    }
}