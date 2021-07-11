pub mod token;

use warp;
use warp::Filter;

use crate::db::Pool;

pub fn api_filters(
    pool: Pool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "v1" / ..)
        .and(
            token::generate_auth_token(pool.clone())
                .or(token::ping())
        )
}
