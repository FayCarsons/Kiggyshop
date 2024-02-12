#[cfg(test)]
mod tests {
    use diesel::{r2d2::ConnectionManager, RunQueryDsl};
    use std::{collections::HashMap, fs};

    use actix_web::{test, web, App};
    use diesel::SqliteConnection;

    use crate::{
        admin::upload_image,
        api::{order, stock::get_stock},
        model::{
            cart::JsonCart,
            item::{InputItem, Item, NewItem},
            order::JsonOrder,
            ItemId,
        },
        tests::test_db,
    };

    fn create_db_pool() -> (
        test_db::TestDb,
        diesel::r2d2::Pool<ConnectionManager<diesel::SqliteConnection>>,
    ) {
        let db = test_db::TestDb::new();
        let path = db.path();
        let manager = ConnectionManager::<SqliteConnection>::new(path.to_str().unwrap());
        (
            db,
            r2d2::Pool::builder()
                .build(manager)
                .expect("INVALID DB URL // DB POOL CANNOT BE BUILT"),
        )
    }

    #[actix_web::test]
    async fn test_get_stock() {
        let (db, pool) = create_db_pool();

        let stock = include_str!("../../stock.json");
        let stock: Vec<InputItem> =
            serde_json::from_str(&stock).expect("Cannot deserialize stock.json");
        let stock: Vec<NewItem> = stock
            .iter()
            .map(
                |InputItem {
                     title,
                     kind,
                     description,
                     quantity,
                 }| {
                    NewItem {
                        title,
                        kind,
                        description,
                        quantity: *quantity,
                    }
                },
            )
            .collect();

        let mut conn = db.connection();
        diesel::insert_into(crate::schema::stock::table)
            .values(stock)
            .execute(&mut conn)
            .expect("Cannot insert stock.json into DB");
        let app =
            test::init_service(App::new().app_data(web::Data::new(pool)).service(get_stock)).await;
        let req = test::TestRequest::get().uri("/stock/get").to_request();
        let dummy = test::call_service(&app, req).await;
        assert!(dummy.status().is_success());
        let req = test::TestRequest::get().uri("/stock/get").to_request();
        let _: HashMap<ItemId, InputItem> = test::try_call_and_read_body_json(&app, req)
            .await
            .expect("Cannot deserialize body");
    }

    // #[actix_web::test]
    // Not working - for soe reason FS is locked for second call
    async fn _test_insert_delete_order() {
        // Initialize test double DB and get connection pool
        let (_, pool) = create_db_pool();

        // Dummy order
        let order = serde_json::from_str::<JsonOrder>(include_str!("./mock_order.json"))
            .expect("Cannot deserialize mock order");

        // Create app, add pool and insert/delete services
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool))
                .service(order::post_order)
                .service(order::delete_order),
        )
        .await;

        // Build request with dummy order JSON body
        let req = test::TestRequest::post()
            .uri("/orders")
            .set_json(order)
            .to_request();
        // Make request to `post_order`
        test::call_service(&app, req).await;

        // Build new request to `delete_order` with received order id
        let req = test::TestRequest::delete()
            .uri("/orders/1")
            .to_request();
        // Send request and verify success
        let response = test::call_service(&app, req).await;
        println!("{:?}", response.status());
        assert!(response.status().is_success());

        // TODO: assert DB contains no orders
    }

    #[actix_web::test]
    async fn test_image_upload() {
        let image = include_bytes!("./cat.png");
        let app = test::init_service(App::new().service(upload_image)).await;
        let req = test::TestRequest::post()
            .set_payload(image.to_vec())
            .uri("/upload_image/test")
            .to_request();
        let response = test::call_service(&app, req).await;
        assert!(response.status().is_success());
        fs::remove_file("resources/images/test.png").unwrap();
    }
}
