#![allow(warnings)]
use axum_server::tls_rustls::RustlsConfig;
use prest::*;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let router: Router<()> = route(
        "/",
        get(html!((DOCTYPE) html {
            (Head::with_title("With HTTPS"))
            body {h1{"Check out the connection / protocol!"}}
        })),
    );

    // init http -> https redirection service
    tokio::spawn(redirect_to_origin("https://localhost"));

    let tls_config = RustlsConfig::from_pem_file("./cert.pem", "./key.pem")
        .await
        .unwrap();

    let https_addr = SocketAddr::from(([127, 0, 0, 1], 443));

    axum_server::bind_rustls(https_addr, tls_config)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn redirect_to_origin<N: AsRef<str>>(origin: N) {
    use axum::handler::HandlerWithoutStateExt;
    let origin = origin.as_ref().to_owned();
    let redirect = |uri: http::Uri| async move {
        let path = uri.path_and_query().map(|pq| pq.as_str()).unwrap_or("/");
        let target = format!("{origin}{path}");
        Redirect::permanent(&target)
    };
    axum_server::bind(SocketAddr::from(([127, 0, 0, 1], 80)))
        .serve(redirect.into_make_service())
        .await.unwrap();
}
