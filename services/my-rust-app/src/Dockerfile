# --- Stage 1: Build Stage ---
FROM rust:1.75-slim as builder

# Create a workspace
WORKDIR /usr/src/app

# Copy the source code and configuration
COPY . .

# Build for release
RUN cargo build --release

# --- Stage 2: Runtime Stage ---
FROM debian:bookworm-slim

# Install SSL certificates if your app makes HTTPS requests
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Copy the binary from the builder stage
# Replace 'my-rust-app' with the name defined in your Cargo.toml
COPY --from=builder /usr/src/app/target/release/my-rust-app .

# Run the application
CMD ["./my-rust-app"]