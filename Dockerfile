# Use the official Rust image to build the app
FROM rust:latest as builder


# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and source code
COPY . .

# Build the application
RUN cargo build --release

# Use a smaller base image for running the app
FROM debian:latest

# Install necessary runtime dependencies (if needed)
RUN apt-get update && apt-get install -y libc6 && rm -rf /var/lib/apt/lists/*


# Copy the compiled binary from the builder image
<<<<<<< HEAD
COPY --from=builder /app/target/release/calculator /app/calculator

=======
COPY --from=builder /usr/src/calculator/target/debug/calculator /usr/local/bin/
>>>>>>> 2b5e317ff40becd973c024cb82ec871a49dba05b

# Set the default command to do it 
CMD ["/app/calculator"]

