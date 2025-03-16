
FROM rust:1.85 AS builder

WORKDIR /app
COPY . .

RUN cargo build --release
CMD ["./target/release/referral_service"]