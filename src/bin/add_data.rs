use std::{error::Error, io, process};
use csv;
use diesel::prelude::*;
use bigdecimal::BigDecimal;
use final_proj::{establish_connection, create_laptop};
use std::str::FromStr;


fn example() -> Result<(), Box<dyn Error>> {


    let mut rdr = csv::Reader::from_reader(io::stdin());
    let connection: &mut PgConnection = &mut establish_connection();

    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        let brand = record.get(0).unwrap_or("").to_string();
        let model = record.get(1).unwrap_or("").to_string();
        let cpu = record.get(2).unwrap_or("").to_string();
        let gpu = record.get(4).unwrap_or("").to_string();
        let ram_gb_str = record.get(3).unwrap_or("").trim_end_matches("GB");
        let ram_gb = ram_gb_str.parse::<i32>()?;
        let conversion_rate: BigDecimal = BigDecimal::from_str("0.012")?;
        let price_in_rupees = BigDecimal::from_str(record.get(5).unwrap_or(""))?;
        let price_in_usd = price_in_rupees * conversion_rate.clone() * conversion_rate;

        let laptop = create_laptop( connection, &brand, &model, &cpu, &gpu, ram_gb, price_in_usd);
        println!("\nSaved laptop {} with id {}", brand, laptop.id);
    }
    Ok(())
}


fn main() {
    // run this command cat laptop_data.csv | cargo run --bin add_data to import data from the csv into the database.
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}