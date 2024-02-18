use actix_web::{test, App};
use crate::upload_image;

#[actix_web::test]
async fn test_image_upload() {
    let image = &[0; 1]; //include_bytes!("./cat.png");
    let app = test::init_service(App::new().service(upload_image)).await;
    let req = test::TestRequest::post()
        .set_payload(image.to_vec())
        .uri("/upload_image/test")
        .to_request();
    let response = test::call_service(&app, req).await;
    assert!(response.status().is_success());
    std::fs::remove_file("resources/images/test.png").unwrap();
}
