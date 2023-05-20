pub mod models;
pub mod schema;
pub mod api;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::{NewLaptop, Laptop};
use bigdecimal::BigDecimal;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub fn create_laptop(conn: &mut PgConnection, brand: &str, model: &str, cpu: &str, gpu: &str, ram_gb: i32, price: BigDecimal) -> Laptop {
    use crate::schema::laptops;
    let new_post = NewLaptop { brand, model, cpu, gpu, ram_gb, price };

    diesel::insert_into(laptops::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new laptop")
}


pub fn get_laptop(conn: &mut PgConnection, id_num: i32) -> Option<Laptop> {
    use crate::schema::laptops::dsl::*;

    laptops.filter(id.eq(id_num))
        .first(conn)
        .optional()
        .expect("Error loading laptop")
}

pub fn delete_laptop(conn: &mut PgConnection, id: i32) -> QueryResult<usize> {
    use crate::schema::laptops::dsl::*;

    diesel::delete(laptops.filter(id.eq(id)))
        .execute(conn)
}

