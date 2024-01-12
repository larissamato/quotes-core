use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Quote {
    pub id: i32,
    pub uuid: String,
    pub quote: String,
    pub author: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "quotes"]
pub struct NewQuote {
    pub uuid: String,
    pub quote: String,
    pub author: String,
}

