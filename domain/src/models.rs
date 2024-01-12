use crate::schema::quotes;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Quote {
    pub id: i32,
    pub uuid: String,
    pub quote: String,
    pub author: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = quotes)]
pub struct NewQuote {
    pub uuid: String,
    pub quote: String,
    pub author: String,
}

