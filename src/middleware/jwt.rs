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

#[derive(Serialize,Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "UPPERCASE")]
pub enum JwtType {
    Secret(JwtData),
    Access(JwtData)
}

impl JwtType {
    pub fn inner(&self) -> &JwtData {
        match self {
            JwtType::Secret(data) => data,
            JwtType::Access(data) => data,
        }
    }
    
    pub fn exp(&mut self, exp: i64) {
        match self {
            JwtType::Secret(data) => data.exp(exp),
            JwtType::Access(data) => data.exp(exp),
        }
    }
}

impl JwtData {
    pub fn new(id: Uuid, exp: Option<i64>) -> Self {
        JwtData { id, exp }
    }
    
    pub fn exp(&mut self, exp: i64) {
        self.exp = Some(exp)
    }
    
    pub fn has_exp(&self) -> bool {
        self.exp.is_some()
    }
}

pub fn verify_jwt(token: &str, key: &DecodingKey) -> CarbonResult<TokenData<JwtType>> {
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    decode::<JwtType>(token, key, &validation)
        .map_err(|_| crate::result::CarbonError::SerializerError)
}

pub fn sign_jwt(mut token: JwtType, key: &EncodingKey) -> CarbonResult<String> {
    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
    if !token.inner().has_exp() {
        token.exp(1000 * 60 * 60 * 24); // default to 1d
    }
    let token = jsonwebtoken::encode(
        &header, 
        &token, 
        key
    );
    token.map_err(|_| crate::result::CarbonError::SerializerError)
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    #[test]
    pub fn should_deserialize_jwt() {
        let secret = "{\"type\":\"SECRET\",\"id\":\"840a8924297f4097ab51db10dfe57f1c\",\"exp\":null}";
        let token = serde_json::from_str::<JwtType>(secret);
        assert!(token.is_ok())
    }
    
    #[test]
    pub fn should_serialize_jwt() {
        let token = JwtType::Secret(JwtData { id: Uuid::from_str("840a8924-297f-4097-ab51-db10dfe57f1c").unwrap(), exp: None });
        let v = serde_json::to_string(&token).expect("failed to serialize");
        assert_eq!(v, "{\"type\":\"SECRET\",\"id\":\"840a8924297f4097ab51db10dfe57f1c\",\"exp\":null}")
    }
    
    #[test]
    pub fn roundtrip_jwt() {
        let uid = Uuid::new_v4();
        let token = JwtType::Secret(JwtData { id: uid, exp: None });
        let v = serde_json::to_string(&token).expect("failed to serialize");
        let token = serde_json::from_str::<JwtType>(&v).expect("failed to deserialize");
        let token = if let JwtType::Secret(t) = token {
            t
        } else {
            panic!("incorrect token type!")
        };
        assert_eq!(token.id, uid);
    }
}