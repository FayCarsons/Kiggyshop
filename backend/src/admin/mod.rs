use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

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

fn get_session(session: &Session) -> ShopResult<bool> {
    match session.get::<bool>("admin-access") {
        Ok(msg) => msg.map_or(Ok(false), Ok),
        Err(e) => Err(BackendError::ContentNotFound(e.to_string())),
    }
}

fn set_session(session: Session) -> ShopResult<()> {
    match session.insert("admin-access", true) {
        x @ Ok(()) => x.map_err(|_| BackendError::Unauthorized),
        _ => Err(BackendError::Unauthorized),
    }
}

fn is_valid_password(pass: &str) -> bool {
    pass == ENV.get().cloned().unwrap_or_default().admin_pass
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
    if is_valid_password(&form.password) {
        set_session(session)?;
        Ok(Redirect::to("/admin/dashboard").permanent())
    } else {
        Ok(Redirect::to("/admin").permanent())
    }
}

#[get("/dashboard")]
pub async fn get_dashboard(session: Session) -> ShopResult<Redirect> {
    if !get_session(&session)? {
        return Ok(Redirect::to("/admin").permanent());
    }
    Ok(Redirect::to("/admin/dashboard/auth").permanent())
}

#[post("/dashboard")]
pub async fn post_dashboard(session: Session) -> ShopResult<Redirect> {
    if !get_session(&session)? {
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

#[get("/dashboard/dashboard.js")]
pub async fn get_js() -> ShopResult<HttpResponse> {
    let buffer = fs::read_to_string("./resources/admin/dashboard.js")?;
    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(buffer))
}

const MAX_SIZE: usize = 10_000_000;
#[post("/upload_image/{item_title}")]
pub async fn upload_image(
    mut payload: web::Payload,
    item_title: web::Path<String>,
) -> ShopResult<HttpResponse> {
    use futures::StreamExt;

    let path = format!(
        "./resources/images/{}.png",
        item_title.trim().replace(' ', "")
    );
    if Path::new(&path).exists() {
        fs::remove_file(&path).unwrap();
    }

    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        let curr_len = body.len() + chunk.len();
        if curr_len > MAX_SIZE {
            return Err(BackendError::BadRequest("overflow".to_string()));
        }
        body.extend_from_slice(&chunk);
    }

    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open(path)?;
    let res = match file.write_all(&body) {
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        Ok(()) => HttpResponse::Ok().finish(),
    };

    Ok(res)
}
