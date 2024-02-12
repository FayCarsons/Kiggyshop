#[cfg(test)]
mod tests {
    use std::fs::{self};

    use diesel::{dsl::count, query_dsl::methods::SelectDsl, RunQueryDsl};

    use crate::{
        model::{
            cart::{JsonCart, NewCart},
            item::{InputItem, NewItem},
            order::{JsonOrder, NewOrder},
        },
        tests::test_db,
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
        let order = JsonOrder {
            name: String::from("William Burroughs"),
            street: String::from("2323 E Broad st, Richond, VA"),
            zipcode: String::from("23223"),
            total: 108,
            cart: vec![JsonCart { item: 0, qty: 4 }, JsonCart { item: 1, qty: 2 }, ],
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

    #[test]
    fn insert_stock() {
        use crate::schema::stock::{self, id};
        let stock = fs::read_to_string("stock.json").unwrap();
        let stock: Vec<InputItem> = serde_json::from_str(&stock).unwrap();

        let num_items = stock.len();

        let db = test_db::TestDb::new();
        let mut conn = db.connection();

        let ins: Vec<NewItem> = stock
            .iter()
            .map(
                |InputItem {
                     title,
                     kind,
                     description,
                     quantity,
                 }| NewItem {
                    title: title,
                    kind: kind,
                    description: description,
                    quantity: *quantity,
                },
            )
            .collect();

        let res = diesel::insert_into(stock::table)
            .values(ins)
            .execute(&mut conn);

        assert!(res.is_ok());

        let res = SelectDsl::select(stock::table, count(id)).get_result::<i64>(&mut conn);
        assert!(res.is_ok());
        assert_eq!(num_items, res.unwrap() as usize);
    }
}
