#[macro_use] extern crate rocket;

mod quote_handler;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![quote_handler::list_quotes_handler])
}
