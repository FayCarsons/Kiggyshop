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
        email -> Text,
        total -> Integer,
        shipped -> Bool,
        tracking_number -> Nullable<Text>,
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

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        ip -> Text,
        user_agent -> Nullable<Text>,
        device -> Text,
        time -> Nullable<Timestamp>,
        country -> Text,
        state -> Text,
        city -> Nullable<Text>,
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
    users,
);
