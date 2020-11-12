# Build stage
FROM rust:latest as build

# Build only deps
RUN cargo install cargo-build-dependencies
WORKDIR /tmp
RUN USER=root cargo new --bin code-executor
WORKDIR /tmp/code-executor 
COPY Cargo.toml Cargo.lock ./
RUN cargo build-dependencies --release

# Copy source tree and build
COPY src /tmp/code-executor/src
RUN cargo build --release 

FROM python:3.8 

COPY --from=build /tmp/code-executor/target/release/code-executor code-executor
COPY .env ./

# Set the entrypoint
ENTRYPOINT [ "./code-executor" ]
