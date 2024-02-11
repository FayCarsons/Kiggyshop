#[cfg(test)]
mod tests {
    use diesel::RunQueryDsl;

    use crate::{
        model::{
            cart::{JsonCart, NewCart},
            order::{JsonOrder, NewOrder},
        },
        tests::test_db,
    };

    #[test]
    fn create_db() {
        println!("Creating a database");
        let _ = test_db::TestDb::new();
    }

    #[test]
    fn connect_db() {
        let conn = test_db::TestDb::new();
        conn.connection();
    }

    #[actix_web::test]
    async fn insert_order() {
        let order = JsonOrder {
            name: String::from("William Burroughs"),
            street: String::from("3616 E Broad st, Richond, VA"),
            zipcode: String::from("23223"),
            total: 108,
            cart: vec![JsonCart { item: 0, qty: 4 }],
        };

        let db = test_db::TestDb::new();
        let mut conn = db.connection();

        use crate::schema::{carts, orders};

        let inserted_id = diesel::insert_into(orders::table)
            .values(&NewOrder {
                name: &order.name,
                street: &order.street,
                zipcode: &order.zipcode,
                fulfilled: false,
            })
            .returning(orders::dsl::id)
            .get_result::<i32>(&mut conn);

        assert!(inserted_id.is_ok());

        let new_carts = order
            .cart
            .into_iter()
            .map(|JsonCart { item, qty }| NewCart {
                order_id: *inserted_id.as_ref().unwrap(),
                item_id: item,
                quantity: qty,
            })
            .collect::<Vec<NewCart>>();

        let insert = diesel::insert_into(carts::table)
            .values(&new_carts)
            .execute(&mut conn);

        assert!(insert.is_ok())
    }
}
