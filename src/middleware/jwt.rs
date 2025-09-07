use jsonwebtoken::{decode, DecodingKey, EncodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::result::CarbonResult;

#[derive(Serialize,Deserialize)]
pub struct JwtData {
    #[serde(with = "uuid::serde::simple")]
    pub id: Uuid,
    exp: Option<i64>
}

impl JwtData {
    pub fn exp(&mut self, exp: i64) {
        self.exp = Some(exp)
    }
    
    pub fn has_exp(&self) -> bool {
        self.exp.is_some()
    }
}

pub fn verify_jwt(token: &str, key: &DecodingKey) -> CarbonResult<TokenData<JwtData>> {
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    decode::<JwtData>(token, key, &validation)
        .map_err(|_| crate::result::CarbonError::SerializerError)
}

pub fn sign_jwt(mut token: JwtData, key: &EncodingKey) -> CarbonResult<String> {
    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
    if !token.has_exp() {
        token.exp(1000 * 60 * 60 * 24); // default to 1d
    }
    let token = jsonwebtoken::encode(
        &header, 
        &token, 
        key
    );
    token.map_err(|_| crate::result::CarbonError::SerializerError)
}