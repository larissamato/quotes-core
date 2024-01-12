use crate::application::quote::list_quotes;
use rocket::get;
use rocket::response::status::NotFound;
use serde_json::json;

#[get("/quotes")]
pub fn list_quotes_handler()-> Result<String, NotFound<String>> {
    match list_quotes() {
        Ok(quotes) => {
            let response = json!({"quotes": quotes}); 
            Ok(response.to_string())
        },
        Err(_) => {
            let response = json!({"error": "Failed to retrieve quotes"})
            Err(NotFound(response.to_string()))
        }
    }
}
