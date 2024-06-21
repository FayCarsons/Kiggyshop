#[cfg(test)]
mod tests {
    use diesel::{r2d2::ConnectionManager, QueryDsl, RunQueryDsl};
    use std::collections::HashMap;

    use crate::{
        api::{order::delete_order, stock::get_stock},
        tests::test_db,
    };
    use actix_web::{test, web, App};
    use diesel::SqliteConnection;
    use model::{
        item::{Item, NewItem},
        order::{NewOrder, Order},
        ItemId,
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
        let stock: Vec<Item> = serde_json::from_str(stock).expect("Cannot deserialize stock.json");
        let stock: Vec<NewItem> = stock.iter().map(NewItem::from).collect();

        let mut conn = db.connection();
        diesel::insert_into(model::schema::stock::table)
            .values(stock)
            .execute(&mut conn)
            .expect("Cannot insert stock.json into DB");
        let app =
            test::init_service(App::new().app_data(web::Data::new(pool)).service(get_stock)).await;
        let req = test::TestRequest::get().uri("/stock").to_request();
        let dummy = test::call_service(&app, req).await;
        assert!(dummy.status().is_success());
        let req = test::TestRequest::get().uri("/stock").to_request();
        let _: HashMap<ItemId, Item> = test::try_call_and_read_body_json(&app, req)
            .await
            .expect("Cannot deserialize body");
    }

    #[actix_web::test]
    async fn test_delete_order() {
        let (db, pool) = create_db_pool();

        let Order { name, .. } = serde_json::from_str::<Order>(include_str!("./mock_order.json"))
            .expect("Cannot deserialize mock order");

        let mut conn = db.connection();
        diesel::insert_into(model::schema::orders::table)
            .values([NewOrder {
                name: &name,
                total: 30_00,
                email: "",
                shipped: false,
            }])
            .execute(&mut conn)
            .expect("Cannot insert mock order into DB");

        let app = test::init_service(
            App::new()
                .service(delete_order)
                .app_data(web::Data::new(pool)),
        )
        .await;
        let req = test::TestRequest::delete().uri("/orders/1").to_request();
        let response = test::call_service(&app, req).await;
        assert!(response.status().is_success());

        let mut conn = db.connection();
        assert_eq!(model::schema::orders::table.count().first(&mut conn), Ok(0))
    }
}
