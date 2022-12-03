use std::fs;
use std::io::Write;
use std::path::Path;

use warp::hyper::body::Bytes;
use warp::{Filter, path, put};

use warp::{Rejection, Reply};
use warp::filters::path::Tail;


// Use the warp::body::aggregate function to aggregate the request body
// into a Vec<u8> value, which implements the std::io::Read trait.
async fn put_file(filepath: String, file_buf: Vec<u8>) -> Result<impl Reply, Rejection> {
    let file_path = Path::new(&filepath);

    // create full path if it doesn't exist
    if let Err(err) = fs::create_dir_all(file_path.parent().unwrap()) {
        return Ok(warp::reply::with_status(err.to_string(), warp::http::StatusCode::INTERNAL_SERVER_ERROR));
    }

    let mut file = fs::File::create(filepath).unwrap();

    if let Err(err) = file.write_all(&file_buf) {
        println!("Error: {}", err);  // todo better error handling

        return Ok(warp::reply::with_status(err.to_string(), warp::http::StatusCode::INTERNAL_SERVER_ERROR));
    }

    Ok(warp::reply::with_status("File uploaded successfully.".to_string(), warp::http::StatusCode::CREATED))
}


// // This function reads the file at the specified path and returns its contents.
// async fn get_file(filepath: String) -> Result<impl warp::Reply, warp::Rejection> {
//     // Open the file in read-only mode.
//     let mut file = match fs::File::open(&filepath) {
//         Ok(file) => file,
//         Err(err) => return Ok(warp::reply::with_status(err.to_string(), warp::http::StatusCode::INTERNAL_SERVER_ERROR)),
//     };

//     // Read the file contents into a vector of bytes.
//     let mut contents = Vec::new();
//     if let Err(err) = file.read_to_end(&mut contents) {
//         return Ok(warp::reply::with_status(err.to_string(), warp::http::StatusCode::INTERNAL_SERVER_ERROR));
//     }

//     // Return the file contents as a response.
//     // Ok(warp::reply::with_status(contents, warp::http::StatusCode::OK))

//     let response = Response::builder()
//         .status(200)
//         .header("Content-Type", "image/png")
//         .body(content)
//         .unwrap();

//     Ok(warp::reply::with_status(response, warp::http::StatusCode::OK))
// }

#[tokio::main]
async fn main() {

    // // Set up the route that returns the contents of a file.
    // let get_routes = get()
    //     .and(path("get").and(path::param::<String>()))
    //     .and_then(get_file);

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

    // // Set up the route that returns the contents of a file.
    // let get_routes = get()
    //     .and(path("get").and(path::tail()))
    //     .and_then(|filepath_str: Tail| {
    //         let filepath = filepath_str.as_str().to_owned();
    //         let mut file = fs::File::open(filepath).unwrap();
    //         let mut data = Vec::new();
    //         file.read_to_end(&mut data).unwrap();
    //         Ok(warp::http::Response::new(data))
    //     });

    // Start the server.
    warp::serve(put_routes).run(([127, 0, 0, 1], 3030)).await;
}
