# -------- Build --------
FROM rust:1.89-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# -------- Runtime --------
FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/rust-port-scanner .

ENTRYPOINT ["./rust-port-scanner"]
