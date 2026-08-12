# Nutanix Objects Manual: Monitoring, Alerts, and Event Notifications

## MONITORING AND ALERTS IN NUTANIX

## OBJECTS

Monitor object store usage, bucket performance, and system alerts in Nutanix Objects, with visibility rules based on user roles in Prism Central.You can monitor the usage and the performance of object stores and buckets, and you can also view system-generated alerts for Nutanix Objects.Non-admin users can view the usage and performance of object stores and buckets, and also view system-generated alerts for objects only if assigned a role with View Object Store permission by the admin user in Prism Central. Admins are Super Admins or Prism Admins in Prism Central; non-admins are Prism Central users without admin privileges.For more information on the built-in roles in Prism Central, see Built-in Role Management in the Security Guide.For the object counts that are shown at the object store instance level, Nutanix Objects counts each upload of a multipart upload as a separate object until the object is finalized. At the bucket level, only finalized objects are counted.For example, if you upload 10 objects with 5 parts each, the object store instance shows a count of 50, while the bucket shows 0 until the uploads are complete.

### Viewing Performance of Object Stores

You can view and analyze the performance of an object store by placing the cursor anywhere on the horizontal axis to display the value at that time. You can also select the time interval (last 1 hour, 3 hours, 6 hours, 12 hours, and 24 hours) from the pull-down list above the graphs.

### About this taskTo view the performance of an object store, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. In the object stores table, click the name of the object store to view its performance.

| The | Object Store page appears. Objects | Monitoring and Alerts in Nutanix Objects | |
| --- | --- |

4. Click Performance.

### Figure 37: Object Store Performance Graph

The graph shows the following information:

| a. | Requests Per Second: Displays the following graphs: |
| --- | --- |
| • The | Total graph displays the total input and output requests in each second in the object store or |

bucket.

| • The | Puts graph displays the total input in each second |
| --- | --- |
| • The | Gets graph displays the total output in each second. |
| b. | Throughput (MB per sec): The Throughput graph displays granular read-and-write throughput in |

MB in each second.You can see the total in, total out, gets, puts, NFS reads, and NFS writes in a bucket.

| c. | Time to First Byte (GET Operations): This graph displays the time taken to read the first byte from |
| --- | --- |

the object in milliseconds.

### Viewing Performance of Buckets

You can view and analyze the performance of a bucket by placing the cursor anywhere on the horizontal axis to display the value at that time. You can also select the time interval (last 1 hour, 3 hours, 6 hours, 12 hours, and 24 hours) from the pull-down list above the graphs.

### About this task

### Note: The Throughput chart shows data for each connection. This data is not the cumulative throughput

across all the clients and connections. To view the performance of a bucket, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects. . In the object stores table, click the name of the object store to view its performance.

| The | Object Store page appears. |
| --- | --- |

### 4. Click the Bucket tab, and then click the name of the bucket from the Buckets list table, and then click

Performance.

### Figure 38: Bucket Performance Graph

The graph displays the performance of the bucket.

| a. | Requests Per Second: Displays the following graphs: |
| --- | --- |
| • The | Total graph displays the total input and output requests in each second in the object store or |

bucket.

| • The | Puts graph displays the total input in each second |
| --- | --- |
| • The | Gets graph displays the total output in each second. |
| • The | NFS Reads graph displays the total NFS reads in each second. |
| • The | NFS Writes graph displays the total NFS writes in each second. |
| b. | Throughput (MB per sec): This graph displays granular read-and-write throughput in MB per |

second.You can see the total in, total out, gets, puts, NFS reads, and NFS writes in a bucket.

| c. | Time to First Byte (GET Operations): This graph displays the time taken to read the first byte from |
| --- | --- |

the object in milliseconds.

### Viewing Object Store Usage

The Usage tab displays the physical and logical storage usage. You can view the space used across all buckets in an object store.

### About this taskTo view the object store usage, follow these steps:

### Procedure

1. Log on to the Prism Central web console. . In the Application Switcher, click Objects.3. In the object stores table, click the name of the object store to view its usage.

| The | Object Store page appears. |
| --- | --- |

4. Click Usage > Usage Summary.

### Figure 39: Object Store Usage

The storage usage section displays the following information:Physical Usage: Displays the physical capacity used by the object store on the cluster.

| • You can switch between | Across Clusters and Cluster wise for the required view in graph. |
| --- | --- |

- Total used and total usable storage for the cluster.• Space used across all buckets on the existing cluster displayed in a bar.• Total and free physical capacity of the cluster.• Logical capacity assigned for the object store.Logical Usage: Displays the logical capacity used by the object store on the cluster. As the

Redundancy Factor is 2 (RF-2), the data is stored in two backups.

- Data stored locally• Data tiered to another S3 endpoint

### Assigning Quota Policy to a User

Assigning a quota policy to a user Nutanix Objects to set soft and hard thresholds on storage limits of all buckets owned by the user and number of buckets created by the user within an object store. You can assign quota policy to multiple users at the same time. If a soft threshold is set and if any user exceeds the limit, they can still create buckets and objects, however an alert is raised in the Alerts page. However, if a hard threshold is set, then the user is restricted from exceeding the limit. For example, if a user has 20 TB storage and has six buckets, then storage for all the six buckets will be accounted. If you are an owner of a bucket and have shared your bucket with multiple users, then your storage quota will be used up even if other users use the shared bucket storage. Quota policies will be automatically removed within 24 hours after the user is deleted. If some multi-part operations are pending, those pending parts will count towards the quota. Objects | Monitoring and Alerts in Nutanix Objects |

Before you beginA non-admin user can assign quota policy to a user only after the administrator creates an access control policy on a role in Prism Central for the non-admin user. The specific role must have the following minimum permissions:

- Create Quota Policy• Delete Quota Policy• View Quota Policy• View Object Store

### Note: Administrators and non-admin users are Prism Central users with the following roles:

- Administrator—A Super Admin or a Prism Admin in Prism Central.• Non-admin user—A Prism Central user without any administrator privileges.For more information on the built-in roles in Prism Central, see

Built-in Role Management in the Security Guide.

### About this task

- You cannot list the buckets only owned by a particular user. So, if you have exceeded your storage

quota and try to delete a bucket shared with you to release the storage space, the quota level remains as is. You have to delete your own bucket to release the storage space.For example, if user A has exceeded the storage quota and tries to list all the buckets, the bucket list will also contain the buckets that were shared with user A by user B. To release the storage space within user A quotas, if user A deletes objects in the buckets shared with user A by user B, the quota level remains as is.You can run get-bucket-acl command on the bucket to list the user owned buckets, and then delete the data from that bucket.

- Quota policies are enforced for buckets or space usage across all namespaces, federated or non-

federated alike.

- If federation is enabled, quotas for a particular owner/user are enforced only on the OSS cluster where

the policy was created and not on other OSS clusters which are part of the same federation.

### Important:

- Usage is accounted under the owner of the bucket, and not under the user who uploaded the

