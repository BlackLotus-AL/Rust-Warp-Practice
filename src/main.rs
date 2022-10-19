use std::collections::HashMap;
use warp::Filter;
use std::env;

const WEB_DIR: &str = "web/";

async fn get_items(
    param: String,
    param_map: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("get {}: {:?}", param, param_map))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUST_APP_LOG", "DEBUG");
    pretty_env_logger::init_custom_env("RUST_APP_LOG");
    let log = warp::log("basic");

    let greet = warp::path!("greet"/String/i32)
        .map(|name, age| {format!("name: {} age: {}", name, age)});
    let add = warp::path!("add"/i32/i32)
        .map(|a, b| {format!("a + b = {}", a + b)});
    let items = warp::get()
        .and(warp::path("items"))
        .and(warp::path::param::<String>())
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::path::end())
        .and_then(get_items);
    let apis = warp::get().and(greet.or(add).or(items)).with(log);

    let dir_static = warp::fs::dir(WEB_DIR);
    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/index.html", WEB_DIR)));
    let static_route = dir_static.or(index);

    let routes = static_route.or(apis);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3000))
        .await;
    Ok(())
}
