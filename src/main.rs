#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world")]              
fn world() -> &'static str {
    "hello, estou testando"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index, world])
}
