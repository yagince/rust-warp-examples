use super::handlers;
use warp::Filter;

pub fn api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    hello().or(receive_json())
}

pub fn hello() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("hello").and(hello_string().or(hello_json()))
}

pub fn hello_string() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!(String)
        .and(warp::get())
        .and_then(handlers::hello)
}

pub fn hello_json() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("json" / String)
        .and(warp::get())
        .and_then(handlers::hello_json)
}

pub fn receive_json() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("receive" / "json")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handlers::receive_json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_json_diff::assert_json_eq;
    use serde_json::json;

    #[tokio::test]
    async fn test_hello() {
        let filter = api();

        let res = warp::test::request()
            .path("/hello/warp")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), 200);
        assert_eq!(res.body(), "Hello, warp!".as_bytes());
    }

    #[tokio::test]
    async fn test_hello_json() -> anyhow::Result<()> {
        let filter = api();

        let res = warp::test::request()
            .path("/hello/json/warp")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), 200);

        assert_json_eq!(
            serde_json::from_slice::<serde_json::Value>(res.body())?,
            json!({ "data": "Hello, warp!"})
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_receive_json() -> anyhow::Result<()> {
        let body = json!({ "data": "Hello, warp!"});
        let res = warp::test::request()
            .method("POST")
            .path("/receive/json")
            .body(body.to_string())
            .reply(&api())
            .await;

        assert_eq!(res.status(), 200);

        assert_json_eq!(
            serde_json::from_slice::<serde_json::Value>(res.body())?,
            body,
        );

        Ok(())
    }
}
