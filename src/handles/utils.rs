use serde::de::DeserializeOwned;
use serde::Serialize;
use warp::Filter;

use crate::db::Pool;
use crate::errors::AppError;

pub fn with_pool(
    pool: Pool,
) -> impl Filter<Extract = (Pool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

pub fn respond<T: Serialize>(
    result: Result<T, AppError>,
    status: warp::http::StatusCode,
) -> Result<impl warp::Reply, warp::Rejection> {
    match result {
        Ok(response) => Ok(warp::reply::with_status(
            warp::reply::json(&response),
            status,
        )),
        Err(err) => {
            // log::error!("Error while trying to respond: {}", err.to_string());
            Err(warp::reject::custom(err))
        }
    }
}

pub fn with_json_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