objects to the bucket.For example, user A and user B each have a 50 GiB quota. User A creates bucket X and shares it with user B, and both users upload 10 GiB to bucket X. User A's usage is 20 GiB and user B's usage is 0 GiB, because all usage is attributed to user A, the owner of bucket X.

- Admin-created buckets (buckets created from the UI) are not accounted for under any quota.

To assign a quota policy to a user, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects. Objects | Monitoring and Alerts in Nutanix Objects |

### 3. In the object stores table, click the name of the object store to see the object store usage and to assign

quota policy to the users.

| The | Object Store page appears. |
| --- | --- |

4. Click Usage > Quota Policy.5. Select the user for which you want to create the quota policy. If you are creating quota policy for the first time, skip this step. 6. Click + Create Policy.

### Figure 40: Configure Quota

| A | Configure Quota window appears. |
| --- | --- |

7. Select a user or multiple users from the People drop-down. You can create quote policy for multiple users simultaneously, but you cannot assign multiple quotas to the same user. 8. In the Quota Limit section, select either one check box or both check boxes, and enter the limit.

| a. (Optional) Click the | Storage Limit check box and enter the total storage capacity limit for the user. |
| --- | --- |

Capacity Limit cannot be larger than the current available storage.

| b. (Optional) Click the | Bucket Number Limit check box and enter the number of buckets the user can |
| --- | --- |

create. . In the Quota Limit Enforcement section, select any of the following, and then click Save.

| » | Hard limit: Once the user reaches the specified quota limits, the request is blocked. |
| --- | --- |
| » | Soft limit: Once the user has exceeded the specified quota limits, only alerts are raised and the |

request is not blocked. You can always update the existing policy.A list of users with the configured storage- and bucket-quota, and the current usage is displayed.When the storage usage or bucket usage reaches 90% of the defined limit, you are notified with an exclamation mark.A background task checks for any violation of quota policies and generates this alert. So, it can take about an hour to generate the alert. On a heavily loaded object store, it can take up to a day to generate an alert. What to do nextYou can view the alerts. For more information, see Viewing Nutanix Objects Alerts on page 172.

### Viewing Nutanix Objects Alerts

You can view any informational, warning, and critical alerts in a tabular format. The table presents each alert with the color-coded severity level, a description of the alert, and the time stamp.

### About this task

### Note: Nutanix Objects-related alerts are now displayed in the Prism Central Alerts section as well. You see

a banner at the top of the page to open the Prism Central alerts page. To view object store alerts, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. In the object stores table, click the name of the object store.

| The | Object Store page appears. Objects | Monitoring and Alerts in Nutanix Objects | |
| --- | --- |

4. Click the Alerts tab.

### Figure 41: Alerts List in the Nutanix Objects Alerts Page

The alerts table displays the following information:

**Table 15: Alerts Table**

| Field | Description |
| --- | --- |
| Name | Name of the alert |
| Description | Description of the alert issue |
| Severity | Severity level of the alert: |
| • | Critical: Requires immediate attention |
| • | Warning: Needs attention soon |
| • | Info: Condition to be aware of. |
| Created | Number of days since the alert was created. Hover over the number of days to view the date and time when the alert was created. Objects | Monitoring and Alerts in Nutanix Objects | |

| Field | Description |
| --- | --- |
| Silenced | Suppress an alert. |
| You can also suppress multiple alerts by using | Silence at the top-left of the screen. |

### Note:

- Silencing alerts cannot be done from Prism Central alerts page.• Silencing of alerts is temporary.

| You can use | Filters at the top-right corner to filter the alerts you want to view. You can filter the alerts by |
| --- | --- |
| severity levels ( | Critical, Warning, and Info) and state (Active and Silenced). |
| You can also view these alerts from the Prism Central | Alerts page. The following image shows the |
| list of Nutanix Objects alerts on Prism Central | Alerts page. The Source Entity column for the Nutanix |

Objects alerts will be the object store name.

### Figure 42: Alerts List in the Prism Central Alerts page

### Nutanix Objects Specific Alerts

The following tables describe the Nutanix Objects specific alerts.Apart from the Nutanix Objects specific alerts, alerts that monitor the underlying infrastructure of the Nutanix Objects clusters are also triggered. For more information on these alerts, contact Nutanix Support at https://portal.nutanix.com. For more information on MSP alerts on Nutanix Objects, see MSP Alerts on Nutanix Objects Clusters on page 183.

**Table 16: Nutanix Objects Alert - High Time to First Byte**

| Title | High Time to First Byte |
| --- | --- |
| Name | HighTimeToFirstByte |
| Description | The HighTimeToFirstByte alert appears when the Time To First Byte (TTFB) for all the HTTP GET operations in the past 10 minutes exceeds 1 second. |
| Alert Message | Get operations issued to the object store in the past 10minutes have been showing TTFB of <value> msec Objects | Monitoring and Alerts in Nutanix Objects | |

| Cause | High network latency, improper sizing, and component crashes may generateHighTimeToFirstByte alert. |
| --- | --- |
| Impact | The response for Object GET requests becomes slow. |
| Resolution | Do the following: • Check for the networking issues impacting the nodes in the Prism Element cluster. • Check if the GETs in each second and PUTs |
| in each second shown on the | Performance page of the object store exceeds the estimated workload at the time of deployment. If there is an indication of an overload, consider reducing the workload on the Object Store Service (OSS). • Check for crashing services on the object store.• Check the Prism Element cluster on which you deployed the object store for any alerts that may be relevant to HighTimeToFirstByte alert. |

**Table 17: Nutanix Objects Alert - High Object Store Usage**

| Title | High Object Store Usage |
| --- | --- |
| Name | HighObjectStoreUsage |
| Description | The HighObjectStoreUsage alert appears when the total object store space usage exceeds the estimated capacity specified at the time of deployment. |
| Alert Message | Current object store usage <val> TB exceeds theprovisioned capacity <val>TB |
| Cause | The following may cause the HighObjectStoreUsage alert to generate. • Missing lifecycle management policies• Performing too many object operations. |
| Impact | Object store gets overloaded, thus causing slow performance. Objects | Monitoring and Alerts in Nutanix Objects | |

| Resolution | Do the following: • Delete nonessential objects from the object store to reduce the number of objects in the object store. • If the object store usage is unexpectedly high based on the workload and available resources, check for misconfigured Lifecycle Policies on the buckets in the object that may be retaining objects longer than intended. • If the storage space on the Prism Element cluster outside of the object store is high, consider deleting nonessential VMs, snapshots, data, or adding extra nodes to free the storage space in the Prism Element cluster. |
| --- | --- |

**Table 18: Nutanix Objects Alert - High Error Rate**

| Title | High Error Rate |
| --- | --- |
| Name | HighErrorRate |
| Description | The HighErrorRate alert appears when the object store returns one or more HTTP 4XX or HTTP 5XX errors in each second for the last 10 minutes. |
| Alert Message | Operations issued to Nutanix Buckets have been failingwith 5XX/4XX errors with observed error rate <val>/sover the past 10 minutes |
| Cause | Improper credentials and component crashes may generate the HighErrorRate alert. |
| Impact | The object store operations fail. |
| Resolution | Do the following: • Check the client applications for the correct access and secret key combination. • Check for any crashing services on the object store. • Check the Prism Element cluster on which you deployed the object store for any alerts that may be relevant to HighErrorRate alert. |

