use final_proj::models::*;
use diesel::prelude::*;
use final_proj::establish_connection;
// use diesel_demo::*;

fn main() {
    use final_proj::schema::laptops::dsl::*;

    let connection: &mut PgConnection = &mut establish_connection();
    let results: Vec<Laptop> = laptops
        .limit(5)
        .load::<Laptop>(connection)
        .expect("Error loading data");

    println!("Displaying {} laptops", results.len());
    for laptop in results {
        println!("Brand: {}, Model: {}, CPU: {}, GPU: {}, RAM: {}, Price: {}", laptop.brand, laptop.model, laptop.cpu, laptop.gpu, laptop.ram_gb, laptop.price);
    }
}