use crate::application::quote::list_quotes;
use rocket::get;
use serde_json::json;
use rocket::response::status;


#[get("/quotes")]
pub fn list_quotes_handler()-> Result<String> {
    match list_quotes() {
        Ok(quotes) => {
            let response = json!({"quotes": quotes}); 
            Ok(response.to_string())
        },
        Err(_) => {
            let response = status::NotFound("Sorry, I couldn't find it!");
            Err(NotFound(response.to_string()))
        }
    }
}
