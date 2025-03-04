# Stage 1: Build the Rust application
FROM rust:latest AS builder
WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./ 
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copy the rest of the source code and build the final binary
COPY . . 
RUN cargo build --release

# Stage 2: Create the final runtime image with PostgreSQL and Rust application
FROM debian:bullseye-slim AS runtime
WORKDIR /app

# Install necessary dependencies including PostgreSQL
RUN apt update && apt install -y wget build-essential postgresql && \
    wget http://ftp.gnu.org/gnu/libc/glibc-2.34.tar.gz && \
    tar -xvzf glibc-2.34.tar.gz && \
    cd glibc-2.34 && \
    mkdir build && cd build && \
    ../configure --prefix=/opt/glibc-2.34 && \
    make -j$(nproc) && \
    make install && \
    cd /app && rm -rf /opt/glibc-2.34.tar.gz /opt/glibc-2.34

# Set environment variables to use GLIBC 2.34
ENV LD_LIBRARY_PATH=/opt/glibc-2.34/lib:$LD_LIBRARY_PATH
ENV PATH="/opt/glibc-2.34/bin:$PATH"

# Install PostgreSQL and setup the environment
RUN apt-get install -y postgresql postgresql-contrib

# Set up PostgreSQL database
RUN service postgresql start && \
    sudo -u postgres psql -c "CREATE USER admin WITH PASSWORD 'admin';" && \
    sudo -u postgres psql -c "CREATE DATABASE euron;" && \
    sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE euron TO admin;"

# Copy the Rust binary from the builder stage
COPY --from=builder /app/target/release/euron_referral_api .

# Ensure the binary is executable
RUN chmod +x /app/euron_referral_api

# Expose the necessary ports
EXPOSE 8080
EXPOSE 5432

# Start PostgreSQL and the Rust app in the same container
CMD service postgresql start && ./euron_referral_api
