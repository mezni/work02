# 🔐 Auth-Service — Summary

The **auth-service** is the central identity & access-management domain service responsible for handling **business-level user accounts**, orchestrating **Keycloak authentication**, managing **company-based access**, and enforcing **role-driven permissions** across the entire platform.

It works alongside Keycloak (for authentication) and PostgreSQL (for domain state) to deliver a complete authorization solution.

---

## ⭐ Primary Responsibilities

### **1. User Management (Domain Logic)**
The auth-service stores and manages **business users**, including:

- admin  
- partner  
- operator  
- user  
- guest  

These users have **company-scoped relationships**, metadata, and domain-specific rules.

Examples:

- Assign a partner to company A  
- Operators only manage resources for their company  
- “user” role cannot belong to a company  
- “guest” is global and read-only  

---

## 🧩 2. Authentication via Keycloak

Keycloak handles:

- passwords  
- login & refresh tokens  
- JWT generation  
- client role assignment  

Auth-service interacts with Keycloak Admin API to:

- create users  
- assign roles  
- create groups (companies)  
- deactivate users  
- reset passwords  

---

## 🟦 3. Access Control Across Microservices

Your architecture uses **role + company + service-level** access rules.

| Role      | Company Scoped? | Access                                             |
|-----------|------------------|----------------------------------------------------|
| admin     | Yes              | Full access to all services                       |
| partner   | Yes              | Manage stations/org for their company             |
| operator  | Yes              | Operate & manage resources                        |
| user      | No               | Access **locator-service** only                   |
| guest     | No               | Read-only access to public resources              |

---

## ⚙️ 4. CQRS Architecture

Commands & queries are separated.

### **Commands (Write Model)**  
- create_user  
- assign_company  
- assign_role  
- deactivate_user  
- update_user  

Commands emit domain events.

### **Queries (Read Model)**  
- list_users  
- get_user_by_id  
- get_users_by_company  
- get_user_permissions  

Data is read from SQLx projections for performance.

---

## 📦 5. Domain Events

The service emits:

- `UserRegistered`
- `UserSelfRegistered`
- `UserCreatedByAdmin`
- `UserRoleAssigned`
- `UserCompanyAssigned`
- `UserDisabled`
- `UserPasswordReset`
- `GuestUserCreated`

These may be logged or published to other services.

---

## 🗄️ 6. PostgreSQL Domain Database

The auth-service database stores:

- business user metadata  
- company assignments  
- domain audit logs  
- domain events  
- CQRS query projections  

(Keycloak does not store this domain state.)

---

## 🧭 7. Self-Registration Flow

Self-registered users:

- have **no company**  
- automatically receive **role: user**  
- must activate via email  
- have access only to **locator-service**  

---

## 🔑 8. Middleware Enforcement

Actix Web middleware validates:

- JWT signatures (Keycloak public keys)  
- user roles  
- company-scoped access  
- microservice-level permissions  
- guest read-only rules  

---

## ☑️ 9. OpenAPI / Swagger Support

Using `utoipa` + `utoipa-swagger-ui`, the service documents:

- authentication endpoints  
- user commands  
- query endpoints (CQRS read model)  
- company assignment  
- admin/user/partner/operator flows  

---

## 🛠️ 10. Why the Auth-Service Exists

Keycloak alone **cannot** handle:

- company scoping  
- partner/operator roles  
- domain events  
- audit logging  
- custom user metadata  
- CQRS  
- multi-service authorization  

The auth-service delivers the **business identity layer**, while Keycloak provides **authentication**.

---

## 🔥 In One Sentence

**The auth-service is the business identity and authorization domain for your platform, managing users, roles, companies, CQRS events, and Keycloak-backed authentication.**


# Auth-Service — API Documentation

The **auth-service** provides user and access management, company-based authorization, and Keycloak-backed authentication for the platform.

All endpoints return JSON.

---

# 🔑 Authentication

## POST /auth/login
Authenticate a user with Keycloak credentials.

**Request**
```json
{
  "username": "admin",
  "password": "password"
}
```
**Response**
```json
{
  "access_token": "jwt-here",
  "refresh_token": "refresh-here",
  "expires_in": 300
}
```

## POST /auth/refresh
Refresh the access token.

**Refresh the access token.**
```json
{
  "refresh_token": "xxx"
}
```

## POST /auth/logout
Invalidates Keycloak session.

# 👤 Self-Registration (Public)
## POST /auth/self-register

Creates a user without a company.
Automatically assigned role: user.
**Request**
```json
{
  "username": "john@example.com",
  "email": "john@example.com",
  "password": "secret123"
}
```

**Response**
```json
{
  "user_id": "uuid",
  "activation_required": true
}
```

## GET /auth/activate/{token}

Activates account via email link.

# Users (Admin / Partner / Operator)

These endpoints allow administrative and company-scoped management of users.
Authentication is required. Role restrictions apply.

---

## POST /users
Create a user.

### Authorization
- **admin** → can create *any* user
- **partner/operator** → can only create users inside their own company

### Notes
- Users created here *may* belong to a company.
- Self-registration users are **not** created from this endpoint.
- Valid roles: `admin`, `partner`, `operator`, `user`, `guest`
- `user` and `guest` **cannot** belong to a company.

### Request
```json
{
  "username": "dali",
  "email": "dali@company.com",
  "password": "secret",
  "role": "operator",
  "company_id": "c123"
}
```

### Response
```json
{
  "id": "u1",
  "username": "dali",
  "role": "operator",
  "company_id": "c123",
  "status": "active"
}
```
