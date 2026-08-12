# Nutanix Files Manual: Directory Services and User Management

## Active Directory and LDAP Integration

Nutanix Files integrates natively with Active Directory (AD) and OpenLDAP for identity, authentication, and access control.

### Active Directory Joining & Management

- **Joining AD**: Requires Domain Admin credentials or a pre-created Computer Account in an Organizational Unit (OU).
- **Machine Account Password Expiry**: Automated background password rotation between FSVMs and AD Domain Controllers.
- **Disjoint Domains**: Support for client access across non-trusting or disjoint AD forests using Kerberos / NTLM pass-through.

---

## User Mapping (NFS to SMB Identity Alignment)

In multiprotocol environments where files are accessed via both SMB and NFS, Nutanix Files provides a User Mapping engine to align Windows SIDs with Unix UIDs/GIDs.

### Mapping Rule Types

1. **Active Directory / LDAP Mapping**: Maps Windows usernames to Unix UIDs via RFC 2307 attributes (`uidNumber`, `gidNumber`) in AD.
2. **Explicit Mapping Rules**: Custom regex or explicit table rules mapping `DOMAIN\winuser` -> `unixuser`.
3. **Default Mapping**: Fallback mapping for unassigned accounts (e.g., mapping all unmapped users to guest account or predefined UID).

---

## Role-Based Access Control (RBAC)

Nutanix Files implements granular admin roles within Prism Central:

| Role Name | Description & Permissions |
| :--- | :--- |
| **Files Admin** | Full management rights across file servers, shares, quotas, networking, and security settings. |
| **Files Backup Admin** | Read-only rights for snapshot management, SSR configuration, and backup application service accounts. |
| **Files Operator** | Monitoring rights to view alerts, tasks, performance metrics, and capacity usage. |
| **REST API Limited User** | API key authenticated account restricted to specific REST endpoints (SMB share creation/querying). |
