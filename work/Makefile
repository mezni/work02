CARGO = cargo
RUSTFLAGS = -D warnings

SERVICES = auth-service #config-service
SERVICE_DIR = services

.PHONY: all format clippy build test clean

all: format clippy test build

format:
	@for service in $(SERVICES); do \
		echo "Checking formatting for $$service..."; \
		cd $(SERVICE_DIR)/$$service && $(CARGO) fmt -- --check || exit 1; \
	done

clippy:
	@for service in $(SERVICES); do \
		echo "Running clippy on $$service..."; \
		cd $(SERVICE_DIR)/$$service && $(CARGO) clippy -- -D warnings || exit 1; \
	done

build:
	@for service in $(SERVICES); do \
		echo "Building $$service..."; \
		cd $(SERVICE_DIR)/$$service && $(CARGO) build --release || exit 1; \
	done

test:
	@for service in $(SERVICES); do \
		echo "Testing $$service..."; \
		cd $(SERVICE_DIR)/$$service && $(CARGO) test || exit 1; \
	done

clean:
	@for service in $(SERVICES); do \
		echo "Cleaning $$service..."; \
		cd $(SERVICE_DIR)/$$service && $(CARGO) clean || exit 1; \
	done