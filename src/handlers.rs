use serde_derive::{Deserialize, Serialize};
use std::convert::Infallible;

pub async fn hello(name: String) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("Hello, {}!", name))
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HelloJson {
    data: String,
}

pub async fn hello_json(name: String) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&HelloJson {
        data: format!("Hello, {}!", name),
    }))
}

pub async fn receive_json(body: HelloJson) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&body))
}
