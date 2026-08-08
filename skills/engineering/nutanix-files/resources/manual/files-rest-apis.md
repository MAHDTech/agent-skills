# Nutanix Files Manual: Files REST APIs

## Overview and Endpoints

Nutanix Files provides RESTful APIs for programmatic management, automated share provisioning, and telemetry retrieval.

### Key API Endpoints (v4 API Architecture)

```http
# List all file servers
GET /api/files/v4.0/config/file-servers

# Get file server details by ID
GET /api/files/v4.0/config/file-servers/{extId}

# Create a new SMB share / NFS export
POST /api/files/v4.0/config/file-servers/{extId}/shares

# Update share quotas or permissions
PATCH /api/files/v4.0/config/file-servers/{extId}/shares/{shareExtId}
```

---

## OData Filtering and Query Parameters

Nutanix v4 Files APIs support OData query options:

- `$filter`: Filter shares by name or protocol (e.g., `name eq 'vdi-profiles'`).
- `$select`: Return only specified fields (e.g., `extId,name,capacityBytes`).
- `$limit` & `$page`: Control pagination.

---

## Authentication Example (Python)

```python
import requests

url = "https://<prism-central-ip>:9444/api/files/v4.0/config/file-servers"
headers = {
    "Authorization": "Bearer <api_token>",
    "Content-Type": "application/json"
}

response = requests.get(url, headers=headers, verify=False)
print(response.json())
```
