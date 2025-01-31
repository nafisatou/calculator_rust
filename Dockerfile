# Use the official Rust image to build the app
FROM rust:1.70 as builder

# Set the working directory
WORKDIR /usr/src/calculator

# Copy the Cargo.toml and source code
COPY Cargo.toml Cargo.lock ./
COPY . .

# Build the application
RUN cargo build --release

# Use a smaller base image for running the app
FROM debian:bullseye-slim

# Install necessary runtime dependencies (if needed)
RUN apt-get update && apt-get install -y libc6 && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder image
COPY --from=builder /usr/src/calculator/target/debug/calculator /usr/local/bin/

# Set the default command
ENTRYPOINT ["/usr/local/bin/calculator"]
