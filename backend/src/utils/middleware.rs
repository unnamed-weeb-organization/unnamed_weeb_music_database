use hyper::{header, Body, Request, Response};
use routerify::ext::RequestExt;
use std::io;
use tracing::info;

use crate::constants;

pub async fn setup_cors(mut req: Response<Body>) -> Result<Response<Body>, io::Error> {
    let headers = req.headers_mut();

    headers.insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        constants::ALLOWED_CONTROL_HOSTS.clone(),
    );

    Ok(req)
}

pub async fn logger(req: Request<Body>) -> Result<Request<Body>, io::Error> {
    info!(
        "{} {} {}",
        req.remote_addr(),
        req.method(),
        req.uri().path()
    );
    Ok(req)
}
