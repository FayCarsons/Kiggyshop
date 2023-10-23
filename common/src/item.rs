use std::io::Error;

use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum ItemKind {
    BigPrint,
    SmallPrint,
    Button,
}

impl Default for ItemKind {
    fn default() -> Self {
        ItemKind::BigPrint
    }
}

impl From<ItemKind> for f32 {
    fn from(value: ItemKind) -> Self {
        match value {
            ItemKind::BigPrint => 20.00,
            ItemKind::SmallPrint => 7.00,
            ItemKind::Button => 3.00,
        }
    }
}

impl From<&ItemKind> for f32 {
    fn from(value: &ItemKind) -> Self {
        match value {
            ItemKind::BigPrint => 20.00,
            ItemKind::SmallPrint => 7.00,
            ItemKind::Button => 3.00,
        }
    }
}

impl From<&str> for ItemKind {
    fn from(value: &str) -> Self {
        match value.trim() {
            "small print" | "SmallPrint" | "Small Print" => Self::SmallPrint,
            "button" | "Button" => Self::Button,
            _ => Self::BigPrint,
        }
    }
}

impl From<String> for ItemKind {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<ItemKind> for String {
    fn from(value: ItemKind) -> Self {
        match value {
            ItemKind::BigPrint => "Large Print".to_owned(),
            ItemKind::Button => "Button".to_owned(),
            ItemKind::SmallPrint => "Small Print".to_owned(),
        }
    }
}

impl From<&ItemKind> for String {
    fn from(value: &ItemKind) -> Self {
        match value {
            ItemKind::BigPrint => "Large Print".to_owned(),
            ItemKind::Button => "Button".to_owned(),
            ItemKind::SmallPrint => "Small Print".to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Properties)]
pub struct Item {
    pub title: String,
    pub kind: ItemKind,
    pub description: String,
    pub stock: u32,
}

impl Item {
    pub fn new(title: String, kind: String, description: String, stock: u32) -> Self {
        Self {
            title,
            kind: ItemKind::from(kind),
            description,
            stock,
        }
    }

    pub fn from_admin(from_admin: AdminItem) -> Result<Self, Error> {
        let AdminItem {
            image,
            title,
            kind,
            description,
            stock,
            ..
        } = from_admin;

        let img_path = format!("resources/images/{title}");
        image::save_buffer(&img_path, &image, 1920, 1080, image::ColorType::Rgba8)
            .map_err(|e| Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        Ok(Self {
            title,
            kind,
            description,
            stock,
        })
    }

    pub fn dec(&self) -> Self {
        Self {
            stock: self.stock.checked_sub(1).unwrap_or(0),
            ..self.clone()
        }
    }
}

/// So Kristen can add new prints etc to stock
#[derive(Clone, Debug, Deserialize)]
pub struct AdminItem {
    title: String,
    kind: ItemKind,
    image: Vec<u8>,
    description: String,
    stock: u32,
}
