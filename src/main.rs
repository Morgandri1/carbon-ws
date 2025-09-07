use std::{convert::Infallible, net::{IpAddr, Ipv4Addr, SocketAddr}};
use dotenvy::dotenv;
use warp::{http::StatusCode, reply::{Json, WithStatus}, Filter};

use crate::{result::CarbonError, routes::*};

mod ws;
mod routes;
mod schema;
mod models;
mod result;
mod constants;
mod middleware;
mod db;
mod utils;
mod cache;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(["Authorization",])
        .allow_methods(["GET","POST","PUT","DELETE"])
        .build();
    let routes = ws::chat_handler()
        .or(auth::routes())
        .recover(|e: warp::reject::Rejection| async move {
            match e.find::<CarbonError>() {
                Some(e) => Ok::<WithStatus<Json>,Infallible>(warp::reply::with_status(warp::reply::json(&e), match e {
                    CarbonError::UserError { code, .. } => StatusCode::from_u16(*code)
                        .unwrap_or(StatusCode::BAD_REQUEST),
                    CarbonError::DatabaseError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
                    CarbonError::ExternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
                    CarbonError::WebSocketError => StatusCode::BAD_GATEWAY,
                    CarbonError::SerializerError => StatusCode::INTERNAL_SERVER_ERROR,
                    CarbonError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR
                })),
                None => Ok(warp::reply::with_status(warp::reply::json(&""), StatusCode::INTERNAL_SERVER_ERROR))
            }
        })
        .with(cors);
    println!("starting server...");
    warp::serve(routes)
        .run(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 80)).await;
}