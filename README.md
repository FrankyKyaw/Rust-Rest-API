# Rust-Rest-API
This project is a RESTful API developed using the [Rocket](https://rocket.rs) framework in Rust. It leverages [Diesel](http://diesel.rs) to interact with a database populated with a sample laptop dataset sourced from [Kaggle](https://www.kaggle.com/datasets/arnabchaki/laptop-price-prediction). The API provides full CRUD (Create, Read, Update, Delete) as well as query operations on the data.

## Built With
- [Rust](https://www.rust-lang.org)
- [Rocket](https://rocket.rs)
- [Diesel](http://diesel.rs)
- [Serde](https://serde.rs/)


## Usage
Here's an example of how to use the API:

### Get a laptop by ID
```
GET /laptop/<id>
```
Retrieve a laptop by its ID. If the laptop is found, it returns a JSON object of the laptop data, otherwise a `Status::NotFound` error is returned.

### Get the total count of laptops
```
GET /count
```

### Search for laptops
```
GET /laptops/search?<params..>
```
Search for laptops by providing search parameters, such as `brand`, `min_price` and `max_price`.
An example API request would be like this:
```
localhost:8000/laptops/search?brand=Apple&min_price=1000&max_price=2000
```
If the search is successful, it returns a JSON array of laptop objects that match the search parameters. If there's an error, it returns a `Status::InternalServerError` error.

### Create a new laptop
```
POST /laptop
```
Creates a new laptop record in the database. It requires a JSON object with `brand`, `model`, `cpu`, `gpu`, `ram_gb`, and `price`. It returns a JSON object of the newly created laptop.

### Update a laptop  
```
PATCH /update/<id>
```
Updates a laptop record in the database. It requires the ID of the laptop to update and a JSON object with `brand`, `model`, `cpu`, `gpu`, `ram_gb`, and `price`. If the update is successful, it returns a `Status::NoContent`, otherwise a `Status::InternalServerError` error.

### Delete a laptop
```
DELETE /laptop/<id>
```
Deletes a laptop record from the database using its ID. If the deletion is successful, it returns a `Status::NoContent`, otherwise a `Status::InternalServerError` error.







