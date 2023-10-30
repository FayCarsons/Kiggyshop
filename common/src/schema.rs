// @generated automatically by Diesel CLI.

diesel::table! {
    carts (id) {
        id -> Integer,
        order_id -> Integer,
        item_name -> Text,
        quantity -> Integer,
    }
}

diesel::table! {
    orders (id) {
        id -> Integer,
        name -> Text,
        street -> Text,
        zipcode -> Integer,
        fulfilled -> Bool,
    }
}

diesel::table! {
    stock (id) {
        id -> Nullable<Integer>,
        title -> Text,
        kind -> Text,
        description -> Text,
        quantity -> Integer,
    }
}

diesel::joinable!(carts -> orders (order_id));

diesel::allow_tables_to_appear_in_same_query!(carts, orders, stock,);
