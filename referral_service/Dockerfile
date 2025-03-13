# Stage 1: Build the Rust application
FROM rust:latest AS builder

WORKDIR /app
COPY . .

RUN cargo build --release
CMD ["./target/release/referral_service"]