FROM rust:latest as build

WORKDIR /usr/src/myapp
RUN USER=root cargo new --bin final_proj

# Copy my source code into the Docker container
COPY ./Cargo.toml ./Cargo.lock ./final_proj/
WORKDIR /usr/src/myapp/final_proj
COPY . .

# Install diesel_cli
RUN cargo install diesel_cli --no-default-features --features postgres

# Build my application
RUN cargo build --release 

# Start a new stage
FROM debian:buster-slim


# Copy the build artifacts from the build stage to the final stage
COPY --from=build /usr/src/myapp/final_proj/target/release/final_proj .