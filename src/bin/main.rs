use std::env;
#[macro_use]
extern crate prettytable;
use prettytable::Table;

fn main() {
    let mut args = env::args();
    let script_name = match args.next() {
        Some(arg) => arg,
        None => String::from(""),
    };
    let query = match args.next() {
        Some(arg) => arg,
        None => String::from(""),
    };
    let query = &query.to_lowercase();

    eprintln!("Usage: {} [query]", script_name);
    let mut found = false;
    let mut table = Table::new();
    table.add_row(row!["Name", "Alpha2", "Alpha3", "Numeric"]);

    for country in rust_iso3166::ALL {
        if country.alpha2.to_lowercase().contains(query)
            || country.alpha3.to_lowercase().contains(query)
            || country.numeric_str().to_lowercase().contains(query)
        {
            table.add_row(row![
                country.name,
                country.alpha2,
                country.alpha3,
                country.numeric_str()
            ]);
            found = true;
        }
    }

    if found == false {
        for country in rust_iso3166::ALL {
            if country.name.to_lowercase().contains(query) {
                table.add_row(row![
                    country.name,
                    country.alpha2,
                    country.alpha3,
                    country.numeric_str()
                ]);
            }
        }
    }
    table.printstd();
}
