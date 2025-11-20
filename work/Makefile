# Variables
CARGO = cargo
DOCKER = docker
RUSTFLAGS = -D warnings

# Services
SERVICES = auth-service

# Docker
DOCKER_REPO ?= your-registry

# Default target
.PHONY: all
all: format clippy test build

# Code Quality
.PHONY: format
format:
	@for service in $(SERVICES); do \
		echo "Formatting $$service..."; \
		$(CARGO) fmt --manifest-path services/$$service/Cargo.toml -- --check || exit 1; \
	done

.PHONY: format-fix
format-fix:
	@for service in $(SERVICES); do \
		echo "Fixing formatting for $$service..."; \
		$(CARGO) fmt --manifest-path services/$$service/Cargo.toml; \
	done

.PHONY: clippy
clippy:
	@for service in $(SERVICES); do \
		echo "Running clippy on $$service..."; \
		$(CARGO) clippy --manifest-path services/$$service/Cargo.toml -- -D warnings || exit 1; \
	done

.PHONY: clippy-fix
clippy-fix:
	@for service in $(SERVICES); do \
		echo "Fixing clippy suggestions for $$service..."; \
		$(CARGO) clippy --manifest-path services/$$service/Cargo.toml --fix -Z unstable-options -- -D warnings || exit 1; \
	done

# Building
.PHONY: build
build:
	@for service in $(SERVICES); do \
		echo "Building $$service..."; \
		$(CARGO) build --release --manifest-path services/$$service/Cargo.toml || exit 1; \
	done

.PHONY: build-dev
build-dev:
	@for service in $(SERVICES); do \
		echo "Building $$service (dev)..."; \
		$(CARGO) build --manifest-path services/$$service/Cargo.toml || exit 1; \
	done

# Testing
.PHONY: test
test:
	@for service in $(SERVICES); do \
		echo "Testing $$service..."; \
		$(CARGO) test --manifest-path services/$$service/Cargo.toml || exit 1; \
	done

.PHONY: test-verbose
test-verbose:
	@for service in $(SERVICES); do \
		echo "Testing $$service (verbose)..."; \
		$(CARGO) test --manifest-path services/$$service/Cargo.toml -- --nocapture || exit 1; \
	done

# Test Coverage (requires cargo-tarpaulin)
.PHONY: test-coverage
test-coverage:
	@for service in $(SERVICES); do \
		echo "Generating coverage for $$service..."; \
		cargo tarpaulin --manifest-path services/$$service/Cargo.toml --out Lcov --output-dir coverage/$$service || exit 1; \
	done

# Security
.PHONY: audit
audit:
	@echo "Running security audit..."
	@cargo audit

# Docker Operations
.PHONY: docker-build
docker-build:
	@for service in $(SERVICES); do \
		echo "Building Docker image for $$service..."; \
		$(DOCKER) build -t $$service -f services/$$service/Dockerfile . || exit 1; \
	done

.PHONY: docker-push
docker-push:
	@for service in $(SERVICES); do \
		echo "Pushing Docker image for $$service..."; \
		$(DOCKER) tag $$service:latest $(DOCKER_REPO)/$$service:latest; \
		$(DOCKER) push $(DOCKER_REPO)/$$service:latest || exit 1; \
	done

# Cleanup
.PHONY: clean
clean:
	@for service in $(SERVICES); do \
		echo "Cleaning $$service..."; \
		$(CARGO) clean --manifest-path services/$$service/Cargo.toml || exit 1; \
	done

.PHONY: clean-docker
clean-docker:
	@echo "Cleaning Docker images..."
	@for service in $(SERVICES); do \
		$(DOCKER) rmi $$service:latest 2>/dev/null || true; \
		$(DOCKER) rmi $(DOCKER_REPO)/$$service:latest 2>/dev/null || true; \
	done

# Development Setup
.PHONY: setup
setup:
	@echo "Setting up development environment..."
	rustup component add clippy rustfmt
	cargo install cargo-audit cargo-tarpaulin

# Service-specific targets
.PHONY: $(SERVICES)
$(SERVICES):
	@echo "Building $@..."
	@$(CARGO) build --release --manifest-path services/$@/Cargo.toml

# Individual service commands
.PHONY: auth-service-test
auth-service-test:
	@$(CARGO) test --manifest-path services/auth-service/Cargo.toml

.PHONY: auth-service-run
auth-service-run:
	@$(CARGO) run --manifest-path services/auth-service/Cargo.toml

# CI target
.PHONY: ci
ci: format clippy test audit
	@echo "All CI checks passed!"

# Help
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  all        - Run format, clippy, test, build"
	@echo "  ci         - Run all CI checks (format, clippy, test, audit)"
	@echo "  format     - Check code formatting"
	@echo "  clippy     - Run clippy linting"
	@echo "  test       - Run tests"
	@echo "  build      - Build in release mode"
	@echo "  audit      - Security audit"
	@echo "  docker-build - Build Docker images"
	@echo "  setup      - Install development tools"
	@echo "  clean      - Clean build artifacts"