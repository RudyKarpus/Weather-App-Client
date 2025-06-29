FROM rust:latest AS chef
RUN cargo install trunk --locked
RUN cargo install cargo-chef --locked

FROM chef AS planner
WORKDIR /usr/src/app
COPY . .
RUN rm -f rust-toolchain.toml
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /usr/src/app
COPY --from=planner /usr/src/app/recipe.json /usr/src/app/recipe.json
COPY rust-toolchain.toml rust-toolchain.toml
RUN cargo chef cook --recipe-path recipe.json --target wasm32-unknown-unknown
COPY src src
COPY .env .env
COPY build.rs build.rs
COPY Cargo.toml Cargo.toml
COPY index.html index.html
COPY public public
RUN trunk build
CMD rm .env
RUN cargo install simple-http-server

COPY entrypoint.sh /entrypoint.sh

RUN chmod +x /entrypoint.sh

RUN sed -i 's/\r$//' /entrypoint.sh

EXPOSE 8080

ENTRYPOINT ["/entrypoint.sh"]