**Table 19: Nutanix Objects Alert - Endpoint <name> Unreachable**

| Title | Endpoint <cluster_name> Unreachable |
| --- | --- |
| Name | EndpointUnreachable |
| Description | The EndpointUnreachable alert appears when connectivity to the remote endpoint is down. Objects | Monitoring and Alerts in Nutanix Objects | |

| Alert Message | Connectivity to the remote endpoint <cluster_name>is down. Identify and fix the connectivity issue. |
| --- | --- |
| Cause | The network connectivity to the remote endpoint cluster is lost. |
| Impact | This issue can result in object GET or PUT failures. |
| Resolution | Check the network connectivity to the remote endpoint cluster and fix the connectivity issue. |

**Table 20: Nutanix Objects Replication Alert - Replication Pause Time For a Bucket Exceeded by 24**

### Hours

| Title | Replication Pause Time For a Bucket Exceeded by 24 Hours |
| --- | --- |
| Name | PauseReplicationWarningDurationTimeExceeded |
| Description | The PauseReplicationWarningDurationTimeExceeded alert appears when the replication rule is paused for more than 24 hours and if at least one object was uploaded during the pause state. |
| Alert Message | Pause replication duration time from source bucket to destination bucket has exceeded warning pause time by duration. |
| Cause | The replication rule was paused for more than 24 hours and at least one object was uploaded during the pause state. |
| Impact | If the replication of a bucket is paused, it can cause potential data loss and bloat the number of objects pending for replication. |
| Resolution | Resume the replication to avoid bloating the number of objects pending for replication. |

**Table 21: Nutanix Objects Replication Alert - Last replication sync time for a bucket exceeded by 12**

### hours

| Title | Last replication sync time for a bucket exceeded by 12 hours |
| --- | --- |
| Name | ReplicationWarningRPOTimeExceeded |
| Description | The ReplicationWarningRPOTimeExceeded alert appears when the replication of pending objects does not finish even after 12 hours of RPO. |
| Alert Message | Last sync time for bucket <bucket name> exceeded RPOtime by <time_period_secs> |
| Cause | The cause can be a combination of many reasons, such as low network bandwidth, sizing, replication failures, and so on. Objects | Monitoring and Alerts in Nutanix Objects | |

| Impact | Replication for one bucket has not synced within expected time and can cause potential data loss. |
| --- | --- |
| Resolution | Do the following: • Check if any other replication alerts are generated. • Check the connectivity or if the available network bandwidth is sufficient. • Check for any alerts generated due to service crashes. |

**Table 22: Nutanix Objects Replication Alert - Last replication sync time for a bucket exceeded by 3**

### days

| Title | Last replication sync time for a bucket exceeded by 3 days. |
| --- | --- |
| Name | ReplicationCriticalRPOTimeExceeded |
| Description | The ReplicationCriticalRPOTimeExceeded alert appears when the replication of pending objects does not finish even after 3 days of RPO. |
| Alert Message | Last sync time from bucket <source bucket name>to destination bucket <destination bucket> exceededCritical RPO time by <time_period_secs>. |
| Cause | The cause can be a combination of many reasons, such as low network bandwidth, sizing, replication failures, and so on. |
| Impact | Replication of one bucket has not synced within expected time and can cause potential data loss. |
| Resolution | Do the following: • Check if any other replication alerts are generated. • Check the connectivity or if the available network bandwidth is sufficient. • Check for any alerts generated due to service crashes. |

**Table 23: Nutanix Objects Replication Alert - Replication Endpoint Storage Full**

| Title | Replication Endpoint Storage Full |
| --- | --- |
| Name | ReplicationEndpointStorageFull |
| Description | The ReplicationEndpointStorageFull alert appears when the object store instance where the remote bucket exists runs out of storage space. Objects | Monitoring and Alerts in Nutanix Objects | |

| Alert Message | Storage full on replication endpoint <endpoint name>. |
| --- | --- |
| Cause | The remote objects instance storage becomes full. |
| Impact | Replication to the remote site fails since the destination object store is out of space. |
| Resolution | Add capacity to the remote objects PE cluster or delete existing objects to create space. |

**Table 24: Nutanix Objects Replication Alert - Replication Endpoint Unreachable**

| Title | Replication Endpoint Unreachable |
| --- | --- |
| Name | ReplicationEndpointUnreachable |
| Description | The ReplicationEndpointUnreachable alert appears when the object store instance where the remote bucket exists is unreachable for 15 mins. |
| Alert Message | Replication to endpoint <endpoint name> has lostconnectivity for the last 15 minutes |
| Cause | The following can be the reasons for this alert: • The network connectivity to the remote object store instance is lost. • The remote object store instance is down. |
| Impact | Replication of bucket to remote site fails since remote site is unreachable. |
| Resolution | Do the following: • Check the network connectivity to the remote objects instance. • Check if the remote objects instance is up and running. |

**Table 25: Nutanix Objects Replication Alert - Bucket Sync Completed**

| Title | Bucket Sync Completed |
| --- | --- |
| Name | kBucketSyncCompleted |
| Description | The system raises this alert to notify the user upon successful completion of the bucket sync operation. |
| Alert Message | Sync operation has completed for bucket:source_bucket_ name |
| Cause | The synchronization of bucket replication has been completed, and the status has changed from "Running" to "Completed." |
| Impact | Alerts are displayed on both Prism Central and |
| Nutanix Objects | Alerts page. Objects | Monitoring and Alerts in Nutanix Objects | |

| Resolution | No resolution is required. The alert will automatically resolve itself after 7 days. |
| --- | --- |

**Table 26: Nutanix Objects Replication Alert - Few Nonconforming Nutanix Objects Skipped during**

### Sync

| Title | Few nonconforming objects skipped during sync |
| --- | --- |
| Name | kObjectsSkippedInSync_<bucket_name> |
| Description | The system raises this alert this alert when the bucket synchronization operation skips certain objects during replication. This usually occurs when the versioning state of the skipped objects does not match the versioning state of the destination bucket |
| Alert Message | Some objects were skipped due to differing versionstates.: Match the bucket version state and reissue sync toreplicate all objects. |
| Cause | Some objects were skipped because they have different version states. |
| Impact | Alerts are displayed on both Prism Central and |
| Nutanix Objects | Alerts page. |
| Resolution | Match the bucket version state and reissue sync to replicate all objects. |

**Table 27: Nutanix Objects Replication Alert - Replication Sync Operation on Bucket Stalled**

| Title | Replication Sync operation on bucket stalled |
| --- | --- |

Name kBucketSyncProgressStalled

