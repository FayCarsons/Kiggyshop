use chrono::NaiveDateTime;
use diesel::{Insertable, Selectable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum Device {
    Linux,
    Mac,
    Windows,
    Iphone,
    Android,
    #[default]
    Unknown,
}

impl<T> From<T> for Device
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        use Device::*;
        let value = value.as_ref().to_lowercase();
        if value.contains("linux") {
            if value.contains("android") {
                Android
            } else {
                Linux
            }
        } else if value.contains("macintosh") || value.contains("mac os x") {
            Mac
        } else if value.contains("windows") {
            Windows
        } else if value.contains("iphone") {
            Iphone
        } else {
            Unknown
        }
    }
}

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        use Device::*;
        write!(
            f,
            "{}",
            match self {
                Linux => "Linux",
                Mac => "Mac",
                Windows => "Windows",
                Iphone => "Iphone",
                Android => "Android",
                Unknown => "Unknown Device",
            }
        )
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Location {
    pub country: String,
    pub state: String,
    pub city: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub device: Device,
    pub ip: String,
    pub user_agent: Option<String>,
    pub time: NaiveDateTime,
    pub country: String,
    pub state: String,
    pub city: Option<String>,
}

impl From<TableUser> for User {
    fn from(
        TableUser {
            ip,
            user_agent,
            device,
            time,
            country,
            state,
            city,
            ..
        }: TableUser,
    ) -> Self {
        Self {
            device: Device::from(device),
            ip,
            user_agent,
            time,
            country,
            state,
            city,
        }
    }
}

#[derive(Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct TableUser {
    ip: String,
    user_agent: Option<String>,
    device: String,
    time: NaiveDateTime,
    country: String,
    state: String,
    city: Option<String>,
}

#[derive(Insertable, Clone)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    ip: &'a str,
    user_agent: Option<Cow<'a, str>>,
    device: Cow<'a, str>,
    time: &'a NaiveDateTime,
    country: &'a str,
    state: &'a str,
    city: Option<Cow<'a, str>>,
}

impl<'a, 'b: 'a> From<&'b User> for NewUser<'a> {
    fn from(
        User {
            device,
            ip,
            user_agent,
            time,
            country,
            state,
            city,
            ..
        }: &'b User,
    ) -> Self {
        Self {
            device: Cow::Owned(device.to_string()),
            ip,
            user_agent: user_agent.as_deref().map(Cow::Borrowed),
            time,
            country,
            state,
            city: city.as_deref().map(Cow::Borrowed),
        }
    }
}
