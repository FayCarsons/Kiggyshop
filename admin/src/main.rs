#[cfg(test)]
mod tests;

use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use actix_session::Session;

use actix_web::{
    get, post,
    web::{self, Redirect},
    App, HttpResponse, HttpServer, Result,
};

use serde::Deserialize;

#[derive(Debug, Clone)]
struct AppData {
    auth: String,
}

#[derive(Debug, Clone, Deserialize)]
struct Password {
    password: String,
}

fn get_session(session: &Session) -> Option<bool> {
    session.get::<bool>("admin-access").ok().flatten()
}

fn set_session(session: Session) -> Result<()> {
    Ok(session.insert("admin-access", true)?)
}

#[post("/login")]
pub async fn login(
    web::Form(form): web::Form<Password>,
    app_data: web::Data<AppData>,
    session: Session,
) -> Result<Redirect> {
    if form.password == app_data.auth {
        set_session(session)?;
        Ok(Redirect::to("/admin/dashboard").permanent())
    } else {
        Ok(Redirect::to("/admin").permanent())
    }
}

#[get("/dashboard")]
pub async fn dashboard(session: Session) -> Result<Redirect> {
    if let Some(false) = get_session(&session) {
        return Ok(Redirect::to("/admin").permanent());
    }
    Ok(Redirect::to("/admin/dashboard/auth").permanent())
}

#[get("/dashboard/auth")]
pub async fn admin_dashboard() -> Result<HttpResponse> {
    let buffer = fs::read_to_string("./resources/admin/dashboard.html")?;
    Ok(HttpResponse::Ok().content_type("text/html").body(buffer))
}

const MAX_SIZE: usize = 10_000_000;
#[post("/upload_image/{item_title}")]
pub async fn upload_image(
    mut payload: web::Payload,
    item_title: web::Path<String>,
) -> Result<HttpResponse> {
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
            return Err(actix_web::error::ErrorPayloadTooLarge(""));
        };

        body.extend_from_slice(&chunk);
    }

    let mut file = OpenOptions::new().create_new(true).write(true).open(path)?;

    Ok(file
        .write_all(&body)
        .map(|_| HttpResponse::Created().finish())?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect(".env file not found!");
    let auth = std::env::var("PASSWORD")
        .expect("Admin password not found!")
        .to_string();
    let app_data = AppData { auth };
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_data.clone()))
            .service(login)
            .service(dashboard)
            .service(admin_dashboard)
            .service(upload_image)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