| Description | The system raises this alert when the bucket synchronization operation is stalled for longer than expected, indicating it is unable to make progress. |
| --- | --- |
| Alert Message | Unexpected error occurred while syncing bucket. IfReplication Sync does not complete, please contactNutanix support. |
| Cause | An unexpected error occurred while syncing the bucket. |
| Impact | Alerts are displayed on both Prism Central and |
| Nutanix Objects | Alerts page. |
| Resolution | If replication sync does not complete, contact Nutanix Support. Objects | Monitoring and Alerts in Nutanix Objects | |

**Table 28: Nutanix Objects Replication Alert - Bucket Replication Sync has Pending Replication**

### Entries

| Title | Bucket Replication Sync has pending replication entries |
| --- | --- |
| Name | kBucketSyncPendingEntriesTimeout |
| Description | The system raises this alert when the bucket synchronization operation has pending replication entries for a long time. |
| Alert Message | Replication Sync operation is stuck for a while. Pleasecontact Nutanix support. |
| Cause | The replication synchronization operation has been stuck for a while. |
| Impact | Alerts are displayed on both Prism Central and |
| Nutanix Objects | Alerts page. |
| Resolution | If replication sync does not complete, contact Nutanix Support. |

**Table 29: Nutanix Objects NFS Alert - High NFS Ops Drop Rate**

| Title | High NFS Ops Drop Rate |
| --- | --- |
| Name | HighNfsOpsDropRate |
| Description | The HighNfsOpsDropRate alert is triggered when some of the NFS ops are not being executed for past 10 minutes. |
| Alert Message | Due to high payload, few operations submitted to NFSare not executed for the past 10 minutes. Operationsare dropped when the outstanding NFS operations haveexceeded the threshold value of 1000, or when the QoSqueue has reached its maximum capacity of 128 ops, orwhen the operation wasn't admitted to the queue within10 seconds. |
| Cause | The following can be the reasons for this alert: • NFS payload is causing the number of ops to exceed the threshold value of 1000. • Overall high payload is causing QoS queue to reach its maximum capacity. |
| Impact | NFS ops are being dropped. Latencies may increase due to client retries. Objects | Monitoring and Alerts in Nutanix Objects | |

| Resolution | Do the following: • Reduce the overall payload on the impacted bucket. • Modify maximum number of concurrent NFS requests.For example, in CentOS, this can be done by changing /proc/sys/sunrpc/tcp_slot_table_entries value to 128. |
| --- | --- |

**Table 30: Nutanix Objects Too Many Delete Markers**

| Title | Too Many Delete Markers |
| --- | --- |
| Name | kTooManyDeleteMarkers |
| Description | The kTooManyDeleteMarkers alert is an early warning triggered when the number of objects and delete markers in a bucket exceed safe thresholds. |
| Alert Message | The average number of delete markers for each object isgreater than 1 million. |
| Cause | This typically occurs when a bucket is misconfigured with versioning enabled (instead of disabled), causing applications to leave behind delete markers that are never cleaned up. |
| Impact | Accumulated delete markers can lead to kTimeouts and other performance issues when serving LIST- class S3 requests. |
| Resolution | Use life-cycle policies on versioned buckets to define retention and expiry policies on delete- markers. |

**Table 31: Nutanix Objects Too Many Versions**

| Title | Too Many Versions |
| --- | --- |
| Name | kTooManyVersions |
| Description | The kTooManyVersions alert is an early warning triggered when average number of versions for each unique object name is greater than a set threshold 1000. |
| Alert Message | The average number of versions for each object is greaterthan 1000. |
| Cause | This typically occurs when the average number of versions for each object is high. |
| Impact | Certain Nutanix Objects version APIs may experience performance degradation. Objects | Monitoring and Alerts in Nutanix Objects | |

| Resolution | Use life-cycle policies on versioned buckets to define retention and expiry policies on non-current versions of objects. |
| --- | --- |

### MSP Alerts on Nutanix Objects Clusters

The following tables describe the MSP alerts on Nutanix Objects clusters. For more information on these alerts, contact Nutanix Support at https://portal.nutanix.com.

**Table 32: MSP Alert - Kube Pod Crash Looping**

| Title | Kube Pod Crash Looping |
| --- | --- |
| Name | KubePodCrashLooping |
| Description | The KubePodCrashLooping alert appears when a pod in the Nutanix Objects cluster is in a crash loop. |
| Alert Message | Pod namespace/pod container is in waiting state dueto CrashLoopBackOff |
| Cause | The pod enters a crash loop because the application becomes unresponsive or crashes, leading Kubernetes to attempt automatic restarts. |
| Impact | Service degradation or unavailability |
| Resolution | Do the following: • Check pod events. $ kubectl -n $NAMESPACE describe pod $POD • Check pod logs. $ kubectl -n $NAMESPACE logs $POD -c $CONTAINER |

**Table 33: MSP Alert - Kube Persistent Volume Filling Up**

| Title | Kube Persistent Volume Filling Up |
| --- | --- |
| Name | KubePersistentVolumeFillingUp |
| Description | The KubePersistentVolumeFillingUp alert appears when a volume used by a stateful pod in the Nutanix Objects cluster is nearing its capacity. |
| Alert Message | The PersistentVolume claimed bypersistentvolumeclaim in Namespace namespace is only value free. |
| Cause | The volume is filling up due to specific reasons related to the application.. |
| Impact | Service degradation Objects | Monitoring and Alerts in Nutanix Objects | |

| Resolution | Review the application configuration. You might need to delete unnecessary data after identifying the cause. |
| --- | --- |

**Table 34: MSP Alert - Kube Client Certificate Expiration**

| Title | Kube Client Certificate Expiration |
| --- | --- |
| Name | KubeClientCertificateExpiration |
| Description | The KubeClientCertificateExpiration alert appears when a client certificate used for authenticating to the Kubernetes API server is nearing its expiration date. |
| Alert Message | A client certificate used to authenticate to the apiserver isexpiring in less than 90 days. Please perform certificaterotation to avoid downtime. Check KB 13311. |
| Cause | A client certificate used for authenticating to the Kubernetes API server is nearing its expiration date. |
| Impact | The certificate is not automatically rotated and expires on the expiration date. |
| Resolution | Plan and upgrade MSP prior to the certificate expiration date. The MSP upgrade automatically rotates the certificate. |

**Table 35: MSP Alert - Kube API Down**

| Title | Kube API Down |
| --- | --- |
| Name | KubeAPIDown |
| Description | The KubeAPIDown alert is triggered when the monitoring system cannot reach all Kubernetes API servers for more than 15 minutes. |
| Alert Message | KubeAPI has disappeared from Prometheus targetdiscovery. |
| Cause | The Kubernetes API server is not responding. |
| Impact | The cluster might be partially or fully non-functional. Applications that do not use the Kubernetes API directly continue to function as expected, while services that use the Kubernetes API directly might exhibit erratic behavior. Objects | Monitoring and Alerts in Nutanix Objects | |

| Resolution | Do the following: • Check pod events. $ kubectl -n kube-system describe pod -l component=kube-controller-manager One or two pods might be present. • Check the logs of each pod: $ kubectl -n kube-system logs $POD -c kube-apiserver |
| --- | --- |

