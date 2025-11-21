# configurator-service

A comprehensive EV station management system built with Rust, Actix-web, and SQLx.

## Features

- Station management
- Connector management
- Network management
- User management
- Real-time status updates
- RESTful API with Swagger documentation

## Entities

- network

## Getting Started

1. Clone the repository
2. Set up environment variables (copy .env.example to .env)
3. Run database migrations: `./scripts/migrate.sh`
4. Start the server: `cargo run`

## Project Structure

The project follows Domain-Driven Design (DDD) principles:

- `domain/`: Core business logic, entities, value objects
- `application/`: Use cases, commands, queries
- `infrastructure/`: Database, external services, configuration
- `api/`: Web API, routes, handlers
- `utils/`: Shared utilities

## Build

```bash
cargo build
cargo run
```

## API Documentation

Once running, access the API at: http://localhost:8080/api/
