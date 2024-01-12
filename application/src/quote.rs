use domain::models::{Quote};
use infrastructure::db::establish_connection;
use diesel::result::Error;
use diesel::query_dsl::RunQueryDsl;

pub fn list_quotes()-> Result<Vec<Quote>, Error> {
    use domain::schema::quotes;

    let connnection = establish_connection();
    
    quotes::table.load::<Quote>(&connnection)
}