**Table 36: MSP Alert - Kubelet Down**

| Title | Kubelet Down |
| --- | --- |
| Name | KubeletDown |
| Description | The KubeletDown alert is triggered when the monitoring system cannot reach Kubelets of any of the cluster for over 15 minutes. |
| Alert Message | Kubelet has disappeared from Prometheus targetdiscovery |
| Cause | The Kubernetes API server is not responding. |
| Impact | This alert signifies a critical threat to the stability of the cluster. If there is no network issue affecting the ability of the monitoring system to collect Kubelet metrics, multiple nodes might be unable to respond to configuration changes for pods and other resources. Also, debugging tools like kubectlexec and kubectl logs might not work. |
| Resolution | Check the status of nodes, recent events on node objects, or recent events in general: $ kubectl get nodes $ kubectl describe node $NODE_NAME $ kubectl get events --field-selector 'involvedObject.kind=Node' $ kubectl get events With SSH access to the nodes, access the logs for the Kubelet directly: $ sudo journalctl -u kubelet-master / kubelet-worker |

For more information on Nutanix Objects specific alerts, see Nutanix Objects Specific Alerts on page 174. Objects | Monitoring and Alerts in Nutanix Objects |

## NUTANIX OBJECTS NOTIFICATIONS

Notifications for Nutanix Objects enables you to send the completed events logs to the configured endpoints in your Nutanix Objects instance. This helps you with centralized events log management, thus enabling you to monitor and analyze the logs and identify performance or configuration issues. TCP is the supported protocol for Nutanix Objects notifications.The following endpoints are supported for logging the notification events:

| • | Syslog—System Logging Protocol is a standard protocol for sending events logs to the Syslog server. |
| --- | --- |

Enter the hostname or host IP address and port number of your Syslog server when configuring the endpoints. The Syslog server should be up and running when performing the endpoint configuration in your Nutanix Objects instance.

| • | Nats-streaming—A lightweight, reliable streaming platform built on top of the core NATS platform |
| --- | --- |

that provides persistent logs. Enter the hostname or host IP address and port number of your NATS streaming server when configuring the endpoints. NATS streaming server should be up and running when performing the endpoint configuration in your Nutanix Objects instance.

### Note: The default topic used to create the NATS queue is OSSEvents. You can use this as the subject

while using the NATS client to connect to the NATS streaming server.

| • | Apache Kafka—Kafka is a distributed system consisting of servers and clients that communicate |
| --- | --- |

through a high-performance TCP network protocol. It can be deployed on bare-metal hardware, virtual machines, and containers in on-premise as well as cloud environments. Enter the hostname or host IP address and port number of your Kafka server when configuring the endpoints. The Kafka server should be up and running when performing the endpoint configuration in your Nutanix Objects instance.

### Notification Types for Nutanix Objects

There are two types of notifications for Nutanix Objects that can be enabled for either successful, or failed, or both successful and failed instance-level and data events.

| • | Instance-level events—All operations performed on a bucket. For example, create a bucket, update |
| --- | --- |

a bucket, delete a bucket, enable versioning, and so on. The following instance-level events are supported for successful, or failed, or both successful and failed events: Note: All the instance-level events notifications are enabled by default once you configure the endpoints. You cannot enable or disable the notifications for a single instance-level event.

**Table 37: S3 APIs for Only Successful Instance-level Events**

| Supported Instance-level Events | S3 APIs for Only Successful Instance-level Events |
| --- | --- |
| Create bucket | S3:BucketCreated:Put |
| Delete bucket | S3:BucketRemoved:Delete |
| Add bucket policy | S3:BucketPolicy:Put |
| Enable static website | S3:BucketWebsite:Put |
| Disable static website | S3:BucketWebsite:Delete |
| Create bucket CORS | S3:BucketCORS:Put Objects | Nutanix Objects Notifications | |

| Supported Instance-level Events | S3 APIs for Only Successful Instance-level Events |
| --- | --- |
| Delete bucket CORS | S3:BucketCORS:Delete |
| Update bucket lifecycle policy | S3:BucketLifecyclePolicy:Update |
| Delete bucket lifecycle policy | S3:BucketLifecyclePolicy:Delete |
| Enable bucket lock (WORM) | S3:BucketLock:Enabled |
| Disable bucket lock (WORM) | S3:BucketLock:Disabled |
| Add bucket replication rule | S3:BucketReplication:Put |
| Delete bucket replication rule | S3:BucketReplication:Delete |
| Enable bucket versioning | S3:BucketVersioning:Enabled |
| Suspend bucket versioning | S3:BucketVersioning:Disabled |
| Enable bucket notification | S3:BucketNotify |

**Table 38: S3 APIs for Only Failed Instance-level Events**

| Supported Instance-level Events | S3 APIs for Only Failed Instance-level Events |
| --- | --- |
| Create bucket | S3:BucketCreatedError:Put |
| Delete bucket | S3:BucketRemovedError:Delete |
| Add bucket policy | S3:BucketPolicyError:Put |
| Enable static website | S3:BucketWebsiteError:Put |
| Disable static website | S3:BucketWebsiteError:Delete |
| Create bucket CORS | S3:BucketCORSError:Put |
| Delete bucket CORS | S3:BucketCORSError:Delete |
| Update bucket lifecycle policy | S3:BucketLifecyclePolicyError:Update |
| Delete bucket lifecycle policy | S3:BucketLifecyclePolicyError:Delete |
| Enable bucket lock (WORM) | S3:BucketLockError:Enabled |
| Disable bucket lock (WORM) | S3:BucketLockError:Disabled |
| Add bucket replication rule | S3:BucketReplicationError:Put |
| Delete bucket replication rule | S3:BucketReplicationError:Delete |
| Enable bucket versioning | S3:BucketVersioningError:Enabled |
| Suspend bucket versioning | S3:BucketVersioningError:Disabled Objects | Nutanix Objects Notifications | |

| Supported Instance-level Events | S3 APIs for Only Failed Instance-level Events |
| --- | --- |
| Enable bucket notification | S3:BucketNotifyError |

**Table 39: S3 APIs for Both Successful and Failed Instance-level Events**

