-- Additional performance indexes for users table
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_users_role_active 
ON users(role, is_active);

CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_users_network_station 
ON users(network_id, station_id) 
WHERE network_id != 'X' OR station_id != 'X';

CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_users_source_active 
ON users(source, is_active);

CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_users_verified 
ON users(is_verified, created_at DESC);

CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_users_fulltext_search 
ON users USING gin(to_tsvector('english', 
    coalesce(email, '') || ' ' || 
    coalesce(username, '') || ' ' || 
    coalesce(first_name, '') || ' ' || 
    coalesce(last_name, '')
));

-- Additional indexes for audit_logs table
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_audit_user_action 
ON audit_logs(user_id, action, created_at DESC);

CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_audit_resource 
ON audit_logs(resource_type, resource_id, created_at DESC);

CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_audit_date_range 
ON audit_logs(created_at DESC);

CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_audit_ip_country 
ON audit_logs(ip_address, country) 
WHERE ip_address IS NOT NULL;

-- Partial index for recent audit logs (last 90 days)
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_audit_recent 
ON audit_logs(created_at DESC) 
WHERE created_at > NOW() - INTERVAL '90 days';