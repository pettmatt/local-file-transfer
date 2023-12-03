FROM rust:1.73.0

WORKDIR /usr/src/file-transfer-server
COPY Cargo.toml Cargo.lock ./
RUN mkdir src/

COPY src/ ./src/
# build the application
RUN cargo build --release

EXPOSE 7878
CMD ["./target/release/file-transfer-server"]