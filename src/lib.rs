use bytes::Bytes;
use getset::Getters;
use std::sync::Arc;
use warp::filters::path::FullPath;
use warp::http::{HeaderMap, Method};
use warp::Filter;

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct Request {
    method: Method,
    path: FullPath,
    query: String,
    headers: HeaderMap,
    body: Bytes,
}

pub trait RequestLogger {
    fn log_request(&self, req: &Request);
}

pub async fn start_server<T: RequestLogger + Sync + Send + 'static>(port: u16, logger: T) {
    let logger_arc = Arc::new(logger);

    let handler = warp::any()
        .map(move || logger_arc.clone())
        .and(warp::method())
        .and(warp::path::full())
        .and(
            warp::query::raw()
                .or(warp::any().map(|| String::new()))
                .unify(),
        )
        .and(warp::header::headers_cloned())
        .and(warp::body::bytes())
        .and_then(
            |logger_arc: Arc<T>,
             method: Method,
             path: warp::path::FullPath,
             query: String,
             headers: HeaderMap,
             body: Bytes| async move {
                let req = Request {
                    method,
                    path,
                    query,
                    headers,
                    body,
                };

                logger_arc.log_request(&req);

                Ok::<_, warp::Rejection>(warp::reply())
            },
        );

    warp::serve(handler).run(([0, 0, 0, 0], port)).await;
}
