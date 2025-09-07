use diesel::{OptionalExtension, QueryDsl, RunQueryDsl};
use warp::Filter;
use crate::db::establish_connection;
use crate::middleware::jwt::verify_jwt;
use crate::result::CarbonError;
use crate::schema::users;
use crate::models::User;
use crate::constants::DECODING_KEY;

pub fn with_user() -> impl Filter<Extract = (User,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::header::header::<String>("Authorization"))
        .and_then(|auth: String| async move {
            match verify_jwt(&auth, &DECODING_KEY) {
                Ok(data) => {
                    let conn = &mut establish_connection();
                    let user = users::table.find(data.claims.inner().id)
                        .first::<User>(conn)
                        .optional()
                        .map_err(|_| CarbonError::DatabaseError { message: "failed to load users".to_string() }.reject())?;
                    if let Some(u) = user {
                        Ok(u)
                    } else {
                        Err(CarbonError::UserError { message: "not found".to_string(), code: 404 }.reject())
                    }
                },
                Err(_) => Err(CarbonError::UserError { message: "unauthorized".to_string(), code: 401 }.reject())
            }
        })
}