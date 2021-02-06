use super::handlers;
use warp::Filter;

pub fn api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    hello().or(hello_json())
}

pub fn hello() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("hello" / String)
        .and(warp::get())
        .and_then(handlers::hello)
}

pub fn hello_json() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("hello" / "json" / String)
        .and(warp::get())
        .and_then(handlers::hello_json)
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
}
