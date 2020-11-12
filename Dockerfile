# Build stage
FROM rust:latest as build

# Build only deps
RUN cargo install cargo-build-dependencies
WORKDIR /tmp
RUN USER=root cargo new --bin code-executer
WORKDIR /tmp/code-executer 
COPY Cargo.toml Cargo.lock ./
RUN cargo build-dependencies --release

# Copy source tree and build
COPY src /tmp/code-executer/src
RUN cargo build --release 

FROM python:3.8 

COPY --from=build /tmp/code-executer/target/release/code-executer code-executer
COPY .env ./

# Set the entrypoint
ENTRYPOINT [ "./code-executer" ]
