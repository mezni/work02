-- ---------------------------
-- Table: companies
-- ---------------------------
CREATE TABLE companies (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);

-- ---------------------------
-- Table: users
-- ---------------------------
CREATE TABLE users (
    id UUID PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('Admin','Operator','Partner','User','Guest')),
    company_id UUID REFERENCES companies(id) ON DELETE SET NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);

-- ---------------------------
-- Table: permissions
-- ---------------------------
CREATE TABLE permissions (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);

-- ---------------------------
-- Table: roles_permissions
-- ---------------------------
CREATE TABLE roles_permissions (
    role TEXT NOT NULL,
    permission_id UUID NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    PRIMARY KEY (role, permission_id)
);

-- ---------------------------
-- Table: audit_logs
-- ---------------------------
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    action TEXT NOT NULL,
    entity TEXT NOT NULL,
    entity_id UUID,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT now(),
    details JSONB
);

-- ---------------------------
-- Indexes
-- ---------------------------
CREATE INDEX idx_users_company ON users(company_id);
CREATE INDEX idx_roles_permissions_role ON roles_permissions(role);
CREATE INDEX idx_audit_logs_user ON audit_logs(user_id);


-- ---------------------------
-- Insert Permissions
-- ---------------------------
INSERT INTO permissions (id, name, description)
VALUES
  ('11111111-1111-1111-1111-111111111111', 'create', 'Create resources'),
  ('22222222-2222-2222-2222-222222222222', 'read', 'Read resources'),
  ('33333333-3333-3333-3333-333333333333', 'update', 'Update resources'),
  ('44444444-4444-4444-4444-444444444444', 'delete', 'Delete resources'),
  ('55555555-5555-5555-5555-555555555555', 'manage', 'Manage resources');

-- ---------------------------
-- Map Roles to Permissions
-- ---------------------------

-- Admin: all permissions
INSERT INTO roles_permissions (role, permission_id)
VALUES
  ('Admin', '11111111-1111-1111-1111-111111111111'),
  ('Admin', '22222222-2222-2222-2222-222222222222'),
  ('Admin', '33333333-3333-3333-3333-333333333333'),
  ('Admin', '44444444-4444-4444-4444-444444444444'),
  ('Admin', '55555555-5555-5555-5555-555555555555');

-- Operator: CRUD permissions only
INSERT INTO roles_permissions (role, permission_id)
VALUES
  ('Operator', '11111111-1111-1111-1111-111111111111'),
  ('Operator', '22222222-2222-2222-2222-222222222222'),
  ('Operator', '33333333-3333-3333-3333-333333333333'),
  ('Operator', '44444444-4444-4444-4444-444444444444');

-- Partner: read + update + manage
INSERT INTO roles_permissions (role, permission_id)
VALUES
  ('Partner', '22222222-2222-2222-2222-222222222222'),
  ('Partner', '33333333-3333-3333-3333-333333333333'),
  ('Partner', '55555555-5555-5555-5555-555555555555');

-- User: read only
INSERT INTO roles_permissions (role, permission_id)
VALUES
  ('User', '22222222-2222-2222-2222-222222222222');

-- Guest: read only (non-company scoped)
INSERT INTO roles_permissions (role, permission_id)
VALUES
  ('Guest', '22222222-2222-2222-2222-222222222222');
