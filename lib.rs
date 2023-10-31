#![allow(dead_code, unused_imports)]
mod html;
mod embed;

pub(crate) use crate as prest;

pub use anyhow::{self, Error, Result, bail};
pub use axum::{
    self,
    body::{Body, HttpBody},
    Form,
    extract::*,
    response::*,
    routing::{any, delete, get, patch, post, put},
    Router,
    middleware::*,
};
pub use embed::*;
pub use embed_macro::*;
pub use embed_utils::*;
pub use html::*;
pub use html_macro::html;
pub use http::{self, Uri, header, HeaderMap, HeaderValue, StatusCode};
pub use tower::{self, Layer, Service};
pub use once_cell::sync::Lazy;

pub const DOCTYPE: PreEscaped<&'static str> = PreEscaped("<!DOCTYPE html>");
pub const REGISTER_SW_SNIPPET: &str = 
    "if ('serviceWorker' in navigator) navigator.serviceWorker.register('sw.js', {type: 'module'});";

pub static DIST_DIR: Lazy<String> = Lazy::new(|| {
    let dir = format!("{}/dist", std::env::var("OUT_DIR").unwrap());
    std::fs::create_dir_all(&dir).unwrap();
    dir
});

pub fn out_path(filename: &str) -> String {
    format!("{}/{filename}", *DIST_DIR)
}

use std::net::SocketAddr;
pub struct ServeOptions {
    pub addr: SocketAddr,
}
impl Default for ServeOptions {
    fn default() -> Self {
        Self {
            addr: SocketAddr::from(([0, 0, 0, 0], 80))
        }
    }
}

#[cfg(feature = "build-pwa")]
mod build_pwa;
#[cfg(feature = "build-pwa")]
pub use build_pwa::*;

#[cfg(feature = "sw")]
mod sw;
#[cfg(feature = "sw")]
pub use sw::*;

#[cfg(feature = "host")]
pub async fn serve(router: Router, opts: ServeOptions) {
    let svc = router.into_make_service();
    hyper_server::bind(opts.addr).serve(svc).await.unwrap();
}

#[cfg(all(target = "wasm32-wasi", feature = "host-wasi"))]
pub async fn serve(router: Router, opts: ServeOptions) { 
    use hyper::server::conn::Http;
    use tokio::net::TcpListener;       
    let listener = TcpListener::bind(opts.addr).await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let svc = router.clone();
        tokio::task::spawn(async move {
            if let Err(err) = Http::new().serve_connection(stream, svc).await {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}    
