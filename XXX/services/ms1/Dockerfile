FROM rust:1.70-slim as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/ms1 /app/
COPY --from=builder /app/migrations /app/migrations

EXPOSE 8080
ENV RUST_LOG=info

CMD ["./ms1"]
