use std::convert::Infallible;
use std::env;
use std::sync::Arc;
use serde_json::{json, Value};
use warp::{Filter, reply::Json};
use crate::header_handler::{auth, ContextUser};

mod header_handler;

#[tokio::main]
async fn main() {
    env::set_var("RUST_APP_LOG", "DEBUG");
    pretty_env_logger::init_custom_env("RUST_APP_LOG");
    let log = warp::log("apis");
    let db_pool = Arc::new(DbPool {});
    let hi = warp::path("hi").and(warp::get()).map(|| "hi");

    let apis = hi.or(rest_api(db_pool.clone())).with(log);
    warp::serve(apis)
        .run(([127, 0, 0, 1], 3000))
        .await;
}

pub fn rest_api(
    pool: Arc<DbPool>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let base_url = warp::path("rest");
    let get = base_url
        .and(warp::get())
        .and(warp::path::param())
        .and(auth())
        .and(with_pool(pool.clone()))
        .and_then(rest_get);
    let list = base_url
        .and(warp::get())
        .and(warp::path::end())
        .and(auth())
        .and(with_pool(pool.clone()))
        .and_then(rest_list);
    let create = base_url
        .and(warp::post())
        .and(warp::body::json())
        .and(auth())
        .and(with_pool(pool.clone()))
        .and_then(rest_create);
    get.or(list).or(create)
}

async fn rest_create(data: Value, _context_user: ContextUser, _db: Arc<DbPool>) -> Result<Json, warp::Rejection> {
    Ok(warp::reply::json(&data))
}

async fn rest_get(id: i32, context_user: ContextUser, _db: Arc<DbPool>) -> Result<Json, warp::Rejection> {
    let some_thing = json!(
        {
            "id": id,
            "user_id": context_user.id,
            "name": format!("name: {}", id)
        }
    );
    let some_thing_warp = warp::reply::json(&some_thing);
    Ok(some_thing_warp)
}

async fn rest_list(_context_user: ContextUser, _db: Arc<DbPool>) -> Result<Json, warp::Rejection> {
    let some_thing = json!([
        {"id": 1, "name": "ok1"},
        {"id": 2, "name": "ok2"},
        {"id": 3, "name": "ok3"},
        {"id": 4, "name": "ok4"},
    ]
    );
    let some_thing_warp = warp::reply::json(&some_thing);
    Ok(some_thing_warp)
}

pub struct DbPool {}

pub fn with_pool(pool: Arc<DbPool>,
) -> impl Filter<Extract = (Arc<DbPool>,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}
