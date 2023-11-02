use std::{fs::File, io::Read};

use actix_web::{
    get, post,
    web::{self, Redirect},
    HttpResponse,
};
use serde::Deserialize;

use crate::error::ShopResult;

#[derive(Debug, Clone, Deserialize)]
struct Password {
    password: String,
}

fn is_valid_password(pass: &str) -> bool {
    pass == std::env::var("ADMIN_PASS").unwrap_or_default()
}

#[get("/admin")]
pub async fn login() -> HttpResponse {
    let mut file = File::open("./resources/admin/login.html").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    HttpResponse::Ok().content_type("text/html").body(buffer)
}

#[post("/admin")]
pub async fn try_login(web::Form(form): web::Form<Password>) -> ShopResult<Redirect> {
    println!("{:?}", &form);

    if is_valid_password(&form.password) {
        Ok(Redirect::to("/admin/dashboard").permanent())
    } else {
        Ok(Redirect::to("/admin/unauthorized").permanent())
    }
}

#[post("/admin/dashboard")]
pub async fn dashboard() -> HttpResponse {
    let mut file = File::open("./resources/admin/dashboard.html").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    HttpResponse::Ok().content_type("text/html").body(buffer)
}

#[post("/admin/unauthorized")]
pub async fn unauthorized() -> HttpResponse {
    let mut file = File::open("./resources/admin/unauthorized.html").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    HttpResponse::Ok().content_type("text/html").body(buffer)
}

#[get("/admin/dashboard/style.css")]
pub async fn get_style() -> HttpResponse {
    let mut file = File::open("./resources/admin/style.css").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    HttpResponse::Ok().content_type("text/css").body(buffer)
}
