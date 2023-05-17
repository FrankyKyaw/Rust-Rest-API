use diesel::prelude::*;
use std::io::{stdin, Read};
use final_proj::{establish_connection, create_laptop};
use bigdecimal::BigDecimal;
use std::str::FromStr;

fn main() {
    let connection: &mut PgConnection = &mut establish_connection();

    let mut brand: String = String::new();
    let mut model: String = String::new();
    let mut cpu: String = String::new();
    let mut gpu: String = String::new();
    let mut ram_gb: String = String::new();
    let mut price: String = String::new();

    println!("Enter the laptop brand: ");
    stdin().read_line(&mut brand).unwrap();
    let brand: &str = brand.trim_end(); 

    println!("Enter the laptop model:");
    stdin().read_line(&mut model).unwrap();
    let model: &str = model.trim_end();

    println!("Enter the CPU:");
    stdin().read_line(&mut cpu).unwrap();
    let cpu: &str = cpu.trim_end();

    println!("Enter the GPU:");
    stdin().read_line(&mut gpu).unwrap();
    let gpu: &str = gpu.trim_end();

    println!("Enter the RAM in GB:");
    stdin().read_line(&mut ram_gb).unwrap();
    let ram_gb: i32 = ram_gb.trim_end().parse::<i32>().expect("Invalid input for RAM");

    println!("Enter the price:");
    stdin().read_line(&mut price).unwrap();
    let price = BigDecimal::from_str(price.trim_end()).expect("Invalid input for price");

    let laptop = create_laptop( connection, &brand, &model, &cpu, &gpu, ram_gb, price);
    println!("\nSaved laptop {} with id {}", brand, laptop.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";