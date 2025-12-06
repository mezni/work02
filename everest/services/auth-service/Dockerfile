# Use an official Rust image as the base
FROM rust:1.90 AS builder

# Set the working directory to /app
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Build the dependencies
RUN cargo build --release

# Copy the source code
COPY src ./src

# Build the application
RUN cargo build --release

# Use a smaller base image for the final image
FROM debian:bookworm-slim

# Set the working directory to /app
WORKDIR /app

# Copy the built binary from the builder image
COPY --from=builder /app/target/release/your_service .

# Expose the port
EXPOSE 8080

# Run the command to start the service
CMD ["./your_service"]