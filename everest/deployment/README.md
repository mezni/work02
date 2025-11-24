Nice — below is a complete, ready-to-run Rust implementation that:

Adds API endpoints to create Keycloak users (POST /users)

Implements an Auth middleware that verifies tokens via Keycloak introspection, extracts roles and company_name (or groups) and attaches them to the request

Implements a permissions model layered on top of roles (create/read/update/delete/manage)

Enforces role-based and company-based access (admins bypass company restriction)

I keep everything reasonably small and practical so you can drop it into your project and iterate.

What you’ll get

Cargo.toml dependencies

.env variables used

src/ files:

main.rs — wiring and endpoints

config.rs — environment config

keycloak.rs — Keycloak interactions (create user, assign role, introspect token)

dto.rs — request/response DTOs

middleware.rs — Actix middleware that introspects token and injects AuthInfo

permissions.rs — permissions model (role -> permissions)

handlers.rs — HTTP handlers using service + permission checks

Example curl to create a user

Important notes before running

This code uses Keycloak’s token introspection endpoint to validate access tokens. That requires a client_id + client_secret that can introspect (a confidential client).

The code expects Keycloak to include company_name as a user attribute mapped into the access token (or you can use groups and adjust the extractor).

Provide correct values in .env.

This is synchronous-safe and production-adaptable; you may want caching for introspection response (TTL), better error handling, and metrics.

0) Add to Cargo.toml