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
use final_proj::models::{NewLaptop, Laptop};



#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct RequestLaptop {
    pub brand: String,
    pub model: String,
    pub cpu: String,
    pub gpu: String,
    pub ram_gb: i32,
    pub price: BigDecimal,
}

#[get("/")]
fn hello() -> &'static str {
    "Hello world"
} 


#[get("/")]
fn test() -> Redirect {
    Redirect::to(uri!(hello()))
}

#[post("/laptop", data="<laptop>")]
fn create(laptop: Json<RequestLaptop>) -> Json<Laptop>{
    let connection = &mut establish_connection();
    let new_laptop = create_laptop(connection, &laptop.0.brand, &laptop.0.model, &laptop.0.cpu, &laptop.0.gpu, laptop.0.ram_gb, laptop.0.price);
    Json(new_laptop)
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/test", routes![test])
        .mount("/", routes![create])
}

// fn main() {
//     let connection = &mut establish_connection();
//     let brand:&str = "MSI";
//     let model: &str = "Katana";
//     let cpu: &str = "11350h";
//     let gpu: &str = "3060rtx";
//     let ram_gb: i32 = 16;
//     let price: BigDecimal = BigDecimal::from_str("800.0").unwrap();

//     let new_laptop = create_laptop(connection, brand, model, cpu, gpu, ram_gb, price);
//     println!("Done")
// }