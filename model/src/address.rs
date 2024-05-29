use std::fmt;

use diesel::{prelude::Associations, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

pub type StreetNumber = u32;
pub type Zipcode = u32;

const VALID_STATES: [&str; 50] = [
    "AL", "AK", "AZ", "AR", "CA", "CO", "CT", "DE", "FL", "GA", "HI", "ID", "IL", "IN", "IA", "KS",
    "KY", "LA", "ME", "MD", "MA", "MI", "MN", "MS", "MO", "MT", "NE", "NV", "NH", "NJ", "NM", "NY",
    "NC", "ND", "OH", "OK", "OR", "PA", "RI", "SC", "SD", "TN", "TX", "UT", "VT", "VA", "WA", "WV",
    "WI", "WY",
];

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Address {
    pub name: String,
    pub order: u32,
    pub number: StreetNumber,
    pub street: String,
    pub city: String,
    pub state: String,
    pub zipcode: Zipcode,
}

impl Address {
    pub fn validate(&self) -> bool {
        VALID_STATES.contains(&self.state.as_str())
            && self.zipcode >= 10_000
            && self.zipcode <= 99_999
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Address {
            number,
            street,
            city,
            state,
            zipcode,
            ..
        } = self;
        write!(f, "{number} {street} {city}, {state}, US {zipcode}")
    }
}

impl From<TableAddress> for Address {
    fn from(
        TableAddress {
            name,
            order_id,
            number,
            street,
            city,
            state,
            zipcode,
        }: TableAddress,
    ) -> Address {
        Address {
            name,
            order: order_id as u32,
            number: number as StreetNumber,
            street,
            city,
            state,
            zipcode: zipcode as Zipcode,
        }
    }
}

#[derive(Queryable, Selectable, Associations, Clone, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::addresses)]
#[diesel(belongs_to(crate::order::TableOrder, foreign_key = order_id))]
pub struct TableAddress {
    name: String,
    order_id: i32,
    number: i32,
    street: String,
    city: String,
    state: String,
    zipcode: i32,
}

impl<'a, 'b: 'a> From<&'b Address> for NewAddress<'a> {
    fn from(
        Address {
            name,
            order,
            number,
            street,
            city,
            state,
            zipcode,
        }: &'b Address,
    ) -> NewAddress<'a> {
        NewAddress {
            name,
            order_id: *order as i32,
            number: *number as i32,
            street,
            city,
            state,
            zipcode: *zipcode as i32,
        }
    }
}

#[derive(Insertable, Clone, Copy, Debug)]
#[diesel(table_name = crate::schema::addresses)]
pub struct NewAddress<'a> {
    name: &'a str,
    order_id: i32,
    number: i32,
    street: &'a str,
    city: &'a str,
    state: &'a str,
    zipcode: i32,
}
