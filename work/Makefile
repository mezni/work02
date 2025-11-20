# Variables
CARGO = cargo
DOCKER = docker
RUSTFLAGS = -D warnings

# Services
SERVICES = auth-service
SERVICE_DIR = services

# Docker
DOCKER_REPO ?= your-registry

.PHONY: all format format-fix clippy build test test-coverage audit

all: format clippy test build

# Format each service individually
format:
	@for service in $(SERVICES); do \
		echo "Checking formatting for $$service..."; \
		cd $(SERVICE_DIR)/$$service && $(CARGO) fmt -- --check || exit 1; \
	done

format-fix:
	@for service in $(SERVICES); do \
		echo "Fixing formatting for $$service..."; \
		cd $(SERVICE_DIR)/$$service && $(CARGO) fmt || exit 1; \
	done

# Clippy for each service
clippy:
	@for service in $(SERVICES); do \
		echo "Running clippy on $$service..."; \
		cd $(SERVICE_DIR)/$$service && $(CARGO) clippy -- -D warnings || exit 1; \
	done

# Build each service
build:
	@for service in $(SERVICES); do \
		echo "Building $$service..."; \
		cd $(SERVICE_DIR)/$$service && $(CARGO) build --release || exit 1; \
	done

# Test each service
test:
	@for service in $(SERVICES); do \
		echo "Testing $$service..."; \
		cd $(SERVICE_DIR)/$$service && $(CARGO) test || exit 1; \
	done

# Coverage for each service
test-coverage:
	@echo "Installing coverage tools..."
	@cargo install cargo-tarpaulin 2>/dev/null || true
	@for service in $(SERVICES); do \
		echo "Generating coverage for $$service..."; \
		mkdir -p coverage/$$service; \
		cd $(SERVICE_DIR)/$$service && \
		cargo tarpaulin --out Lcov --output-dir ../../coverage/$$service --ignore-tests || exit 1; \
	done

# Security audit (run from root if you have a workspace Cargo.toml, otherwise per service)
audit:
	@if [ -f "Cargo.toml" ]; then \
		cargo audit; \
	else \
		for service in $(SERVICES); do \
			echo "Auditing $$service..."; \
			cd $(SERVICE_DIR)/$$service && cargo audit || exit 1; \
		done; \
	fi

# Docker operations
docker-build:
	@for service in $(SERVICES); do \
		echo "Building Docker image for $$service..."; \
		$(DOCKER) build -t $$service -f $(SERVICE_DIR)/$$service/Dockerfile . || exit 1; \
	done

clean:
	@for service in $(SERVICES); do \
		echo "Cleaning $$service..."; \
		cd $(SERVICE_DIR)/$$service && $(CARGO) clean || exit 1; \
	done

.PHONY: ci
ci: format clippy build test
	@echo "All CI checks passed!"