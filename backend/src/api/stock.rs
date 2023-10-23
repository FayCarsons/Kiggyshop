use common::{
    item::{AdminItem, Item},
    Stock,
};

use actix_web::{
    delete, get, post, put,
    web::{Path, ReqData},
    HttpResponse,
};
use std::{
    fs::{File, OpenOptions},
    io::{ErrorKind, Read, Write},
};

#[get("/stock/get")]
pub async fn get_stock() -> HttpResponse {
    println!("\n BACKEND STOCK/GET \n");

    let mut file = File::open("stock.json").expect("CANNOT ACCESS STOCK DATA");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .expect("CANNOT WRITE FILE TO BUFFER");

    let data: Stock = serde_json::from_str(&buffer).expect("CANNOT DESERIALIZE");
    let ser_stock = serde_json::to_string_pretty(&data).expect("CANNOT SERIALIZE");

    HttpResponse::Ok().content_type("text/json").body(ser_stock)
}

#[post("/stock/add")]
pub async fn add_item(item: ReqData<AdminItem>) -> Result<HttpResponse, std::io::Error> {
    let mut file = File::open("stock.json").expect("CANNOT ACCESS STOCK DATA");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .expect("CANNOT WRITE FILE TO BUFFER");

    let stock: Stock = serde_json::from_str(&buffer).expect("CANNOT DESERIALIZE STOCK");
    let new_item = Item::from_admin(item.into_inner())
        .map_err(|e| std::io::Error::new(ErrorKind::Other, e.to_string()))?;
    let new_stock = stock
        .stock
        .into_iter()
        .chain(std::iter::once(new_item))
        .collect::<Stock>();

    update_stock(new_stock)?;

    Ok(HttpResponse::Ok().finish())
}

#[delete("/stock/delete/{name}")]
pub async fn delete_item(name: Path<String>) -> Result<HttpResponse, std::io::Error> {
    let name = name.into_inner();

    let mut file = File::open("stock.json").expect("CANNOT ACCESS STOCK DATA");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .expect("CANNOT WRITE FILE TO BUFFER");

    let stock: Stock = serde_json::from_str(&buffer).expect("CANNOT DESERIALIZE STOCK");
    let new_stock = stock
        .stock
        .into_iter()
        .filter(|item| item.title != name)
        .collect::<Stock>();

    update_stock(new_stock)?;

    Ok(HttpResponse::Ok().finish())
}

fn update_stock(stock: Stock) -> Result<(), std::io::Error> {
    let new_ser = serde_json::to_string_pretty(&stock)?;

    let mut new_data = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("stock.json")?;
    new_data.write(new_ser.as_bytes())?;

    Ok(())
}
