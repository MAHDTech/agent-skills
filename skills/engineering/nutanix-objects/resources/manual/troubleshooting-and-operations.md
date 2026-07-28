# Nutanix Objects Manual: Troubleshooting and VM Lifecycle Operations

## TROUBLESHOOTING NUTANIX OBJECTS

This section explains how to troubleshoot issues that you might encounter while using Nutanix Objects. Handling Deployment FailureIn case you encounter any deployment failure, contact Nutanix Support at https://portal.nutanix.com/.

### Shutting Down Nutanix Objects VMs

This section describes the steps to perform a graceful shutdown of Nutanix Objects VMs on AHV or ESXi. Before you beginMake sure all read and write operations are stopped from the application before shutting down the VM.

### About this task

### Warning: Ensure that you only perform shutdown operations as described in this procedure. Do not perform

any destructive actions such as deleting a VM. To shut down Nutanix Objects VMs, follow these steps:

### Procedure

1. Log on to the Prism Central web console, and select Objects in the Application Switcher.2. Copy the Object Stores cluster name.

### Figure 64: Object Store Cluster

3. SSH into any CVM on the Prism Element cluster where the object store is deployed. Objects | Troubleshooting Nutanix Objects |

### 4. To view the list of VMs, do any one of the following:

| » | AHV: To view the list of VMs, run the following command. |
| --- | --- |

nutanix@NTNX-18FM6F370127-A-CVM:xx.xx.xxx.xx:~$ acli vm.list |grep '<objectstore- name>' -i For example, nutanix@NTNX-18FM6F370127-A-CVM:xx.xx.xxx.xx:~$ acli vm.list |grep'OSS-1611836167' -i , where OSS-1611836167 is the object store name. nutanix@NTNX-18FM6F370127-A-CVM:XX.XX.XXX.XX:~$ acli vm.list |grep 'OSS-1611836167' -i oss-1611836167-898db8-default-0 6596b630-cacd-48bd-90b5-b4f1698e260a oss-1611836167-898db8-ijpsvgbict-envoy-0 c25b5fd8-2787-46b6-b3b4-c982f6ddf1b9

| » | ESXi: To view the list of VMs in vCenter, click Hosts and Clusters tab, expand the ESXi cluster. |
| --- | --- |

The list of VMs appear.To find the primary and secondary MSP clusters, run the command mspctl cluster list. If the cluster_type value is primary_msp, then the cluster is a primary MSP cluster. Rest clusters are secondary.

### Note: It is recommended to shut down the envoy VMs first, and then proceed to shut down the worker

VMs. Also, ensure that the primary cluster is shut down at last as it hosts all the common services such as IAM.

### 5. To shut down the VMs, do any one of the following:

| » | AHV: Run the following command: |
| --- | --- |

acli vm.shutdown <vm name>

| » | ESXi: Right click the VM, and then click Power > Power Off. |
| --- | --- |

Note: First shut down the envoy VMs.

### Figure 65: Shutting Down the VMs: AHV

For AHV, you can check the status of the VM from the Prism Central web console. Click the

### Hamburger icon > Virtual Infrastructure > VMs and confirm that the status of the VM is shown as Off

| in the | Power State column. |
| --- | --- |
| For ESXi, you can check the status of the VM from the vCenter. Click the | Hosts and Clusters tab, and |

expand the cluster where the VM is listed, and confirm that the status of the VM is shown as Off. 6. Perform Step 5 for all the VMs within the cluster. What to do nextYou can also start the Nutanix Objects VMs. See Starting the Nutanix Objects VMs on page 274. Objects | Troubleshooting Nutanix Objects |

### Starting the Nutanix Objects VMs

This section describes the steps to start the Nutanix Objects VM after performing a graceful shutdown of Nutanix Objects VMs on AHV or ESXi.

### About this task

### Warning: Ensure that you only perform power on operations as described in this procedure. Do not perform

any destructive actions such as deleting a VM. To start the Nutanix Objects VMs, follow these steps:

### Procedure

1. Log on to the Prism Central web console, and select Objects in the Application Switcher.2. Copy the Object Stores cluster name.

