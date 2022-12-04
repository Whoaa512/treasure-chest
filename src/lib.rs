pub mod handlers {
    use std::fs;
    use std::io::Write;
    use std::path::Path;

    use warp::hyper::body::Body;

    use warp::reply::WithStatus;
    use warp::{Rejection, Reply};

    // TODO: make this configurable
    const UPLOAD_DIR: &str = "uploads"; // This is the directory where we will store the uploaded files.

    // Use the warp::body::aggregate function to aggregate the request body
    // into a Vec<u8> value, which implements the std::io::Read trait.
    pub async fn put_file(filepath: String, file_buf: Vec<u8>) -> Result<impl Reply, Rejection> {
        let file_path = Path::new(UPLOAD_DIR).join(filepath);

        let parent = file_path.parent().unwrap();

        // create full path if it doesn't exist
        if let Err(err) = fs::create_dir_all(parent) {
            return Ok(warp::reply::with_status(
                err.to_string(),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }

        let mut file = fs::File::create(file_path).unwrap();

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
    pub async fn get_file(filepath_str: String) -> Result<impl warp::Reply, warp::Rejection> {
        // Convert the filepath string to a Path object.

        let filepath = Path::new(UPLOAD_DIR).join(filepath_str.as_str());

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

    pub fn str_to_response(str: String) -> warp::reply::Response {
        let body = Body::from(str);
        warp::reply::Response::new(body)
    }
}
