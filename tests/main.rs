#[cfg(test)]
mod tests {
    use simple_blob_store::handlers::{get_file, put_file};
    use warp::Reply;

    #[tokio::test]
    async fn test_put_file() {
        let filepath = "test.txt";
        let file_buf = vec![1, 2, 3];

        let result = put_file(filepath.to_string(), file_buf).await;

        assert!(result.is_ok());
        let val = result.unwrap().into_response();
        assert_eq!(val.status(), 201);
    }

    #[tokio::test]
    async fn test_get_file() {
        let filepath = "test.txt";
        let file_buf = vec![1, 2, 3];

        let put_res = put_file(filepath.to_string(), file_buf).await;
        assert!(put_res.is_ok());
        let val = put_res.unwrap().into_response();
        assert_eq!(val.status(), 201);

        let result = get_file(filepath.to_string()).await;

        assert!(result.is_ok());
        let val = result.unwrap().into_response();
        assert_eq!(val.status(), 200);
    }

    #[tokio::test]
    async fn test_get_file_not_found() {
        let filepath = "does_not_exist.txt".to_string();

        let result = get_file(filepath).await;
        assert!(result.is_ok());
        let val = result.unwrap().into_response();
        assert_eq!(val.status(), 404);
    }
}
