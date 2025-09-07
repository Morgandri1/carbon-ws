use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;

pub static ENCODING_KEY: Lazy<EncodingKey> = Lazy::new(|| 
    EncodingKey::from_base64_secret(&std::env::var("JWT_SECRET").expect("JWT_SECRET not defined"))
        .expect("invalid jwt secret format")
);

pub static DECODING_KEY: Lazy<DecodingKey> = Lazy::new(|| 
    DecodingKey::from_base64_secret(&std::env::var("JWT_SECRET").expect("JWT_SECRET not defined"))
        .expect("invalid jwt secret format")
);

pub static BUCKET_URL: Lazy<String> = Lazy::new(|| 
    std::env::var("BUCKET_URL").expect("BUCKET_URL not defined")
);