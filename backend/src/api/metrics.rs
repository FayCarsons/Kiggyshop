// NOTE: The functions in this module ignore `Err` values because failing to log a visit is ok
use actix_web::{web, HttpRequest, Result};
use awc::Client;
use chrono::prelude::*;
use model::{schema::users, user};

use crate::DbConn;

async fn get_location(ip: &str) -> Result<user::Location, ()> {
    actix_web::rt::System::new().block_on(async {
        let client = Client::default();
        client
            .get(format!("http://ip-api.com/json/{ip}"))
            .send()
            .await
            .map_err(|_| ())?
            .json::<user::Location>()
            .await
            .map_err(|_| ())
    })
}

async fn get_user(req: HttpRequest) -> Result<user::User, String> {
    let time = Utc::now().naive_utc();
    let ip = req.peer_addr().map(|addr| addr.ip().to_string());

    let location = if let Some(ref ip) = ip {
        get_location(ip).await.unwrap_or_default()
    } else {
        user::Location::default()
    };

    let ip = ip.unwrap_or_default();

    let user_agent = req
        .headers()
        .get("User-Agent")
        .and_then(|field| field.to_str().ok())
        .map(|s| s.to_owned());

    let device = user_agent
        .as_ref()
        .map(user::Device::from)
        .unwrap_or_default();

    let user::Location {
        country,
        state,
        city,
    } = location;

    Ok(user::User {
        device,
        ip,
        user_agent,
        time,
        country,
        state,
        city,
    })
}

async fn insert_user(user: user::User, mut conn: DbConn) {
    use diesel::RunQueryDsl;

    let _ = web::block(move || {
        diesel::insert_into(users::table)
            .values(user::NewUser::from(&user))
            .execute(&mut conn)
    })
    .await;
}

pub async fn log_user(req: HttpRequest, conn: DbConn) {
    if let Ok(user) = get_user(req).await {
        insert_user(user, conn).await
    } // Otherwise do nothing! missing a visit is fine :)
}
