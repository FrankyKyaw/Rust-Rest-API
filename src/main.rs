#[macro_use] extern crate rocket;

#[macro_use] 
extern crate diesel;


use diesel::connection;
use rocket::http::Status;
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use serde_json::to_string;
use rocket::response::Redirect;
use bigdecimal::BigDecimal;
use final_proj::{create_laptop, get_laptop, delete_laptop, establish_connection, update_laptop};
use std::str::FromStr;
use final_proj::models::{NewLaptop, Laptop, RequestLaptop};


// #[derive(Debug, Queryable, Serialize, Deserialize)]
// pub struct RequestLaptop {
//     pub brand: String,
//     pub model: String,
//     pub cpu: String,
//     pub gpu: String,
//     pub ram_gb: i32,
//     pub price: BigDecimal,
// }

#[get("/")]
fn hello() -> &'static str {
    "Hello world"
} 


#[get("/")]
fn test() -> Redirect {
    Redirect::to(uri!(hello()))
}

#[get("/laptop/<id>")]
fn get_laptop_by_id(id: i32) -> Result<Json<Laptop>, Status> {
    let connection = &mut establish_connection();
    match get_laptop(connection, id) {
        Some(laptop) => Ok(Json(laptop)),
        None => Err(Status::NotFound)
    }
}

#[post("/laptop", data="<laptop>")]
fn create(laptop: Json<RequestLaptop>) -> Json<Laptop>{
    let connection: &mut diesel::PgConnection = &mut establish_connection();
    let new_laptop: Laptop = create_laptop(connection, &laptop.0.brand, &laptop.0.model, &laptop.0.cpu, &laptop.0.gpu, laptop.0.ram_gb, laptop.0.price);
    Json(new_laptop)
}

#[patch("/update/<id>", data="<laptop>")]
fn update(id: i32, laptop: Json<RequestLaptop>) -> Result<Status, Status> {
    let connection = &mut establish_connection();
    match update_laptop(connection, id, &laptop.0) {
        Ok(_) => Ok(Status::NoContent),
        Err(_) => Err(Status::InternalServerError),
    }
}


#[delete("/laptop/<id>")]
fn delete_laptop_by_id(id: i32) -> Result<Status, Status> {
    let connection: &mut diesel::PgConnection = &mut establish_connection();
    match delete_laptop(connection, id) {
        Ok(_) => Ok(Status::NoContent),  
        Err(_) => Err(Status::InternalServerError), 
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/test", routes![test])
        .mount("/", routes![create])
        .mount("/", routes![get_laptop_by_id])
        .mount("/", routes![update])
        .mount("/", routes![delete_laptop_by_id])
}
