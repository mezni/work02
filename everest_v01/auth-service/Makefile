.PHONY: help build run test lint clean docker-build docker-run migrate

help:
	@echo "Available commands:"
	@echo "  build     - Build the project"
	@echo "  run       - Run the application"
	@echo "  test      - Run tests"
	@echo "  lint      - Run linter"
	@echo "  clean     - Clean build artifacts"
	@echo "  docker-build - Build Docker image"
	@echo "  docker-run   - Run with Docker Compose"
	@echo "  migrate   - Run database migrations"

build:
	cargo build

run:
	cargo run

test:
	cargo test

lint:
	cargo fmt -- --check
	cargo clippy -- -D warnings

clean:
	cargo clean

docker-build:
	docker build -t auth-service:latest .

docker-run:
	docker-compose up --build

migrate:
	cargo run --bin migrate