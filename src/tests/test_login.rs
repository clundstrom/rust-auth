use actix_web::{http::header::ContentType, test, App};
use crate::{ping};

#[test]
async fn test_index_get() {
    let app = test::init_service(App::new().service(ping)).await;
    let req = test::TestRequest::default()
        .insert_header(ContentType::plaintext())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}