| Supported Instance-level Events | S3 APIs for Both Successful and Failed Instance-level Events |
| --- | --- |
| Create bucket | S3:BucketCreated:Put and S3:BucketCreatedError:Put |
| Delete bucket | S3:BucketRemoved:Delete and S3:BucketRemovedError:Delete |
| Add bucket policy | S3:BucketPolicy:Put and S3:BucketPolicyError:Put |
| Enable static website | S3:BucketWebsite:Put and S3:BucketWebsiteError:Put |
| Disable static website | S3:BucketWebsite:Delete and S3:BucketWebsiteError:Delete |
| Create bucket CORS | S3:BucketCORS:Put and S3:BucketCORSError:Put |
| Delete bucket CORS | S3:BucketCORS:Delete and S3:BucketCORSError:Delete |
| Update bucket lifecycle policy | S3:BucketLifecyclePolicy:Update and S3:BucketLifecyclePolicyError:Update |
| Delete bucket lifecycle policy | S3:BucketLifecyclePolicy:Delete and S3:BucketLifecyclePolicyError:Delete |
| Enable bucket lock (WORM) | S3:BucketLock:Enabled and S3:BucketLockError:Enabled |
| Disable bucket lock (WORM) | S3:BucketLock:Disabled and S3:BucketLockError:Disabled |
| Add bucket replication rule | S3:BucketReplication:Put and S3:BucketReplicationError:Put |
| Delete bucket replication rule | S3:BucketReplication:Delete and S3:BucketReplicationError:Delete |
| Enable bucket versioning | S3:BucketVersioning:Enabled and S3:BucketVersioningError:Enabled |
| Suspend bucket versioning | S3:BucketVersioning:Disabled and S3:BucketVersioningError:Disabled |
| Enable bucket notification | S3:BucketNotify and S3:BucketNotifyError |

For more information, see

- Configuring Event Notifications in Nutanix Objects for Syslog on page 191.• Configuring Event Notifications in Nutanix Objects for Kafka on page 197

Objects | Nutanix Objects Notifications |

- Configuring Event Notifications in Nutanix Objects for NATS on page 194The following is the structure of the notification output for instance-level events that get published to the

endpoints. {"EventType":"s3:BucketCreated:Put","Key":"first","Records": [{"eventError":"","eventVersion":"2.0","eventSource":"aws:s3","awsRegion":"us- east-1","eventTime":"2022-02-09T07:40:13Z","eventName":"s3:BucketCreated:Put","userIdentity": {"principalId":"upgrade@ntnx.com"},"requestParameters": {"sourceIPAddress":"10.44.91.64"},"responseElements": {"x-amz-id-2":"110000+1229+2129+240","x-amz-request- id":"110000+1229+2129+240","x-ntnx-origin-endpoint":"0.0.0.0"},"s3": {"s3SchemaVersion":"1.0","configurationId":"SuccessfulWriteManagementEvent","OSSDomain":"rc4.eng.nutanix.com","bucket": {"name":"frst","ownerIdentity": {"principalId":"upgrade@ntnx.com"},"arn":"arn:aws:s3:::frst"}}}],"level":"info","msg":"","time":"2022-02-09T07:40:13Z"}

| • | Data events—Data events are specific to data operations. These events can be enabled for each |
| --- | --- |

bucket. To enable notifications for successful, or failed, or both data events for a bucket, you need to

| create notification rules. You can define the scope of a notification rule by selecting | All Objects or |
| --- | --- |

### Subset of objects. For more information, see Creating Notification Rules for Data Events in Nutanix

Objects on page 201.The following is the structure of the notification output for data events that get published to the endpoints. {"EventType":"s3:ObjectCreated:Put","Key":"put","Records": [{"eventError":"","eventVersion":"2.0","eventSource":"aws:s3","awsRegion":"us- east-1","eventTime":"2022-02-09T08:04:48Z","eventName":"s3:ObjectCreated:Put","userIdentity": {"principalId":"upgrade@ntnx.com"},"requestParameters": {"sourceIPAddress":"10.44.91.64"},"responseElements": {"x-amz-id-2":"110000+1229+2791+297","x-amz-request- id":"110000+1229+2791+297","x-ntnx-origin-endpoint":"0.0.0.0"},"s3": {"s3SchemaVersion":"1.0","configurationId":"5836c513-51ef-45a9-9c84-6a0c01996e40","OSSDomain":"rc4.eng.nutanix.com","bucket": {"name":"put","ownerIdentity": {"principalId":"upgrade@ntnx.com"},"arn":"arn:aws:s3:::put"},"object": {"key":"csv.docx","size":"31","eTag":"c7a74379a77360a6b85926adb8b2aafb","versionId":"1","sequencer":"110000+1229+2791+297"}}}],"level":"info","msg":"","time":"2022-02-09T08:04:48Z"} The following data events are supported for successful, or failed, or both successful and failed events:

### Note: You can enable notifications for all Nutanix Objects Create Events, Nutanix Objects Access Events,

or Nutanix Objects Delete Events, or you can choose to enable notifications for one or more sub-categories of Nutanix Objects Create Events (such as Put, Post, Copy, Multipart Upload Complete), Nutanix Objects Access Events (such as Get, Head), or Nutanix Objects Delete Events (such as Delete, Delete Markers).

**Table 40: S3 APIs for Only Successful Data Events**

| Supported Data Events | S3 APIs for Only Successful Data Events |
| --- | --- |
| All Create Events | s3:ObjectCreated:* |
| Put | s3:ObjectCreated:Put |
| Post | s3:ObjectCreated:Post |
| Copy | s3:ObjectCreated:Copy |
| Multipart Upload | s3:ObjectCreated:CompleteMultipartUpload |
| All Access Events | s3:ObjectAccessed:* Objects | Nutanix Objects Notifications | |

| Supported Data Events | S3 APIs for Only Successful Data Events |
| --- | --- |
| Get | s3:ObjectAccessed:Get |
| Head | s3:ObjectAccessed:Head |
| All Delete Events | s3:ObjectRemoved:* |
| Delete | s3:ObjectRemoved:Delete |
| Delete Markers | s3:ObjectRemoved:DeleteMarkerCreated |

**Table 41: S3 APIs for Only Failed Data Events**

| Supported Data Events | S3 APIs for Only Failed Data Events |
| --- | --- |
| All Create Events | s3:ObjectCreatedError:* |
| Put | s3:ObjectCreatedError:Put |
| Post | s3:ObjectCreatedError:Post |
| Copy | s3:ObjectCreatedError:Copy |
| Multipart Upload | s3:ObjectCreatedError:CompleteMultipartUpload |
| All Access Events | s3:ObjectAccessedError:* |
| Get | s3:ObjectAccessedError:Get |
| Head | s3:ObjectAccessedError:Head |
| All Delete Events | s3:ObjectRemovedError:* |
| Delete | s3:ObjectRemovedError:Delete |
| Delete Markers | s3:ObjectRemovedError:DeleteMarkerCreated |

**Table 42: S3 APIs for Both Successful and Failed Data Events**

| Supported Data Events | S3 APIs for Both Successful and Failed Data Events |
| --- | --- |
| All Create Events | s3:ObjectCreated:* and s3:ObjectCreatedError:* |
| Put | s3:ObjectCreated:Put and s3:ObjectCreatedError:Put |
| Post | s3:ObjectCreated:Post and s3:ObjectCreatedError:Post |
| Copy | s3:ObjectCreated:Copy and s3:ObjectCreatedError:Copy |
| Multipart Upload | s3:ObjectCreated:CompleteMultipartUpload and s3:ObjectCreatedError:CompleteMultipartUpload |
| All Access Events | s3:ObjectAccessed:* and s3:ObjectAccessedError:* Objects | Nutanix Objects Notifications | |