### Figure 66: Object Store Cluster

3. SSH into any CVM on the Prism Element cluster where the object store is deployed.4. To view the list of VMs, do any one of the following:

| » | AHV: To view the list of VMs, run the following command. |
| --- | --- |

nutanix@NTNX-18FM6F370127-A-CVM:xx.xx.xxx.xx:~$ acli vm.list |grep '<objectstore- name>' -i For example, nutanix@NTNX-18FM6F370127-A-CVM:xx.xx.xxx.xx:~$ acli vm.list |grep'OSS-1611836167' -i , where OSS-1611836167 is the Object Store name. nutanix@NTNX-18FM6F370127-A-CVM:XX.XX.XXX.XX:~$ acli vm.list |grep 'OSS-1611836167' -i oss-1611836167-898db8-default-0 6596b630-cacd-48bd-90b5-b4f1698e260a oss-1611836167-898db8-ijpsvgbict-envoy-0 c25b5fd8-2787-46b6-b3b4-c982f6ddf1b9

| » | ESXi: To view the list of VMs in vCenter, click Hosts and Clusters tab, expand the ESXi cluster. |
| --- | --- |

The list of VMs appear.To find the primary and secondary MSP clusters, run the command mspctl cluster list. If the cluster_type value is primary_msp, then the cluster is a primary MSP cluster. Rest clusters are secondary. Objects | Troubleshooting Nutanix Objects |

### 5. To start the VMs, do any one of the following:

| » | AHV: Run the following command: |
| --- | --- |

acli vm.on <vm name>

| » | ESXi: Right click the VM, and then click Power > Power On. |
| --- | --- |

Note: First start the worker VMs. 6. Check the following to ensure that the VMs are powered on.

| • For AHV, in the Prism Central web console, go to the | VMs page. Select the VM that you powered on |
| --- | --- |
| using the acli and perform the | Launch Console action. You will view the login prompt if the VM is |

powered on.For ESXi, you can check the status of the VM from the vCenter. Click

### Hosts and Clusters tab, and

expand the cluster where the VM is listed, and confirm that the status of the VM is shown as On.

| • In the Prism Central web console, go to | Entity Menu > Services > Objects. Check for the following |
| --- | --- |

points for your Nutanix Objects cluster:

- Statistics are visible in the Buckets and Nutanix Objects columns.• Click on the corresponding Nutanix Objects cluster to check if it is reachable. After the Object

Store page opens, check if the various statistics are visible. For example, Performance, Usage Summary, and so on.

- In the Alerts page, ensure that there are no active alerts. If there are active alerts, wait for a few

minutes. If the alerts persist, contact Nutanix Support.

- Perform read and write operations to ensure that the object cluster is running.

### Detection of Slow Connections by Nutanix Objects

Nutanix Objects can detect slow-performing client connections to Nutanix Objects.Any client connection must be able to read or write data at the rate of 2 MiB in every 600 seconds window (translates to approximately 4 KiB/s throughput). A client connection that is not able to transfer 2 MiB data in 600 seconds window will be treated as a slow connection.Any slow connection will be terminated immediately to avoid Denial of Service (DoS) attacks and better manage the resources on the server.

### Note: Up to 1000 active connections are accepted by each Nutanix Objects endpoint. The slow-performing

client connections can potentially consume all slots and cause Denial of Service (DoS).

### Prism Element Clusters or Subnets Not Listed in the User Interface

If clusters or subnets are missing in the UI, it may be due to synchronization issues, lack of user access, or ongoing upgrades in Prism Central.If the

### Cluster or Subnets dropdown list appears empty or shows an error message, it indicates that no

Prism Element clusters or subnets are currently available for deployment. This issue might also occur when adding a multicluster to the object store.The following are the possible causes for the Prism Element clusters or subnets to not appear in the user interface:

- Prism Central has no other registered Prism Element clusters.

Objects | Troubleshooting Nutanix Objects |

