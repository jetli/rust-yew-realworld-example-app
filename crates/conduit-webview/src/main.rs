#![windows_subsystem = "windows"]

use std::borrow::Cow;
use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use rust_embed::RustEmbed;
use web_view::Content;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/static"]
struct Assets;

async fn assets(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = if req.uri().path() == "/" {
        // If there is no path, return default file
        "index.html"
    } else {
        // Trim leading '/'
        &req.uri().path()[1..]
    };

    // Query the file from embeded assets with specified path
    match Assets::get(path) {
        Some(content) => {
            let body: Body = match content {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into(),
            };
            Ok(Response::new(body))
        }
        None => Ok(Response::builder()
            .status(404)
            .body(Body::from("404 Not Found"))
            .unwrap()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup hyper server
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(assets)) });
    let addr = ([127, 0, 0, 1], 0).into();
    let server = Server::bind(&addr).serve(make_svc);
    let port = server.local_addr().port();

    // Start hyper server in separate thread
    tokio::spawn(server);

    // Start web view in current thread
    web_view::builder()
        .title("Conduit")
        .content(Content::Url(format!("http://127.0.0.1:{}", port)))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();

    Ok(())
}
