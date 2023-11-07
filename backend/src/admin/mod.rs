use std::{fs::{File, self}, io::Read};

use actix_web::{
    get, post,
    web::{self, Redirect},
    HttpResponse,
};
use serde::Deserialize;

use crate::{error::ShopResult, ENV};

#[derive(Debug, Clone, Deserialize)]
struct Password {
    password: String,
}

fn is_valid_password(pass: &str) -> bool {
    pass == &ENV.get().cloned().unwrap_or_default().admin_pass
}

#[get("/admin")]
pub async fn login() -> ShopResult<HttpResponse> {
    let buffer = fs::read_to_string("./resources/admin/login.html")?;
    Ok(HttpResponse::Ok().content_type("text/html").body(buffer))
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
pub async fn dashboard() -> ShopResult<HttpResponse> {
    let buffer = fs::read_to_string("./resources/admin/dashboard.html")?;
    Ok(HttpResponse::Ok().content_type("text/html").body(buffer))
}

#[post("/admin/unauthorized")]
pub async fn unauthorized() -> ShopResult<HttpResponse> {
    let buffer = fs::read_to_string("./resources/admin/unauthorized.html")?;
    Ok(HttpResponse::Ok().content_type("text/html").body(buffer))
}

#[get("/admin/dashboard/style.css")]
pub async fn get_style() -> ShopResult<HttpResponse> {
    let buffer = fs::read_to_string("./resources/admin/style.css")?;
    Ok(HttpResponse::Ok().content_type("text/css").body(buffer))
}