- A newly registered Prism Element has not completed synchronization with Prism Central.• The logged-in user lacks access to any Prism Element cluster.• The logged in user does not have access to any subnets on the allowed Prism Element cluster.• An admin recently created an access policy granting user access to a Prism Element cluster.• An admin recently created an access policy granting user access to a subnet.• The Prism Element cluster is undergoing an upgrade or the upgrade status is not marked as Succeeded.

You can verify the cluster status in the Prism Central UI. For more information, see Clusters Summary View in the Prism Central Infrastructure Guide. Note: If you just register to a Prism Element or receive access, wait five minutes and try again.

### Object Stores Not Listed While Creating a Replication Rule

| You might see that while creating a replication rule for a bucket, the | Object Store drop-down under the |
| --- | --- |

Destination section shows an empty list.The following are the possible causes for the object stores to not appear in the user interface:

- The logged in user does not have permission to view the object store.• The deployment of the object store is not in

a Complete state.

- The object store version is lower than Nutanix Objects 3.0 version.

### Manually Updating DNS Servers for Nutanix Objects Clusters

When you update the DNS servers in the managed network for Nutanix Objects, the MSP VMs do not update automatically. You need to update each cluster manually.

### About this taskTo implement the updates, you must either restart the network service or restart the VMs. You can

manually update the DNS servers for each Nutanix Objects cluster using the managed network.To manually update the DNS servers for the Nutanix Objects cluster, follow these steps:

### Procedure

### 1. From the Prism Element CLI, update the name servers in the AHV managed networks:

nutanix@cvm$ acli net.update_dhcp_dns network_name servers=new server list For example: nutanix@cvm$ acli net.update_dhcp_dns OssNet servers=10.40.64.16 You can also update the network subnet using the Prism Central or Prism Element UI. For more information, see Updating a Subnet in the Flow Virtual Networking Guide and Modifying a Basic VLAN Subnet in Guest VM Interfaces in the Prism Element Web Console Guide. Objects | Troubleshooting Nutanix Objects |

### 2. Update the etc/resolv.conf file manually on the Nutanix Objects cluster with the new name servers in the

control planes and worker nodes: a. Check the configured name servers: nutanix@pcvm$ mspctl cluster ssh cluster-name --all --cmd "cat /etc/resolv.conf" For example: nutanix@pcvm$ mspctl cluster ssh dns-test-store --all --cmd "cat /etc/resolv.conf" b. Remove the immutable attribute: nutanix@pcvm$ mspctl cluster ssh cluster-name --all --cmd "sudo chattr -i /etc/ resolv.conf" For example: nutanix@pcvm$ mspctl cluster ssh dns-test-store --all --cmd "sudo chattr -i /etc/ resolv.conf" c. Remove each old name server: Caution: Do not remove the 127.0.0.1 entry. nutanix@pcvm$ mspctl cluster ssh cluster-name --all --cmd "sudo sed -i '/ nameserver old_ip_address/d' /etc/resolv.conf" For example: nutanix@pcvm$ mspctl cluster ssh dns-test-store --all --cmd "sudo sed -i '/ nameserver 10.40.64.16/d' /etc/resolv.conf" d. For each name server, add an entry in all nodes of the Nutanix Objects clusters: nutanix@pcvm$ mspctl cluster ssh cluster-name --all --cmd "echo "nameserver new_ip_address" | sudo tee -a /etc/resolv.conf" For example: nutanix@pcvm$ mspctl cluster ssh dns-test-store --all --cmd "echo "nameserver 10.40.64.17' | sudo tee -a /etc/resolv.conf" e. Return the immutable attribute: nutanix@pcvm$ mspctl cluster ssh cluster-name --all --cmd "sudo chattr +i /etc/ resolv.conf" For example: nutanix@pcvm$ mspctl cluster ssh dns-test-store --all --cmd "sudo chattr +i /etc/ resolv.conf" f. Verify the changes: nutanix@pcvm$ mspctl cluster ssh cluster-name --all --cmd "cat /etc/resolv.conf" For example: nutanix@pcvm$ mspctl cluster ssh dns-test-store --all --cmd "cat /etc/resolv.conf" Objects | Troubleshooting Nutanix Objects |

