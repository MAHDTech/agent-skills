# Nutanix Files Manual: Smart Tiering and Cloud Storage

## Smart Tiering Overview

Smart Tiering allows Nutanix Files to automatically offload cold or infrequently accessed file data to S3-compatible cloud storage targets, freeing up expensive primary HCI storage.

### Supported Tiering Targets

- **Nutanix Objects** (On-premises or hybrid S3)
- **AWS S3** (Standard, Infrequent Access)
- **Azure Blob Storage**
- **Wasabi Cloud Storage**

---

## Tiering Policies & Capacity Thresholds

- **Capacity Trigger**: Tiering activates when file server storage consumption exceeds a set threshold (e.g., 70% capacity).
- **Inactivity Period**: Files un-accessed for a specified duration (e.g., 30 days, 90 days) are selected for tiering.
- **File Size Filter**: Tiering targets files larger than a minimum size (e.g., > 64 KB).

---

## Data Recall Workflows

- **Transparent Access**: Tiered files remain visible in share directory listings as stub files.
- **Automatic Recall**: When a client reads or writes a stub file, Nutanix Files streams data back from the cloud tier transparently.
- **Manual Recall**: Admins can trigger bulk recall of tiered folders prior to offline migration.
