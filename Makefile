# Variables
CARGO = cargo
RUSTFLAGS = -D warnings

# Services
SERVICES = configurator-service  # config-service (commented out for now)
SERVICE_DIR = services

# Docker (optional - if you need it later)
DOCKER_REPO ?= your-registry

.PHONY: all format format-fix clippy clippy-fix build test clean help

all: format clippy test build

# Code Quality
format:
	@for service in $(SERVICES); do \
		echo "📝 Checking formatting for $$service..."; \
		cd $(SERVICE_DIR)/$$service && $(CARGO) fmt -- --check || exit 1; \
	done

format-fix:
	@for service in $(SERVICES); do \
		echo "🔧 Fixing formatting for $$service..."; \
		cd $(SERVICE_DIR)/$$service && $(CARGO) fmt || exit 1; \
	done

clippy:
	@for service in $(SERVICES); do \
		echo "🔍 Running clippy on $$service..."; \
		cd $(SERVICE_DIR)/$$service && $(CARGO) clippy -- -D warnings || exit 1; \
	done

clippy-fix:
	@for service in $(SERVICES); do \
		echo "🔧 Fixing clippy suggestions for $$service..."; \
		cd $(SERVICE_DIR)/$$service && $(CARGO) clippy --fix -Z unstable-options -- -D warnings || exit 1; \
	done

# Building
build:
	@for service in $(SERVICES); do \
		echo "🔨 Building $$service..."; \
		cd $(SERVICE_DIR)/$$service && $(CARGO) build --release || exit 1; \
	done

build-dev:
	@for service in $(SERVICES); do \
		echo "🔨 Building $$service (dev)..."; \
		cd $(SERVICE_DIR)/$$service && $(CARGO) build || exit 1; \
	done

# Testing
test:
	@for service in $(SERVICES); do \
		echo "🧪 Testing $$service..."; \
		cd $(SERVICE_DIR)/$$service && $(CARGO) test || exit 1; \
	done

test-verbose:
	@for service in $(SERVICES); do \
		echo "🧪 Testing $$service (verbose)..."; \
		cd $(SERVICE_DIR)/$$service && $(CARGO) test -- --nocapture || exit 1; \
	done

# Service-specific targets
.PHONY: $(SERVICES)
$(SERVICES):
	@echo "🔨 Building $@..."
	@cd $(SERVICE_DIR)/$@ && $(CARGO) build --release

# Cleanup
clean:
	@for service in $(SERVICES); do \
		echo "🧹 Cleaning $$service..."; \
		cd $(SERVICE_DIR)/$$service && $(CARGO) clean || exit 1; \
	done

# Development
dev: format-fix clippy-fix test build-dev
	@echo "✅ Development build complete!"

# CI (same as 'all' but with explicit naming)
ci: format clippy test build
	@echo "✅ All CI checks passed!"

# Help
help:
	@echo "Available targets:"
	@echo "  all, ci    - Run format, clippy, test, build"
	@echo "  format     - Check code formatting"
	@echo "  format-fix - Fix code formatting"
	@echo "  clippy     - Run clippy linting"
	@echo "  clippy-fix - Fix clippy suggestions"
	@echo "  build      - Build in release mode"
	@echo "  test       - Run tests"
	@echo "  dev        - Development build (fix formatting, run tests, build)"
	@echo "  clean      - Clean build artifacts"
	@echo ""
	@echo "Service-specific:"
	@for service in $(SERVICES); do \
		echo "  $$service - Build specific service"; \
	done