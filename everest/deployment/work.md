export DATABASE_URL=postgresql://postgres:password@localhost:5700/evcs_db
export JWT_ISSUER=http://keycloak:8080/realms/myrealm
export JWKS_URL=http://keycloak:8080/realms/myrealm/protocol/openid-connect/certs
RUST_BACKTRACE=1 cargo run