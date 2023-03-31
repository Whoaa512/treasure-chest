use bytes::Bytes;
use std::path::PathBuf;
use tokio::fs;
use warp::{Filter, Rejection, Reply};

pub mod handlers {
    use super::*;

    pub async fn get_file(filepath: String) -> Result<impl Reply, Rejection> {
        let path = PathBuf::from("storage").join(&filepath);

        match fs::read(&path).await {
            Ok(contents) => Ok(warp::reply::with_status(
                contents,
                warp::http::StatusCode::OK,
            )),
            Err(_) => Ok(warp::reply::with_status(
                warp::http::StatusCode::NOT_FOUND
                    .as_str()
                    .as_bytes()
                    .to_vec(),
                warp::http::StatusCode::NOT_FOUND,
            )),
        }
    }

    pub async fn put_file(filepath: String, file_buf: Bytes) -> Result<impl Reply, Rejection> {
        let path = PathBuf::from("storage").join(&filepath);
        fs::write(path, file_buf)
            .await
            .map_err(|_| warp::reject())?;
        Ok(warp::reply::with_status(
            warp::http::StatusCode::CREATED.as_str().as_bytes().to_vec(),
            warp::http::StatusCode::CREATED,
        ))
    }
}

pub async fn start_server() {
    let put_file = warp::path("files")
        .and(warp::path::param())
        .and(warp::post())
        .and(warp::body::bytes())
        .and_then(handlers::put_file);

    let get_file = warp::path("files")
        .and(warp::path::param())
        .and(warp::get())
        .and_then(handlers::get_file);

    let routes = put_file.or(get_file);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

#[tokio::main]
async fn main() {
    start_server().await;
}
