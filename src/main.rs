#[macro_use] extern crate rocket;

#[macro_use] 
extern crate diesel;


use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use serde_json::to_string;
use rocket::response::Redirect;
use bigdecimal::BigDecimal;
use final_proj::{create_laptop, establish_connection};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
struct Object {
    book: String,
    year_published: u32
}


#[get("/")]
fn hello() -> Json<String> {
    let obj = Object { book: "Rust Programming".to_string(), year_published: 2022 };
    let json_string = to_string(&obj).unwrap();
    Json(json_string)
}


#[get("/")]
fn test() -> Redirect {
    Redirect::to(uri!(hello()))
}


// #[launch]
// fn rocket() -> _ {
//     rocket::build()
//         .mount("/", routes![hello])
//         .mount("/test", routes![test])
// }

fn main() {
    let connection = &mut establish_connection();
    let brand:&str = "MSI";
    let model: &str = "Katana";
    let cpu: &str = "11350h";
    let gpu: &str = "3060rtx";
    let ram_gb: i32 = 16;
    let price: BigDecimal = BigDecimal::from_str("800.0").unwrap();

    let new_laptop = create_laptop(connection, brand, model, cpu, gpu, ram_gb, price);
    println!("Done")
}