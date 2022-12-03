use warp::{Filter, path, post};
use std::fs;

// This function processes the file upload and writes the file to the specified path.
async fn put_file(filepath: String, file: impl warp::Buf) -> Result<impl warp::Reply, warp::Rejection> {
    // Open the file in write-only mode.
    let mut file = match fs::File::create(&filepath) {
        Ok(file) => file,
        Err(err) => return Ok(warp::reply::with_status(err.to_string(), warp::http::StatusCode::INTERNAL_SERVER_ERROR)),
    };

    // Write the file contents to the file.
    if let Err(err) = file.write_all(file.bytes()) {
        return Ok(warp::reply::with_status(err.to_string(), warp::http::StatusCode::INTERNAL_SERVER_ERROR));
    }

    Ok(warp::reply())
}

#[tokio::main]
async fn main() {
    // Set up the route that accepts file uploads.
    let routes = post()
        .and(path("put").and(path::param::<String>()))
        .and(warp::body::bytes())
        .and_then(put_file);

    // Start the server.
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
