use actix_web::{
    get, put,
    web::{Json, Path},
    HttpResponse,
};
use common::{order::Order, OrderList};
use serde::Deserialize;
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

#[derive(Deserialize)]
enum OrderFilter {
    All,
    Unfulfilled,
    Fulfilled,
}

#[get("/orders/{filter}")]
pub async fn get_orders(filter: Path<OrderFilter>) -> HttpResponse {
    let mut file = File::open("orders.json").expect("CANNOT FIND ORDERS.JSON");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .expect("CANNOT WRITE FILE TO BUFFER");

    let orders: OrderList = serde_json::from_str(&buffer).expect("CANNOT DESERIALIZE ORDERS");

    let res = match filter.into_inner() {
        OrderFilter::All => orders.orders,
        OrderFilter::Fulfilled => orders
            .orders
            .into_iter()
            .filter(|order| order.fulfilled)
            .collect(),
        OrderFilter::Unfulfilled => orders
            .orders
            .into_iter()
            .filter(|order| !order.fulfilled)
            .collect(),
    };

    let body = serde_json::to_string_pretty(&OrderList::from(res))
        .expect("CANNOT RESERIALIZE UNfulfilled ORDERS");
    HttpResponse::Ok().content_type("text/json").body(body)
}

#[put("/orders/new")]
pub async fn put_order(order: Json<Order>) -> HttpResponse {
    let mut file = File::open("orders.json").expect("CANNOT FIND ORDERS.JSON");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer);

    let old: OrderList = serde_json::from_str(&buffer).expect("CANNOT PARSE ORDERS FOR PUT");

    let new = OrderList::from(
        old.orders
            .into_iter()
            .chain(std::iter::once(order.into_inner()))
            .collect::<Vec<Order>>(),
    );

    let ser = serde_json::to_string_pretty(&new).expect("CANNOT RESERIALIZE ORDERS FOR PUT");
    let mut new_data = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("orders.json")
        .expect("CANNOT WRITE NEW ORDERS.JSON FOR PUT");
    new_data
        .write(ser.as_bytes())
        .expect("CANNOT WRITE UPDATED ORDERS TO FILE FOR PUT");
    HttpResponse::Ok().body("")
}
