FROM rust:latest

WORKDIR /usr/src/rcc
COPY ./src ./src
COPY ./tests ./tests
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
