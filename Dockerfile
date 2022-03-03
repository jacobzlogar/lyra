# Rust as the base image
FROM rust:latest as build

# Create a new empty shell project
RUN USER=root cargo new --bin lyra
WORKDIR /lyra

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build for release.
RUN rm ./target/release/deps/lyra*
RUN cargo build --release

# The final base image
FROM debian:buster-slim

# Copy from the previous build
COPY --from=build /lyra/target/release/lyra /usr/src/lyra

# Run the binary
CMD ["/usr/src/lyra"]
