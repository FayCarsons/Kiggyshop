// @generated automatically by Diesel CLI.

diesel::table! {
    carts (id) {
        id -> Integer,
        order_id -> Integer,
        item_id -> Integer,
        quantity -> Integer,
    }
}

diesel::table! {
    orders (id) {
        id -> Integer,
        name -> Text,
        street -> Text,
        zipcode -> Text,
        fulfilled -> Bool,
    }
}

diesel::table! {
    stock (id) {
        id -> Integer,
        title -> Text,
        kind -> Text,
        description -> Text,
        quantity -> Integer,
    }
}

diesel::joinable!(carts -> orders (order_id));

diesel::allow_tables_to_appear_in_same_query!(
    carts,
    orders,
    stock,
);
