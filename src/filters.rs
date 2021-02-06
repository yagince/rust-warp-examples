use super::handlers;
use serde_derive::{Deserialize, Serialize};
use warp::Filter;

pub fn api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    hello().or(receive_json()).or(with_auth_json())
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

pub fn with_auth_json() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path!("with_auth" / "json")
        .and(with_authenticate())
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handlers::with_auth)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("no auth header")]
    NoAuthHeaderError,
    #[error("invalid auth header")]
    InvalidAuthHeaderError,
}
impl warp::reject::Reject for Error {}

pub fn with_authenticate() -> impl Filter<Extract = (User,), Error = warp::Rejection> + Clone {
    warp::header::headers_cloned().and_then(authorize)
}

async fn authorize(
    headers: warp::http::HeaderMap<warp::http::HeaderValue>,
) -> Result<User, warp::Rejection> {
    match token_from_header(&headers) {
        Ok(token) => {
            if token != "hogehoge" {
                return Err(warp::reject::custom(Error::InvalidAuthHeaderError));
            }
            Ok(User {
                id: 0,
                name: "hoge".to_owned(),
            })
        }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

fn token_from_header(
    headers: &warp::http::HeaderMap<warp::http::HeaderValue>,
) -> Result<String, Error> {
    let header = headers
        .get(warp::http::header::AUTHORIZATION)
        .ok_or_else(|| Error::NoAuthHeaderError)?;

    let value = std::str::from_utf8(header.as_bytes()).map_err(|_| Error::NoAuthHeaderError)?;

    if !value.starts_with("Bearer ") {
        return Err(Error::InvalidAuthHeaderError);
    }

    Ok(value.trim_start_matches("Bearer ").to_owned())
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

    #[tokio::test]
    async fn test_receive_json_invalid() -> anyhow::Result<()> {
        let body = json!({ "hoge": "foo"});

        let res = warp::test::request()
            .method("POST")
            .path("/receive/json")
            .body(body.to_string())
            .reply(&api())
            .await;

        assert_eq!(res.status(), 400);
        assert_eq!(
            res.body(),
            "Request body deserialize error: missing field `data` at line 1 column 14".as_bytes()
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_with_auth() -> anyhow::Result<()> {
        let body = json!({ "data": "foo"});

        let res = warp::test::request()
            .method("POST")
            .header("Authorization", "Bearer hogehoge")
            .path("/with_auth/json")
            .body(body.to_string())
            .reply(&api())
            .await;

        assert_eq!(res.status(), 200);
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(res.body())?,
            json!({
                "user": { "id": 0, "name": "hoge" },
                "body": body,
            }),
        );

        Ok(())
    }
}
