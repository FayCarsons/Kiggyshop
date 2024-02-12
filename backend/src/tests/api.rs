#[cfg(test)]
mod tests {
    use diesel::r2d2::ConnectionManager;
    use std::{collections::HashMap, fs};

    use actix_web::{
        body::MessageBody,
        http::header::ContentType,
        test,
        web::{self, JsonBody},
        App,
    };
    use diesel::SqliteConnection;

    use crate::{
        admin::upload_image, api::stock::get_stock, model::{item::Item, ItemId}, tests::test_db
    };

    #[actix_web::test]
    async fn test() {
        let db = test_db::TestDb::new();
        let path = db.path();
        let manager = ConnectionManager::<SqliteConnection>::new(
            path.to_str().unwrap()
        );
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("INVALID DB URL // DB POOL CANNOT BE BUILT");
        let app =
            test::init_service(App::new().app_data(web::Data::new(pool)).service(get_stock)).await;
        let req = test::TestRequest::get().uri("/stock/get").to_request();
        let _: HashMap<ItemId, Item> = test::call_and_read_body_json(&app, req).await;
    }

    #[actix_web::test]
    async fn test_image_upload() {
        let image = include_bytes!("./cat.png");
        let app = test::init_service(App::new().service(upload_image)).await;
        let req = test::TestRequest::post().set_payload(image.to_vec()).uri("/upload_image/test").to_request();
        let response = test::call_service(&app, req).await;
        assert!(response.status().is_success());
        fs::remove_file("resources/images/test.png").unwrap();
    }
}
