use warp::Rejection;

pub type Result<T> = std::result::Result<T, Rejection>;

pub fn cors() -> warp::cors::Builder {
    warp::cors()
        .allow_any_origin()
        .expose_headers(vec![
            "x-duration",
            "x-provided-by",
            "x-initiated-by",
            "x-processed-by",
        ])
        .allow_headers(vec![
            "User-Agent",
            "Sec-Fetch-Mode",
            "Referer",
            "Origin",
            "content-type",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
            "Access-Control-Allow-Headers",
            "Access-Control-Allow-Methods",
            "Access-Control-Allow-Origin",
            "Access-Control-Expose-Headers",
            "Access-Control-Request-Headers",
            "Access-Control-Request-Methods",
            "Accept-Encoding",
            "Accept-Language",
            "Accept-Post",
            "Access-Control-Allow-Credentials",
            "Access-Control-Allow-Origin",
            "keep-alive",
            "x-duration",
            "x-provided-by",
            "x-initiated-by",
            "x-processed-by",
        ])
        .allow_methods(vec!["POST", "GET", "OPTIONS", "PUT", "DELETE", "HEAD"])
}
