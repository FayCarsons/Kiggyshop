// @generated automatically by Diesel CLI.

diesel::table! {
    addresses (id) {
        id -> Integer,
        order -> Integer,
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
        order -> Integer,
        item -> Integer,
    }
}

diesel::table! {
    orders (id) {
        id -> Integer,
        name -> Text,
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

diesel::joinable!(addresses -> orders (order));
diesel::joinable!(carts -> orders (order));
diesel::joinable!(carts -> stock (item));

diesel::allow_tables_to_appear_in_same_query!(addresses, carts, orders, stock,);
