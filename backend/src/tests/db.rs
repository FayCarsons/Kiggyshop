#[cfg(test)]
mod tests {
    use std::fs;

    use crate::tests::test_db;
    use diesel::{
        dsl::count, query_dsl::methods::SelectDsl, ExpressionMethods, QueryDsl, RunQueryDsl,
    };
    use model::{
        cart::NewCart,
        item::{Item, NewItem},
        order::{NewOrder, Order},
    };

    #[test]
    fn create_db() {
        let _ = test_db::TestDb::new();
    }

    #[test]
    fn connect_db() {
        let conn = test_db::TestDb::new();
        conn.connection();
    }

    #[test]
    fn insert_order() {
        let Order { name, ref cart, .. } = serde_json::from_str(include_str!("./mock_order.json"))
            .expect("Cannot open mock order");

        let db = test_db::TestDb::new();
        let mut conn = db.connection();

        use model::schema::{carts, orders};

        let inserted_id = diesel::insert_into(orders::table)
            .values(&NewOrder {
                name: &name,
                email: "",
                total: 30_00,
                shipped: false,
            })
            .returning(orders::dsl::id)
            .get_result::<i32>(&mut conn);

        assert!(inserted_id.is_ok());
        assert_eq!(inserted_id, Ok(1));
        assert_eq!(model::schema::orders::table.count().first(&mut conn), Ok(1));

        let new_carts = cart
            .iter()
            .map(|(item, qty)| NewCart {
                order_id: *inserted_id.as_ref().unwrap(),
                item_id: *item as i32,
                quantity: *qty as i32,
            })
            .collect::<Vec<NewCart>>();

        let insert = diesel::insert_into(carts::table)
            .values(&new_carts)
            .execute(&mut conn);

        assert!(insert.is_ok());
        assert_eq!(
            model::schema::carts::table.count().first::<i64>(&mut conn),
            Ok(cart.len() as i64)
        );
    }

    #[test]
    fn test_update_order() {
        let Order { name, .. } = serde_json::from_str(include_str!("./mock_order.json"))
            .expect("Cannot open mock order");

        let db = test_db::TestDb::new();
        let mut conn = db.connection();

        use model::schema::orders;

        let inserted_id = diesel::insert_into(orders::table)
            .values(&NewOrder {
                name: &name,
                email: "",
                total: 30_00,
                shipped: false,
            })
            .returning(orders::dsl::id)
            .get_result::<i32>(&mut conn);

        assert!(inserted_id.is_ok());
        assert_eq!(inserted_id, Ok(1));
        assert_eq!(model::schema::orders::table.count().first(&mut conn), Ok(1));

        let res = diesel::update(
            model::schema::orders::table.filter(model::schema::orders::id.eq(inserted_id.unwrap())),
        )
        .set(model::schema::orders::shipped.eq(true))
        .execute(&mut conn);
        assert!(res.is_ok());

        assert_eq!(
            model::schema::orders::table
                .filter(model::schema::orders::shipped.eq(true))
                .count()
                .first(&mut conn),
            Ok(1)
        );
    }

    #[test]
    fn test_delete_order() {
        let Order { name, .. } = serde_json::from_str(include_str!("./mock_order.json"))
            .expect("Cannot open mock order");

        let db = test_db::TestDb::new();
        let mut conn = db.connection();

        use model::schema::orders;

        let inserted_id = diesel::insert_into(orders::table)
            .values(NewOrder {
                name: &name,
                email: "",
                total: 30_00,
                shipped: false,
            })
            .returning(orders::dsl::id)
            .get_result::<i32>(&mut conn);

        assert!(inserted_id.is_ok());
        assert_eq!(inserted_id, Ok(1));
        assert_eq!(model::schema::orders::table.count().first(&mut conn), Ok(1));

        let res = diesel::delete(model::schema::orders::table).execute(&mut conn);
        assert!(res.is_ok());

        assert_eq!(model::schema::orders::table.count().first(&mut conn), Ok(0));
    }

    #[test]
    fn insert_stock() {
        use model::schema::stock::{self, id};
        let stock = fs::read_to_string("stock.json").unwrap();
        let stock: Vec<Item> = serde_json::from_str(&stock).unwrap();

        let num_items = stock.len();

        let db = test_db::TestDb::new();
        let mut conn = db.connection();

        let ins: Vec<NewItem> = stock.iter().map(NewItem::from).collect();

        let res = diesel::insert_into(stock::table)
            .values(ins)
            .execute(&mut conn);

        assert!(res.is_ok());

        let res = SelectDsl::select(stock::table, count(id)).get_result::<i64>(&mut conn);
        assert!(res.is_ok());
        assert_eq!(num_items, res.unwrap() as usize);
    }
}
