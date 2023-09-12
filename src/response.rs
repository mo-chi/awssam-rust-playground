use http::StatusCode;
use lambda_http::{Body, Error, Response};

pub fn builder(status_code: StatusCode, body: Body) -> Result<Response<Body>, Error> {
    let resp = Response::builder()
        .status(status_code)
        .header("content-type", "application/json")
        .body(body)
        .map_err(Box::new)?;

    Ok(resp)
}
