#[macro_use] extern crate rocket;

extern crate diesel;

use rocket::http::Status;
use rocket::serde::json::Json;
use final_proj::{create_laptop, get_laptop, delete_laptop, establish_connection, update_laptop, get_total, search_laptops};
use final_proj::models::{Laptop, RequestLaptop, SearchParams};

	

/*
Run this command to get the server started
cargo run --bin final_proj
 */


#[get("/")]
fn hello() -> &'static str {
    "Hello world"
} 



#[get("/laptop/<id>")]
fn get_laptop_by_id(id: i32) -> Result<Json<Laptop>, Status> {
    let connection = &mut establish_connection();
    match get_laptop(connection, id) {
        Some(laptop) => Ok(Json(laptop)),
        None => Err(Status::NotFound)
    }
}

#[get("/count")]
fn get_count() -> Result<Json<usize>, Status>{
    let connection = &mut establish_connection();
    let result = get_total(connection);
    match result {
        Ok(count) => Ok(Json(count as usize)),
        Err(_) => Err(Status::InternalServerError)   
    }
}

#[get("/laptops/search?<params..>")]
fn search(params: SearchParams) -> Result<Json<Vec<Laptop>>, Status>{
    let connection = &mut establish_connection();
    println!("Brand: {:?}", params.brand);
    match search_laptops(connection, &params) {
        Ok(laptops) => Ok(Json(laptops)),
        Err(_) => Err(Status::InternalServerError)
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
        .mount("/", routes![create])
        .mount("/", routes![get_laptop_by_id])
        .mount("/", routes![update])
        .mount("/", routes![delete_laptop_by_id])
        .mount("/", routes![get_count])
        .mount("/", routes![search])
}



#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn hello_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let  response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some("Hello world".into()));
    }

    #[test]
    fn test_count() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/count").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
    #[test]
    fn test_search() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/laptops/search?brand=Apple&min_price=1000&max_price=2000").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}