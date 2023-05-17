-- Your SQL goes here
CREATE TABLE laptops (
    id SERIAL PRIMARY KEY,
    brand TEXT NOT NULL,
    model TEXT NOT NULL,
    cpu TEXT NOT NULL,
    gpu TEXT NOT NULL,
    ram_gb INTEGER NOT NULL,
    price NUMERIC(10, 2) NOT NULL
);