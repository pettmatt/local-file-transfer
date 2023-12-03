FROM rust:latest as builder

WORKDIR /usr/src/server
COPY Cargo.toml Cargo.lock ./

# Fetch dependencies in dummy project
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY . .
RUN cargo build --release

FROM alpine:latest
WORKDIR /usr/src/server

COPY --from=builder /usr/src/server/target/release/file-transfer-server .
EXPOSE 7878

CMD ["./file-transfer-server"]
