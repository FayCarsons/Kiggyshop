use std::fs::{self};

use actix_session::Session;
use actix_web::{
    get, post,
    web::{self, Redirect},
    HttpResponse,
};
use serde::Deserialize;

use crate::{
    error::{BackendError, ShopResult},
    ENV,
};

#[derive(Debug, Clone, Deserialize)]
struct Password {
    password: String,
}

fn get_session(session: Session) -> ShopResult<bool> {
    match session.get::<bool>("admin-access") {
        Ok(msg) => match msg {
            Some(access) => Ok(access),
            None => Ok(false),
        },
        Err(e) => Err(BackendError::ContentNotFound(e.to_string())),
    }
}

fn set_session(session: Session) -> ShopResult<()> {
    match session.insert("admin-access", true) {
        Ok(_) => Ok(()),
        _ => Err(BackendError::Unauthorized),
    }
}

fn is_valid_password(pass: &str) -> bool {
    pass == &ENV.get().cloned().unwrap_or_default().admin_pass
}

#[get("")]
pub async fn login() -> ShopResult<HttpResponse> {
    let buffer = fs::read_to_string("./resources/admin/login.html")?;
    Ok(HttpResponse::Ok().content_type("text/html").body(buffer))
}

#[post("")]
pub async fn try_login(
    web::Form(form): web::Form<Password>,
    session: Session,
) -> ShopResult<Redirect> {
    println!("{:?}", &form);

    if is_valid_password(&form.password) {
        set_session(session)?;
        Ok(Redirect::to("/admin/dashboard").permanent())
    } else {
        Ok(Redirect::to("/admin").permanent())
    }
}

#[get("/dashboard")]
pub async fn get_dashboard(session: Session) -> ShopResult<Redirect> {
    if !get_session(session)? {
        return Ok(Redirect::to("/admin").permanent());
    }
    Ok(Redirect::to("/admin/dashboard/auth").permanent())
}

#[post("/dashboard")]
pub async fn post_dashboard(session: Session) -> ShopResult<Redirect> {
    if !get_session(session)? {
        return Ok(Redirect::to("/admin").permanent());
    }
    Ok(Redirect::to("/admin/dashboard/auth").permanent())
}

#[post("/dashboard/auth")]
pub async fn post_admin_dashboard() -> ShopResult<HttpResponse> {
    let buffer = fs::read_to_string("./resources/admin/dashboard.html")?;
    Ok(HttpResponse::Ok().content_type("text/html").body(buffer))
}

#[get("/dashboard/auth")]
pub async fn get_admin_dashboard() -> ShopResult<HttpResponse> {
    let buffer = fs::read_to_string("./resources/admin/dashboard.html")?;
    Ok(HttpResponse::Ok().content_type("text/html").body(buffer))
}

#[get("/dashboard/style.css")]
pub async fn get_style() -> ShopResult<HttpResponse> {
    let buffer = fs::read_to_string("./resources/admin/style.css")?;
    Ok(HttpResponse::Ok().content_type("text/css").body(buffer))
}
