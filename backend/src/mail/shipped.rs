use model::order;

#[derive(Debug, Clone, askama::Template)]
#[template(path = "shipped.html")]
pub struct Shipped {
    id: u32,
    name: String,
    pub email: String,
    tracking: String,
}

impl TryFrom<order::Order> for Shipped {
    type Error = String;

    fn try_from(
        order::Order {
            id,
            name,
            email,
            tracking_number,
            ..
        }: order::Order,
    ) -> Result<Self, Self::Error> {
        if let Some(tracking) = tracking_number {
            Ok(Self {
                id,
                name,
                email,
                tracking,
            })
        } else {
            Err(format!("No tracking # on order {id}"))
        }
    }
}

impl TryFrom<order::TableOrder> for Shipped {
    type Error = String;

    fn try_from(
        order::TableOrder {
            id,
            name,
            email,
            tracking_number,
            ..
        }: order::TableOrder,
    ) -> Result<Self, Self::Error> {
        if let Some(tracking) = tracking_number {
            Ok(Self {
                id: id as u32,
                name,
                email,
                tracking,
            })
        } else {
            Err(format!("Order {id} does not contain a tracking number!"))
        }
    }
}

impl Shipped {
    pub fn new<T, U>(id: T, email: U, name: U, tracking: U) -> Self
    where
        T: Into<u32>,
        U: std::fmt::Display,
    {
        let id = id.into();
        let name = name.to_string();
        let email = email.to_string();
        let tracking = tracking.to_string();
        Self {
            id,
            name,
            email,
            tracking,
        }
    }

    pub fn render_plaintext(&self) -> String {
        format!(
            r#"Hi, {}! Your order has shipped!\nOrder #: {}\nUSPS tracking #: {}"#,
            self.name, self.id, self.tracking
        )
    }
}
