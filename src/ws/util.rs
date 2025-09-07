use diesel::{ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::result::{CarbonError, CarbonResult};

pub fn parse_mentions(content: Option<&String>, conn: &mut PgConnection) -> CarbonResult<Vec<Uuid>> {
    let content = if let Some(c) = content {
        c
    } else {
        return Ok(vec![]);
    };
    let parts = content.split(" ");
    let mut mentions = vec![];
    for part in parts {
        if part.starts_with('@') {
            use crate::schema::users::dsl::*;
            let uid = users.filter(username.eq(&part[1..]))
                .select(id)
                .first::<Uuid>(conn)
                .optional()
                .map_err(|_| CarbonError::DatabaseError { message: "failed to query users".to_string() })?;
            if let Some(uid) = uid {
                mentions.push(uid);
            }
        }
    }
    Ok(mentions)
}

pub fn get_usernames(ids: &Vec<Uuid>, conn: &mut PgConnection) -> CarbonResult<Vec<String>> {
    use crate::schema::users::dsl::*;
    users.filter(id.eq_any(ids))
        .select(username)
        .load::<String>(conn)
        .map_err(|_| CarbonError::DatabaseError { message: "failed to query users".to_string() })
}
