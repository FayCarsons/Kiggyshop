// @generated automatically by Diesel CLI.

diesel::table! {
    addresses (id) {
        name -> Text,
        id -> Integer,
        order_id -> Integer,
        number -> Integer,
        street -> Text,
        city -> Text,
        state -> Text,
        zipcode -> Integer,
    }
}

diesel::table! {
    carts (id) {
        id -> Integer,
        quantity -> Integer,
        order_id -> Integer,
        item_id -> Integer,
    }
}

diesel::table! {
    orders (id) {
        id -> Integer,
        name -> Text,
        total -> Integer,
        email -> Text,
        shipped -> Bool,
    }
}

diesel::table! {
    stock (id) {
        id -> Integer,
        title -> Text,
        kind -> Integer,
        description -> Text,
        quantity -> Integer,
    }
}

diesel::joinable!(addresses -> orders (order_id));
diesel::joinable!(carts -> orders (order_id));
diesel::joinable!(carts -> stock (item_id));

diesel::allow_tables_to_appear_in_same_query!(
    addresses,
    carts,
    orders,
    stock,
);
