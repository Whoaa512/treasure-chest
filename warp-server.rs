use warp::{Filter, path, post, get};
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

// This function reads the file at the specified path and returns its contents.
async fn get_file(filepath: String) -> Result<impl warp::Reply, warp::Rejection> {
    // Open the file in read-only mode.
    let mut file = match fs::File::open(&filepath) {
        Ok(file) => file,
        Err(err) => return Ok(warp::reply::with_status(err.to_string(), warp::http::StatusCode::INTERNAL_SERVER_ERROR)),
    };

    // Read the file contents into a vector of bytes.
    let mut contents = Vec::new();
    if let Err(err) = file.read_to_end(&mut contents) {
        return Ok(warp::reply::with_status(err.to_string(), warp::http::StatusCode::INTERNAL_SERVER_ERROR));
    }

    // Return the file contents as a response.
    Ok(warp::reply::bytes(&contents))
}

#[tokio::main]
async fn main() {
    // Set up the route that accepts file uploads.
    let put_routes = post()
        .and(path("put").and(path::param::<String>()))
        .and(warp::body::bytes())
        .and_then(put_file);

    // Set up the route that returns the contents of a file.
    let get_routes = get()
        .and(path("get").and(path::param::<String>()))
        .and_then(get_file);

    // Start the server.
    warp::serve(put_routes.or(get_routes)).run(([127, 0, 0, 1], 3030)).await;
}
