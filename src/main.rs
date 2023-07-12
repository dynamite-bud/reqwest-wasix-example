use hyper::{Body, Client, Request, StatusCode, Uri};

#[tokio::main]
async fn main() {
    let url: Uri = ("https://hyper.rs").parse().unwrap();

    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_webpki_roots()
        .https_only()
        .build();

    let client: Client<_, hyper::Body> = Client::builder().build(https);

    let request = Request::builder()
        .method("GET")
        .uri(url)
        .body(hyper::Body::empty())
        .expect("request builder should always be able to build a body");

    let res = client.request(request).await.unwrap();

    dbg!(&res);
    assert_eq!(res.status(), StatusCode::OK);
}
