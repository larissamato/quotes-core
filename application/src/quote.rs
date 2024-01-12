use domain::models::{Quote, NewQuote};
use infrastructure::lib::establish_connection;
use diesel::prelude::*;
use diesel::result::Error;

pub fn list_quotes()-> Result<Vec<Quote>, Error> {
    use domain::schema::quotes;

    let connnection = establish_connection();
    
    quotes::table.load::<Quote>(&connection)
}
