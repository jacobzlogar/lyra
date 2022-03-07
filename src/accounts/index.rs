pub mod accounts;

extern crate lyra;
extern crate diesel;

use self::lyra::*;
use self::models::Account;
use self::diesel::prelude::*;

fn main() {
    use lyra::schema::accounts::dsl::*;

    let connection = establish_connection();
    let results = accounts
        .limit(5)
        .load::<Account>(&connection)
        .expect("Error loading accounts");

    println!("Displaying {} accounts", results.len());
    for account in results {
        println!("{}", account.id);
        println!("----------\n");
        println!("{:?}", account.name);
    }
}
