pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::{NewLaptop, Laptop, RequestLaptop, SearchParams};
use bigdecimal::BigDecimal;
use std::str::FromStr;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub fn create_laptop(conn: &mut PgConnection, brand: &str, model: &str, cpu: &str, gpu: &str, ram_gb: i32, price: BigDecimal) -> Laptop {
    use crate::schema::laptops;
    let new_laptop = NewLaptop { brand, model, cpu, gpu, ram_gb, price };

    diesel::insert_into(laptops::table)
        .values(&new_laptop)
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

pub fn get_total(conn: &mut PgConnection) -> QueryResult<i64>{
    use crate::schema::laptops::dsl::*;
    laptops.count().get_result(conn)
}

pub fn search_laptops(conn: &mut PgConnection, params: &SearchParams) -> QueryResult<Vec<Laptop>>{
    use crate::schema::laptops::dsl::*;

    let mut query = laptops.into_boxed();
    if let Some(ref search_brand) = params.brand {
        query = query.filter(brand.eq(search_brand));
    }

    if let Some(ref min_price) = params.min_price {
        
        match BigDecimal::from_str(&min_price) {
            Ok(min_price) => {
                query = query.filter(price.ge(min_price));
            }
            Err(e) => {
                println!("Failed to parse min_price: {}", e);
            }
        }
    }

    if let Some(ref max_price) = params.max_price {
        match BigDecimal::from_str(max_price) {
            Ok(max_price) => {
                query = query.filter(price.le(max_price));
            }
            Err(e) => {
                println!("Failed to parse max_price: {}", e);
            }
        }
    }
    query.load::<Laptop>(conn)

}

pub fn delete_laptop(conn: &mut PgConnection, id_num: i32) -> QueryResult<usize> {
    use crate::schema::laptops::dsl::*;

    diesel::delete(laptops.filter(id.eq(id_num)))
        .execute(conn)
}

pub fn update_laptop(conn: &mut PgConnection, id_num: i32, laptop: &RequestLaptop) -> QueryResult<usize>{
    use crate::schema::laptops::dsl::*;

    diesel::update(laptops.filter(id.eq(id_num)))
        .set((
            brand.eq(&laptop.brand),
            model.eq(&laptop.model),
            cpu.eq(&laptop.cpu),
            gpu.eq(&laptop.gpu),
            ram_gb.eq(&laptop.ram_gb),
            price.eq(&laptop.price),
        ))
        .execute(conn)
}