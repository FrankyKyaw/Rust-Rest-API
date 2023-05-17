use final_proj::models::*;
use diesel::prelude::*;
use final_proj::establish_connection;
// use diesel_demo::*;

fn main() {
    use final_proj::schema::laptops::dsl::*;

    let connection = &mut establish_connection();
    let results = laptops
        .limit(5)
        .load::<Laptop>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for laptop in results {
        println!("{}", laptop.brand);
        println!("-----------\n");
        println!("{}", laptop.model);
    }
}