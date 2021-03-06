use warp::Filter;

pub mod filters;
pub mod handlers;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "example");
    env_logger::init();

    let routes = filters::api().with(warp::log("example"));
    warp::serve(routes).run(([0, 0, 0, 0], 3000)).await;
}
