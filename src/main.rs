use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let routes = warp::fs::dir("www/html").or(warp::fs::dir("www"));

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