### 3. Update the etc/resolv.conf file manually on the Nutanix Objects cluster with the new name servers in the

envoy VMs: a. Check the configured name servers: nutanix@pcvm$ mspctl cluster ssh cluster-name --all -l --cmd "cat /etc/ resolv.conf" For example: nutanix@pcvm$ mspctl cluster ssh dns-test-store --all -l --cmd "cat /etc/ resolv.conf" b. Remove the immutable attribute: nutanix@pcvm$ mspctl cluster ssh cluster-name --all -l --cmd "sudo chattr -i /etc/ resolv.conf" For example: nutanix@pcvm$ mspctl cluster ssh dns-test-store --all -l --cmd "sudo chattr -i / etc/resolv.conf" c. Remove the old name servers: Caution: Do not remove the 127.0.0.1 entry. nutanix@pcvm$ mspctl cluster ssh cluster-name --all -l --cmd "sudo sed -i '/ nameserver old_ip_address/d' /etc/resolv.conf" For example: nutanix@pcvm$ mspctl cluster ssh dns-test-store --all -l --cmd "sudo sed -i '/ nameserver 10.40.64.16/d' /etc/resolv.conf" d. Add the new name servers: nutanix@pcvm$ mspctl cluster ssh cluster-name -all -l --cmd "echo "nameserver new_ip_address" | sudo tee -a /etc/resolv.conf" For example: nutanix@pcvm$ mspctl cluster ssh dns-test-store -all -l --cmd "echo "nameserver 10.40.64.16" | sudo tee -a /etc/resolv.conf" e. Return the immutable attribute: nutanix@pcvm$ mspctl cluster ssh cluster-name --all -l --cmd "sudo chattr +i /etc/ resolv.conf" For example, nutanix@pcvm$ mspctl cluster ssh dns-test-store --all -l --cmd "sudo chattr +i / etc/resolv.conf" f. Verify the changes: nutanix@pcvm$ mspctl cluster ssh cluster-name --all -l --cmd "cat /etc/ resolv.conf" For example: nutanix@pcvm$ mspctl cluster ssh dns-test-store --all -l --cmd "cat /etc/ resolv.conf" Objects | Troubleshooting Nutanix Objects |

### 4. Identify all of the DNS pods:

nutanix@pcvm$ mspctl cluster ssh cluster-name sudo kubectl get pods -A | grep -i dns For example: nutanix@pcvm$ mspctl cluster ssh dns-test-store ================== 10.44.90.142 ================== [nutanix@dns-test-store-16dcfd-default-0 ~]$ sudo kubectl get pods -A | grep -i dns

### 5. Restart all of the DNS pods by running the following command for each identified pod:

nutanix@pcvm$ mspctl cluster ssh cluster-name kubectl delete pod -n namespace pod For example, for pod coredns-d9bfcdf7f-5xtc7, run the following command: nutanix@pcvm$ mspctl cluster ssh dns-test-store ================== 10.44.90.142 ================== [nutanix@dns-test-store-16dcfd-default-0 ~]$ kubectl delete pod -n kube- system coredns-d9bfcdf7f-5xtc7

### 6. Restart the dnsmasq service:

nutanix@pcvm$ mspctl cluster ssh cluster-name --all --cmd "sudo systemctl restart dnsmasq" For example: nutanix@pcvm$ mspctl cluster ssh dns-test-store --all --cmd "sudo systemctl restart dnsmasq" The DNS servers for your Nutanix Objects clusters are updated. Objects | Troubleshooting Nutanix Objects |

## COPYRIGHT

Copyright 2026 Nutanix, Inc. Nutanix, Inc. 1740 Technology Drive, Suite 150 San Jose, CA 95110 All rights reserved. This product is protected by U.S. and international copyright and intellectual property laws. Nutanix and the Nutanix logo are registered trademarks of Nutanix, Inc. in the United States and/or other jurisdictions. All other brand and product names mentioned herein are for identification purposes only and may be trademarks of their respective holders. Objects | Copyright |
