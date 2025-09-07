use std::i64;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{Filter,Rejection,Reply};
use diesel::{RunQueryDsl};

use crate::{
    constants::{DECODING_KEY, ENCODING_KEY}, db::establish_connection, middleware::{sign_jwt, verify_jwt, JwtData, JwtType}, models::User, result::CarbonError, routes::upload::{download_image_as_png, upload_png_buffer}, utils::user::generate_avatar
};

#[derive(Serialize,Deserialize)]
pub struct RegisterPayload {
    pub pubkey: String,
    pub username: String,
    pub display_name: Option<String>,
    pub img_src: Option<String>,
}

pub async fn register(body: RegisterPayload) -> Result<impl Reply, Rejection> {
    let uid = Uuid::new_v4();
    let img = if let Some(src) = body.img_src {
        download_image_as_png(&src).await
    } else {
        generate_avatar(&body.username)
    }?;
    let url = upload_png_buffer(uid, img).await?;
    let conn = &mut establish_connection();
    let user = User {
        id: uid,
        pubkey: body.pubkey,
        display_name: body.display_name,
        username: body.username,
        img_src: Some(url),
        bio: None,
        contacts: Vec::new()
    };
    diesel::insert_into(crate::schema::users::table)
        .values(&user)
        .execute(conn)
        .map_err(|_| CarbonError::DatabaseError { message: "Failed to insert user".to_string() })?;
    // sign 1 day access token
    let access_token = sign_jwt(JwtType::Access(JwtData::new(uid, None)), &ENCODING_KEY)?;
    // sign (essentially) non-expiring secret. this is the account secret.
    let secret = sign_jwt(JwtType::Secret(JwtData::new(uid, Some(i64::MAX))), &ENCODING_KEY)?;
    Ok(warp::reply::json(&serde_json::json!({
        "access_token": access_token,
        "secret": secret
    })))
}

pub async fn refresh(secret: String) -> Result<impl Reply, Rejection> {
    let data = if let JwtType::Secret(t) = verify_jwt(&secret, &DECODING_KEY)?.claims {
        t
    } else {
        return Err(CarbonError::UserError { message: "invalid key passed".to_string(), code: 401 }.reject());
    };
    let access_token = sign_jwt(JwtType::Access(JwtData::new(data.id, None)), &ENCODING_KEY)?;
    Ok(warp::reply::json(&serde_json::json!({
        "access_token": access_token,
    })))
}

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let reg = warp::path!("register")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(register);
    let refresh = warp::path!("refresh")
        .and(warp::get())
        .and(warp::header("Authorization"))
        .and_then(refresh);
    reg.or(refresh)
}