use std::fs;
use std::io::Write;
use std::path::Path;

use warp::hyper::body::Body;
use warp::hyper::body::Bytes;
use warp::{get, path, put, Filter};

use warp::filters::path::Tail;
use warp::{Rejection, Reply};

// Use the warp::body::aggregate function to aggregate the request body
// into a Vec<u8> value, which implements the std::io::Read trait.
async fn put_file(filepath: String, file_buf: Vec<u8>) -> Result<impl Reply, Rejection> {
    let file_path = Path::new(&filepath);

    // create full path if it doesn't exist
    if let Err(err) = fs::create_dir_all(file_path.parent().unwrap()) {
        return Ok(warp::reply::with_status(
            err.to_string(),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    let mut file = fs::File::create(filepath).unwrap();

    if let Err(err) = file.write_all(&file_buf) {
        println!("Error: {}", err); // todo better error handling

        return Ok(warp::reply::with_status(
            err.to_string(),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    Ok(warp::reply::with_status(
        "File uploaded successfully.".to_string(),
        warp::http::StatusCode::CREATED,
    ))
}

// This function reads the file at the specified path and returns its contents.
async fn get_file(filepath_str: Tail) -> Result<impl warp::Reply, warp::Rejection> {
    // Convert the filepath string to a Path object.
    let filepath = Path::new(filepath_str.as_str());

    // check if file exists
    if !filepath.exists() {
        return Ok(warp::reply::with_status(
            str_to_response("File not found.".to_string()),
            warp::http::StatusCode::NOT_FOUND,
        ));
    }

    // Read the file contents into a buffer.
    let file_contents = match fs::read(filepath) {
        Ok(file_contents) => file_contents,
        Err(err) => {
            return Ok(warp::reply::with_status(
                str_to_response(err.to_string()),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    };

    // Create an HTTP response with the file contents as the body.
    let body = Body::from(file_contents);
    let response = warp::reply::Response::new(body);

    // Return the response.
    Ok(warp::reply::with_status(
        response,
        warp::http::StatusCode::OK,
    ))
}

#[tokio::main]
async fn main() {
    // Set up the routes for handling file uploads.
    let put_routes = put()
        .and(path("put").and(path::tail()))
        // 200MB limit
        .and(warp::body::content_length_limit(1024 * 1024 * 200))
        .and(warp::body::bytes())
        .and_then(|filepath_str: Tail, file: Bytes| {
            let filepath = filepath_str.as_str().to_owned();
            put_file(filepath, file.to_vec())
        });

    // Set up the route that returns the contents of a file.
    let get_routes = get().and(path("get").and(path::tail())).and_then(get_file);

    // Start the server.
    warp::serve(put_routes.or(get_routes))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn str_to_response(str: String) -> warp::reply::Response {
    let body = Body::from(str);
    warp::reply::Response::new(body)
}
