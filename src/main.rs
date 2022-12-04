use warp::hyper::body::Bytes;
use warp::{get, path, put, Filter};

use warp::filters::path::Tail;

use simple_blob_store::handlers::{get_file, put_file};

#[tokio::main]
async fn main() {
    // Set up the routes for handling file uploads.
    let put_routes = put()
        .and(path::tail())
        // 200MB limit
        .and(warp::body::content_length_limit(1024 * 1024 * 200))
        .and(warp::body::bytes())
        .and_then(|filepath_tail: Tail, file: Bytes| {
            let filepath = filepath_tail.as_str().to_owned();
            put_file(filepath, file.to_vec())
        });

    // Set up the route that returns the contents of a file.
    let get_routes = get().and(path::tail()).and_then(|filepath_tail: Tail| {
        let filepath = filepath_tail.as_str().to_owned();
        get_file(filepath)
    });

    // Start the server.
    warp::serve(put_routes.or(get_routes))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
