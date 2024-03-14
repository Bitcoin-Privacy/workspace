FROM rust:1.61-buster

WORKDIR /app

COPY Cargo.lock Cargo.toml ./

RUN cargo fetch

RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release

RUN rm -rf src

COPY . .
RUN cargo build --release

