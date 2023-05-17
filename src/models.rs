use diesel::prelude::*;
use bigdecimal::BigDecimal;
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

#[derive(Debug, Queryable)]
pub struct Laptop {
    pub id: i32,
    pub brand: String,
    pub model: String,
    pub cpu: String,
    pub gpu: String,
    pub ram_gb: i32,
    pub price: BigDecimal,
}