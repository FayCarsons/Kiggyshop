use std::fmt;

use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

const VALID_STATES: [&str; 50] = [
    "AL", "AK", "AZ", "AR", "CA", "CO", "CT", "DE", "FL", "GA", "HI", "ID", "IL", "IN", "IA", "KS",
    "KY", "LA", "ME", "MD", "MA", "MI", "MN", "MS", "MO", "MT", "NE", "NV", "NH", "NJ", "NM", "NY",
    "NC", "ND", "OH", "OK", "OR", "PA", "RI", "SC", "SD", "TN", "TX", "UT", "VT", "VA", "WA", "WV",
    "WI", "WY",
];

#[derive(Queryable, Selectable, Clone, Serialize, Deserialize, Hash, Debug)]
#[diesel(table_name = crate::schema::addresses)]
pub struct TableAddress {
    order: i32,
    number: i32,
    street: String,
    city: String,
    state: String,
    zipcode: i32,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Address {
    pub order: u32,
    pub number: StreetNumber,
    pub street: String,
    pub city: String,
    pub state: String,
    pub zipcode: Zipcode,
}

impl From<TableAddress> for Address {
    fn from(
        TableAddress {
            order,
            number,
            street,
            city,
            state,
            zipcode,
        }: TableAddress,
    ) -> Address {
        Address {
            order: order as u32,
            number: number as StreetNumber,
            street,
            city,
            state,
            zipcode: zipcode as Zipcode,
        }
    }
}

impl<'a, 'b: 'a> From<&'b Address> for NewAddress<'a> {
    fn from(
        Address {
            order,
            number,
            street,
            city,
            state,
            zipcode,
        }: &'b Address,
    ) -> NewAddress<'a> {
        NewAddress {
            order: *order as i32,
            number: *number as i32,
            street,
            city,
            state,
            zipcode: *zipcode as i32,
        }
    }
}

pub type StreetNumber = u32;
pub type Zipcode = u32;

#[derive(Insertable, Clone, Copy, Debug)]
#[diesel(table_name = crate::schema::addresses)]
pub struct NewAddress<'a> {
    pub order: i32,
    pub number: i32,
    pub street: &'a str,
    pub city: &'a str,
    pub state: &'a str,
    pub zipcode: i32,
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
