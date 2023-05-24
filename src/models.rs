use diesel::prelude::*;
use bigdecimal::BigDecimal;
use rocket::FromForm;
use serde::{Deserialize, Serialize};
use crate::schema::laptops;


#[derive(Insertable)]
#[table_name = "laptops"]
pub struct NewLaptop<'a> {
    pub brand: &'a str,
    pub model: &'a str,
    pub cpu: &'a str,
    pub gpu: &'a str,
    pub ram_gb: i32,
    pub price: BigDecimal,

}

#[derive(Debug, Queryable, Deserialize, Serialize)]
pub struct Laptop {
    pub id: i32,
    pub brand: String,
    pub model: String,
    pub cpu: String,
    pub gpu: String,
    pub ram_gb: i32,
    pub price: BigDecimal,
}


#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct RequestLaptop {
    pub brand: String,
    pub model: String,
    pub cpu: String,
    pub gpu: String,
    pub ram_gb: i32,
    pub price: BigDecimal,
}

#[derive(FromForm)]
pub struct SearchParams {
    pub brand: Option<String>,
    pub min_price: Option<String>,
    pub max_price: Option<String>
}