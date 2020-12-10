FROM rust as plan 
WORKDIR /app 
RUN cargo install cargo-chef
COPY . . 
RUN cargo chef prepare --recipe-path recipe.json

FROM rust as cache 
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=plan /app/recipe.json recipe.json
RUN cargo chef cook --release --features lambda --recipe-path recipe.json

FROM rust as build 
WORKDIR /app
COPY . . 
COPY --from=cache /app/target target
COPY --from=cache $CARGO_HOME $CARGO_HOME
RUN cargo build --release --features lambda --bin lambda

FROM amazon/aws-lambda-provided:al2
RUN yum install -y amazon-linux-extras && \
    amazon-linux-extras enable python3.8 && \
    yum clean metadata && \
    yum install -y python3.8
COPY --from=build /app/target/release/lambda ${LAMBDA_RUNTIME_DIR}/bootstrap
# No handler, bootstrap includes everything
CMD [ "" ] 
