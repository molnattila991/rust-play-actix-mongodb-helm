FROM rust:1.62 as builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev pkg-config libssl-dev
RUN update-ca-certificates

WORKDIR /app
COPY ./src/dummy.rs .
COPY ./Cargo.toml .

RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

COPY . .

RUN cargo build --target x86_64-unknown-linux-musl --release

# FROM debian
FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/app /app
CMD ["/app"]