| Supported Data Events | S3 APIs for Both Successful and Failed Data Events |
| --- | --- |
| Get | s3:ObjectAccessed:Get and s3:ObjectAccessedError:Get |
| Head | s3:ObjectAccessed:Head and s3:ObjectAccessedError:Head |
| All Delete Events | s3:ObjectRemoved:* and s3:ObjectRemovedError:* |
| Delete | s3:ObjectRemoved:Delete and s3:ObjectRemovedError:Delete |
| Delete Markers | s3:ObjectRemoved:DeleteMarkerCreated and s3:ObjectRemovedError:DeleteMarkerCreated |

### Configuring Event Notifications in Nutanix Objects for Syslog

Specify endpoints when configuring event notifications. By default, only successful events are logged. You can choose success, failure, both, or none. Before you beginYou must set up and maintain your own Syslog, NATS, or Kafka servers.

### About this taskNutanix Objects supports event notifications through Syslog, NATS, or Kafka. To log operations, enable

notifications to one of these services. Nutanix does not manage these services.Non-admin users can configure event notifications only if an admin assigns them a role with specific

| permissions in Prism Central. Required permissions include | Set Notification Endpoint, View Notification |
| --- | --- |

### Endpoint, and View Object store. Admins are Super Admins or Prism Admins in Prism Central;

non-admins are Prism Central users without admin privileges. For more information, see Built-in Role Management in the Nutanix Security Guide.To configure Syslog notifications, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. In the object stores table, click the name of the object store to view its performance.

| The | Object Store page appears. |
| --- | --- |

4. Click Endpoints > Notification Endpoints.

| The | Notification Endpoints page appears to configure the endpoints by entering the endpoint server |
| --- | --- |
| details. The available endpoints are | Syslog, Nats-streaming, and Kafka. You can enable any one of |

these endpoints or all of them.When you add a notification endpoint (Syslog, NATS, or Kafka) with correctly formatted fields, the operation succeeds even if the endpoint is unreachable or invalid. Errors during setup appear on the Object Store list page.

### 5. Click Actions > Enable

Endpoint is enabled. Objects | Nutanix Objects Notifications |

### 6. For a syslog endpoint, click Actions > Edit

Edit notification endpoint window appears. . In Syslog tab, specify the server connection details.

### Important: Syslog server must be up and running when performing the endpoint configuration in your

Nutanix Objects instance.

### Figure 43: Notification Endpoints: Syslog

Objects | Nutanix Objects Notifications |

| a. In the | Server Address box, enter the host name and port number of your Syslog server in the Host |
| --- | --- |

name:Port format.

| b. In the | Logged Events section, specify the Topic. |
| --- | --- |
| c. In the | Logged Events section, select one of the following object store events for which you plan to |
| enable notifications. By default, | Successful only option is selected. |
| • | Successful and Failed: Logs both successful and failed object store events. |
| • | Successful only: Logs only successful object store events. |
| • | Failed only: Logs only failed object store events. |
| • | None: Logs none of the object store events. |
| You can click the | View Events List link to view the list of bucket events that are logged to the |

endpoints. 8. Click Save to complete the events notification configuration.

### Note: After saving notification endpoints for an object store, the notification status might take one minute

to display correctly. Also, changes to the notification configuration can take up to five minutes to take effect. What to do nextTo configure Kafka and NATS notification, see Configuring Event Notifications in Nutanix Objects for Kafka on page 197, Configuring Event Notifications in Nutanix Objects for NATS on page 194.You can now create notification rules for data events of the bucket, see Creating Notification Rules for Data Events in Nutanix Objects on page 201.

### Configuring Event Notifications in Nutanix Objects for NATS

Specify endpoints when configuring event notifications. By default, only successful events are logged. You can choose success, failure, both, or none. Before you beginYou must set up and maintain your own Syslog, NATS, or Kafka servers.

### About this taskNutanix Objects supports event notifications through Syslog, NATS, or Kafka. To log operations, enable

notifications to one of these services. Nutanix does not manage these services.Non-admin users can configure event notifications only if an admin assigns them a role with specific

| permissions in Prism Central. Required permissions include | Set Notification Endpoint, View Notification |
| --- | --- |

### Endpoint, and View Object store. Admins are Super Admins or Prism Admins in Prism Central;

non-admins are Prism Central users without admin privileges. For more information, see Built-in Role Management in the Nutanix Security Guide.To configure events notifications, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects. . In the object stores table, click the name of the object store to view its performance.

| The | Object Store page appears. |
| --- | --- |

4. Click Endpoints > Notification Endpoints.

| The | Notification Endpoints page appears to configure the endpoints by entering the endpoint server |
| --- | --- |
| details. The available endpoints are | Syslog, Nats-streaming, and Kafka. You can enable any one of |

these endpoints or all of them.When you add a notification endpoint (Syslog, NATS, or Kafka) with correctly formatted fields, the operation succeeds even if the endpoint is unreachable or invalid. Errors during setup appear on the Object Store list page.

### 5. Click Actions > Enable

Endpoint is enabled.

### 6. For a Nats-streaming endpoint, click Actions > Edit

Edit notification endpoint window appears. . In the Nats-streaming tab, specify the server connection details.

### Important: Nats-streaming server must be up and running when performing the endpoint configuration

in your Nutanix Objects instance.

### Figure 44: Notification Endpoints: Nats-Streaming

Objects | Nutanix Objects Notifications |

| a. In the | Server Address box, enter the host name and port number of your NATS server in the Host |
| --- | --- |

name:Port format.

| b. In the | Cluster ID box, enter the cluster ID of the server that you used to start the NATS server. |
| --- | --- |
| c. In the | Logged Events section, specify the Topic. |
| d. In the | Logged Events section, select one of the following object store events for which you plan to |
| enable notifications. By default, | Successful only option is selected. |
| • | Successful and Failed: Logs both successful and failed object store events. |
| • | Successful only: Logs only successful object store events. |
| • | Failed only: Logs only failed object store events. |
| • | None: Logs none of the object store events. |
| You can click | View Events List link to view the list of bucket events that are logged to the endpoints. |

8. Click Save to complete the events notification configuration.

### Note: After saving notification endpoints for an object store, the notification status might take one minute

to display correctly. Also, changes to the notification configuration can take up to five minutes to take effect. What to do nextTo configure Kafka and syslog notification, see Configuring Event Notifications in Nutanix Objects for Kafka on page 197, Configuring Event Notifications in Nutanix Objects for Syslog on page 191.You can now create notification rules for data events of the bucket, see Creating Notification Rules for Data Events in Nutanix Objects on page 201.

### Configuring Event Notifications in Nutanix Objects for Kafka

Specify endpoints when configuring event notifications. By default, only successful events are logged. You can choose success, failure, both, or none. Before you beginYou must set up and maintain your own Syslog, NATS, or Kafka servers.

### About this taskNutanix Objects supports event notifications through Syslog, NATS, or Kafka. To log operations, enable

notifications to one of these services. Nutanix does not manage these services.Non-admin users can configure event notifications only if an admin assigns them a role with specific

| permissions in Prism Central. Required permissions include | Set Notification Endpoint, View Notification |
| --- | --- |

### Endpoint, and View Object store. Admins are Super Admins or Prism Admins in Prism Central;

non-admins are Prism Central users without admin privileges. For more information, see Built-in Role Management in the Nutanix Security Guide.To configure events notifications, follow these steps:

### Procedure

1. Log on to the Prism Central web console. . In the Application Switcher, click Objects.3. In the object stores table, click the name of the object store to view its performance.

| The | Object Store page appears. |
| --- | --- |

4. Click Endpoints > Notification Endpoints.

| The | Notification Endpoints page appears to configure the endpoints by entering the endpoint server |
| --- | --- |
| details. The available endpoints are | Syslog, Nats-streaming, and Kafka. You can enable any one of |

these endpoints or all of them.When you add a notification endpoint (Syslog, NATS, or Kafka) with correctly formatted fields, the operation succeeds even if the endpoint is unreachable or invalid. Errors during setup appear on the Object Store list page.

### 5. Click Actions > Enable

Endpoint is enabled.

### 6. For a Kafka endpoint, click Actions > Edit

Edit notification endpoint window appears. . In the Kafka tab, specify the server connection and security protocol details. When adding a Kafka endpoint, if the OSSEvents topic does not exist, the system creates it. If it already exists, ensure it matches your Kafka server configuration.

### Important: Kafka server must be up and running when performing the endpoint configuration in your

Nutanix Objects instance. Objects | Nutanix Objects Notifications |

### Figure 45: Notification Endpoints: Kafka

In the Kafka tab, enter the following details: Objects | Nutanix Objects Notifications |

### 1. Server Address: Enter the host name and port number of your Kafka server in the hostname:port

format.

### 2. Security Protocol Configuration: Select one of the following options:

| • | None: Select this option to use no encryption or authentication. |
| --- | --- |
| • | TLS: Select this option to encrypt communication using SSL/TLS. Upload the following files: |

- Root certificate• Client certificate• Client key

| • | mTLS: Select this option to use mutual TLS for encryption and client authentication. Upload the |
| --- | --- |

following files:

- Root certificate• Client certificate• Client key

### 3. Topic: Enter the topic name. If the OSSEvents topic does not exist, the system creates it

automatically.

### 4. Publish Message Key: Select No key for no message key. Select <Bucket-name>/ <Object-name>

for publishing <Bucket-name> as a key for management level events and <Bucket-name>/ <Object- name> as a key for data level events.

### 5. Object Store Events: Select one of the following options to determine which events are logged:

| • | Successful and Failed: Logs both successful and failed object store events. |
| --- | --- |
| • | Successful only: Logs only successful object store events. |
| • | Failed only: Logs only failed object store events. |
| • | None: Does not log any object store events. |

8. Click Save to complete the events notification configuration.

### Note: After saving notification endpoints for an object store, the notification status might take one minute

to display correctly. Also, changes to the notification configuration can take up to five minutes to take effect.

### What to do nextTo configure Syslog and NATS notification, see Configuring Event Notifications in Nutanix Objects for

Syslog on page 191, Configuring Event Notifications in Nutanix Objects for NATS on page 194.You can now create notification rules for data events of the bucket, see Creating Notification Rules for Data Events in Nutanix Objects on page 201.

### Creating Notification Rules for Data Events in Nutanix Objects

To track successfully completed data events, create notification rules for the buckets in Nutanix Objects.

### About this taskNon-admin users can create notification rules for data events only if assigned a role with the following

permissions by the admin user in Prism Central. Objects | Nutanix Objects Notifications |

- View Notification Endpoint• Edit Buckets

Users with this permission can create notification rules for data events across all buckets in the assigned object stores.

- View Buckets• View Object StoreAdmins are Super Admins or Prism Admins in Prism Central; non-admins are Prism Central users without

admin privileges. For more information on the built-in roles in Prism Central, see Built-in Role Management in the Security Guide.To create a notification rule for a bucket, follow these steps.

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. In the object stores table, click the name of the object store to view its performance.

| The | Object Store page appears. |
| --- | --- |

### 4. Click the Buckets tab, and then from the list of buckets, select the bucket for which you plan to create a

notification rule. 5. Click Data Event Notification.

| The | Data Event Notification page appears. |
| --- | --- |

6. Click Create Rule to open the Create Rule page. Configuring an endpoint is required to create notification rules. Configure endpoints that can be used to log both bucket events and data events. For more information, see Configuring Event Notifications in Nutanix Objects for Syslog on page 191, Configuring Event Notifications in Nutanix Objects for Kafka on page 197, and Configuring Event Notifications in Nutanix Objects for NATS on page 194. . (Optional) If a rule is already created for the bucket, click + Add Rule.

### Figure 46: Create Rule

### 8. In the Create Rule page, follow these steps:

| a. In the | Endpoint list, select the endpoint where you plan the data events for this bucket to be logged. |
| --- | --- |

The endpoints configured on the object store are listed in the dropdown here.To configure endpoints for your Nutanix Objects instance, see Configuring Event Notifications in Nutanix Objects for Syslog on page 191, Configuring Event Notifications in Nutanix Objects Objects | Nutanix Objects Notifications |

for Kafka on page 197, and Configuring Event Notifications in Nutanix Objects for NATS on page 194.

| b. In the | Scope list, select All Objects or Subset of objects. |
| --- | --- |
| • If you select | All Objects, the notification rule applies to all the objects. |
| • Select | Subset of objects to apply the notification rule to specific objects. You can filter Nutanix |

Objects by entering a prefix, suffix, or both. Nutanix Objects with the prefix and suffix you specified are filtered and the rule applies to those objects.

### Note:

| • You cannot proceed without providing a suffix or prefix for the scope | Subset of |
| --- | --- |

objects.

- You can enter only one prefix and suffix. You can create another rule with a different

suffix and prefix.

| c. In the | Data Events section, click the All Events check box to select all data events, or select a |
| --- | --- |
| check box next to the name of the events listed under | Objects Create Events, Objects Access |

### Events, and Objects Delete Events, and choose from the following that you plan to log to the

endpoint.

| • | Successful and Failed: Logs both successful and failed object store events. |
| --- | --- |
| • | Successful only: Logs only successful object store events. |
| • | Failed only: Logs only failed object store events. |
| When you select the | All Events check box, all the Objects Create Events, Objects Access |

Events, and Objects Delete Events are selected. 9. Click Save to complete, and then click Done to close the Create Rule page.

| The notification rule you created appears in the list of data events notification rules on the | Data |
| --- | --- |

### Event Notification page. You can click All Events or Number Events in the Events column to view

Objects | Nutanix Objects Notifications |

| the configured data events for that bucket. For example, click | 8 Events to view the 8 data events |
| --- | --- |

configured.

### Figure 47: Configured Notification Events

| Also, you can delete a notification rule. In the | Data Event Notification page, select a notification rule |
| --- | --- |
| from the list, and then click | Delete. |
| After you create a notification rule for a bucket, the status in the | Notifications column changes to |

Enabled. Objects | Nutanix Objects Notifications |
