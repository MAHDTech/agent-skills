# Nutanix Enterprise AI Manual: TLS Encryption and Certificate Management

TLS encryption setup, self-signed certificates via Helm, cert-manager ClusterIssuer integration (AWS Route 53, Let's Encrypt), custom TLS certificates, certificate rotation procedures, verification, and TLS troubleshooting.

---

```bash
nai-otel-collector-collector-nmwjw                              1/1     Running     0
16h
nai-otel-collector-targetallocator-c9dcc6544-5wbrx              1/1     Running     0
16h
nai-pulse-job-29522885-6w7pn                                    0/1     Completed   0
10h
nai-ui-6d9cc89b87-b4npf                                         1/1     Running     0
16h
nutanix-ai-operators-nai-clickhouse-operator-7f9965dbdd-trwp5   2/2     Running     0
16h
redis-standalone-6df56bc96d-hp86q                               2/2     Running     0
16h
```

- 

If you had endpoints in

#### Pending

status before the upgrade displaying the status as

#### Failed

with the message

Unable to pull runtime image with provided credentials, hibernate and resume the endpoints.

#### TLS Encryption on Nutanix Enterprise AI

Nutanix Enterprise AI (NAI) uses an Envoy-based gateway to terminate TLS for inbound HTTPS traffic.

You can configure the TLS certificate used by the NAI ingress gateway to enable secure client connections.

### Overview

The NAI Gateway listens on port 443 for HTTPS traffic. he gateway requires a Kubernetes TLS secret to serve encrypted traffic. The

Helm value specifies the TLS secret that the gateway uses. The

```bash
gateway.tlsSecretName
```

default value is

.

```bash
ingress-certificate
```

You can configure TLS encryption for NAI using one of the following methods:

Self-signed certificate through Helm

Choose this method for development, proof-of-concept, or air-gapped environments. For more information, see

Setting Up TLS Encryption using Self-Signed Certificate through Helm

on

page 120.

cert-manager with a ClusterIssuer

Choose this method for production environments that require automated certificate provisioning and renewal. For more information, see

Setting Up TLS Encryption using Your Own (Cluster)Issuer

(cert-manager)

on page 122. For example, you can set up a

for AWS Route 53

```bash
ClusterIssuer
```

and Let's Encrypt. For more information, see

Example: Setting up ClusterIssuer for AWS Route 53

and Let's Encrypt

on page 124.

Bring your own certificate

Choose this method to manage TLS certificates manually without cert-manager. For more information, see

Setting Up TLS Encryption using Your Certificate

on page 125.

Choose the TLS provisioning method that best matches your environment.

**Table 40: TLS Certificate Provisioning Methods**

| TLS certificate provisioning method | Recommended Usage | cert-manager required | Certificate Trust |
| --- | --- | --- | --- |
| Self-signed certificate through Helm | Development, proof-of- concept (PoC), or air- gapped lab environments | Yes | Untrusted (browser warning) |
| TLS certificate provisioning method | Recommended Usage | cert-manager required | Certificate Trust |
| Bring Your Own Issuer (cert-manager) | Production environments that require automated certificate provisioning and renewal. | Yes | Trusted (issued by your CA or ACME) |
| Bring Your Own Certificate | Production environments where certificates are managed manually | No | Trusted (if CA-signed) |

Prerequisites for TLS Certificate Provisioning Review required tools and cluster readiness before configuring TLS certificates for Nutanix Enterprise AI.

Ensure that the required tools are installed and that the Kubernetes cluster is ready before configuring TLS certificates for Nutanix Enterprise AI (NAI). Before configuring TLS certificates, ensure that you have the following:

- 

Kubernetes cluster: A supported Kubernetes cluster with NAI installed or ready for installation.

- 

Helm: Install Helm CLI v4.0.5 on your workstation.

- 

kubectl: Configure

to communicate with the target cluster.

```bash
kubectl
```

- 

Kubeconfig: The kubeconfig file for the NAI cluster. Set the kubeconfig for your Nutanix Enterprise AI cluster:

```bash
export KUBECONFIG=/path/to/nai-kubeconfig
```

Setting Up TLS Encryption using Self-Signed Certificate through Helm Use a self-signed certificate for development, proof-of-concept (PoC), or air-gapped environments where a certificate from a trusted certificate authority (CA) is not available. Do not use a self-signed certificate in production environments.

### Before you begin

Ensure that you meet the prerequisites listed in

Prerequisites for TLS Certificate Provisioning

on

page 120.

### About this task

The Helm chart creates a cert-manager Issuer and Certificate automatically; no manual certificate management is required.

> [!NOTE]
> Note:   Browsers display a security warning when you access NAI using a self-signed certificate because the certificate is not issued by a trusted CA. Do not use this method in production.

### Procedure

#### 1.  Install or upgrade NAI with the self-signed certificate option enabled:

```bash
helm upgrade --install nai-core nai-core/ \
-n nai-system \
--set gateway.certManager.selfSigned=true
```

This creates the following resources in the release namespace:

**Table 41: Resources created for self-signed TLS**

| Resource | Name | Purpose | Property 4 |
| --- | --- | --- | --- |
| Issuer | nai-selfsigned-issuer | Namespace-scoped self-signed issuer |  |
| Certificate | nai-ingressgateway- | Issues a certificate for |  |
|  | selfsigned-certificate | nai.nutanixdev.local | and |
|  |  | stores it in the secret specified by |  |
|  |  | gateway.tlsSecretName |  |

### 2.  Verify if the certificate is ready:

```bash
kubectl get certificate -n nai-system
```

Verify that the

status of

is

.

```bash
READY
nai-ingressgateway-selfsigned-certificate
True
NAME                                          READY   SECRET                AGE
nai-ingressgateway-selfsigned-certificate              True    ingress-certificate
30s
```

A

status of

confirms that cert-manager has issued the certificate and populated the TLS secret.

```bash
READY
True
```

### 3.  Get the external IP address of the gateway:

```bash
kubectl get svc -n envoy-gateway-system \
-l "gateway.envoyproxy.io/owning-gateway-name=nai-ingress-
gateway,gateway.envoyproxy.io/owning-gateway-namespace=nai-system" \
-o jsonpath='{.items[0].status.loadBalancer.ingress[0].ip}'
```

Note the external IP address returned by the command. You need this IP address to map the NAI hostname to the gateway.

#### 4.  Map the NAI hostname to the gateway IP address.:

The self-signed certificate is issued for

. Add an entry that maps this hostname to the

```bash
nai.nutanixdev.local
```

gateway external IP address in the hosts file on the system from which you access the NAI dashboard. Use the following hosts file for your operating system:

- 

For Linux, add

```bash
/etc/hosts
```

- 

For macOS, add

```bash
/etc/hosts
```

- 

For Windows, add

```bash
C:\Windows\System32\drivers\etc\hosts
```

Add the following:

```bash
<GATEWAY_IP>    nai.nutanixdev.local
```

Replace

with the external IP from

3

on page 121.

```bash
<GATEWAY_IP>
```

5. Access the NAI dashboard.

In your browser,

. Because the certificate is self-signed, the browser

```bash
https://nai.nutanixdev.local
```

displays a security warning. Accept the warning to continue to the NAI dashboard.

### What to do next

Helm values reference:

```bash
gateway:
tlsSecretName: "ingress-certificate"   # secret name written by cert-manager
certManager:
selfSigned: true                      # enables self-signed issuer + certificate
```

#### Setting Up TLS Encryption using Your Own (Cluster)Issuer (cert-manager) Use this method to configure TLS encryption for Nutanix Enterprise AI (NAI) in production environments where cert-manager is already deployed and a preconfigured

or namespace-scoped

```bash
ClusterIssuer
```

is available.

```bash
Issuer
```

### Before you begin

- 

Prerequisites for TLS Certificate Provisioning

Ensure that you meet the prerequisites listed in

on page 120.

- 

Ensure that you have the following:

- 

A cert-manager

or namespace-scoped

that is in a

state.

```bash
ClusterIssuer
Issuer
Ready
```

- 

The DNS name for the NAI deployment. For example,

.

```bash
nai.example.com
```

### About this task

The Helm chart creates a cert-manager Certificate resource that references your issuer. cert-manager then issues and manages the certificate lifecycle, including automatic renewal.

> [!NOTE]
> Important:   The NAI Helm chart does not create or manage the

or

. Create and configure

```bash
Issuer
ClusterIssuer
```

the issuer before installing or upgrading NAI.

### Procedure

### 1.  Verify the issuer is ready:

»

For a ClusterIssuer:

```bash
kubectl get clusterissuer <ISSUER_NAME>
```

»

For a namespace-scoped Issuer:

```bash
kubectl get issuer <ISSUER_NAME> -n nai-system
```

- 

Confirm the

column shows

.

```bash
READY
True
```

2. Install or upgrade NAI with the issuer reference.

Using a ClusterIssuer:

```bash
helm upgrade --install nai-core nai-core/ \
-n nai-system \
--set gateway.certManager.issuerRef.name=letsencrypt-prod \
--set gateway.certManager.issuerRef.kind=ClusterIssuer \
--set gateway.certManager.dnsNames[0]=nai.example.com
```

Using a namespace-scoped Issuer:

```bash
helm upgrade --install nai-core nai-core/ \
-n nai-system \
--set gateway.certManager.issuerRef.name=my-ca-issuer \
--set gateway.certManager.issuerRef.kind=Issuer \
--set gateway.certManager.dnsNames[0]=nai.example.com
```

To include additional Subject Alternative Names (SANs):

```bash
helm upgrade --install nai-core nai-core/ \
-n nai-system \
--set gateway.certManager.issuerRef.name=letsencrypt-prod \
--set gateway.certManager.issuerRef.kind=ClusterIssuer \
--set gateway.certManager.dnsNames[0]=nai.example.com \
--set gateway.certManager.dnsNames[1]=www.nai.example.com
```

(Optional) use a YAML values file for complex configurations:

```yaml
# tls-values.yaml
gateway:
tlsSecretName: "ingress-certificate"
certManager:
selfSigned: false
issuerRef:
name: letsencrypt-prod
kind: ClusterIssuer
# group: cert-manager.io   # optional; defaults to cert-manager.io
dnsNames:
- "nai.example.com"
- "www.nai.example.com"
helm upgrade --install nai-core nai-core/ \
-n nai-system \
-f tls-values.yaml
```

This creates the following resource in the release namespace:

**Table 42: Resource created for cert-manager issuer flow**

| Resource | Property 2 | Property 3 | Name | Property 5 | Property 6 | Purpose | Property 8 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Certificate |  |  | nai-ingressgateway- |  |  | References your issuer; cert-manager issues the certificate and writes it to the |  |
|  |  |  | certificate |  |  |  |  |
|  |  |  |  |  |  | gateway.tlsSecretName | secret |
| The first entry in | dnsNames | is also used as the certificate |  | commonName | . |  |  |

3. Verify the certificate is ready.
```bash
kubectl get certificate -n nai-system
```

Expected output:

```bash
NAME                             READY   SECRET                AGE
nai-ingressgateway-certificate   True    ingress-certificate   60s
```

4. Create a DNS record.

Get the gateway external IP or hostname:

```bash
kubectl get svc -n envoy-gateway-system \
-l "gateway.envoyproxy.io/owning-gateway-name=nai-ingress-
gateway,gateway.envoyproxy.io/owning-gateway-namespace=nai-system" \
-o jsonpath='{.items[0].status.loadBalancer.ingress[0].ip}'
```

If the output is empty, the service may use a hostname instead:

```bash
kubectl get svc -n envoy-gateway-system \
-l "gateway.envoyproxy.io/owning-gateway-name=nai-ingress-
gateway,gateway.envoyproxy.io/owning-gateway-namespace=nai-system" \
-o jsonpath='{.items[0].status.loadBalancer.ingress[0].hostname}'
```

Create an A record (for IP) or CNAME record (for hostname) in your DNS provider pointing

```bash
<NAI_HOSTNAME>
```

to the value returned above.

5. Access the NAI Dashboard.

Open

in your browser. The connection should be trusted if the issuer is backed by a

```bash
https://<NAI_HOSTNAME>
```

public or enterprise CA.

#### Example: Setting up ClusterIssuer for AWS Route 53 and Let's Encrypt

Example of setting up a

for AWS Route 53 and Let's Encrypt.

```bash
ClusterIssuer
```

### About this task

To set up a

for AWS Route 53 and Let's Encrypt, follow these steps:

```bash
ClusterIssuer
```

### Procedure

#### 1.  Create AWS Route 53 credential secret on Kubernetes:

```bash
kubectl create secret generic prod-route53-credentials-secret \
--from-literal=access-key-id="${AWS_ACCESS_KEY_ID}" \
--from-literal=secret-access-key="${AWS_SECRET_ACCESS_KEY}" \
-n cert-manager \
--dry-run=client -o yaml | kubectl apply -f -
```

### 2.  Create the

:

```bash
ClusterIssuer
apiVersion: cert-manager.io/v1
kind: ClusterIssuer
metadata:
name: nai-letsencrypt-cluster-issuer
spec:
acme:
email: admin@example.com
server: https://acme-v02.api.letsencrypt.org/directory
privateKeySecretRef:
name: nai-letsencrypt-cluster
solvers:
- dns01:
route53:
region: us-east-1
accessKeyIDSecretRef:
name: prod-route53-credentials-secret
key: access-key-id
secretAccessKeySecretRef:
name: prod-route53-credentials-secret
key: secret-access-key
hostedZoneID: <HOSTED_ZONE_ID>
kubectl apply -f clusterissuer.yaml
```

For HTTP-01 ACME Solver Configuration refer the following codeblock

When the ClusterIssuer or Issuer uses an HTTP-01 ACME challenge solver (common with Let's Encrypt), cert- manager must route challenge traffic through the NAI Gateway. Configure the solver as follows:

```bash
solvers:
- http01:
gatewayHTTPRoute:
parentRefs:
- group: gateway.networking.k8s.io
kind: Gateway
name: nai-ingress-gateway
namespace: nai-system
```

This instructs cert-manager to create a temporary HTTPRoute attached to the NAI Gateway so that ACME HTTP-01 challenge requests on port 80 are answered correctly during certificate issuance and renewal.

### 3.  Install NAI with the issuer reference:

```bash
helm upgrade --install nai-core nai-core/ \
-n nai-system \
--set gateway.certManager.issuerRef.name=nai-letsencrypt-cluster-issuer \
--set gateway.certManager.issuerRef.kind=ClusterIssuer \
--set gateway.certManager.dnsNames[0]=nai.example.com \
--set gateway.certManager.dnsNames[1]=www.nai.example.com
```

Setting Up TLS Encryption using Your Certificate Use this method in production environments where you have an existing TLS certificate and private key and do not want to use cert-manager. For example, you want to use a TLS certificate and private key isued by your organization's internal CA or a public CA such as DigiCert or Let's Encrypt.

### Before you begin

Ensure the following:

- 

Prerequisites for TLS Certificate Provisioning

Ensure that you meet the prerequisites listed in

on page 120.

- 

A valid TLS certificate file (PEM-encoded) and its corresponding private key file.

- 

The certificate Subject Alternative Names (SANs) include the hostname that you use to access NAI.

### About this task

To configure TLS using your own certificate, follow these steps:

### Procedure

1. Create the Kubernetes TLS secret.
```bash
kubectl create secret tls ingress-certificate \
--cert=/path/to/tls.crt \
--key=/path/to/tls.key \
-n nai-system
```

Ensure that the secret name matches the value of

(default:

).

```bash
gateway.tlsSecretName
ingress-certificate
```

If you use a different name, pass

during the Helm

```bash
--set gateway.tlsSecretName=<your-secret-name>
```

install or upgrade.

If your CA provides an intermediate certificate bundle, concatenate it with the server certificate before creating the secret:

```bash
cat server.crt intermediate.crt > tls-bundle.crt
kubectl create secret tls ingress-certificate \
--cert=tls-bundle.crt \
--key=server.key \
-n nai-system
```

2. Install or upgrade NAI.
- 

If you used the default secret name (

), do not add additional Helm flags:

```bash
ingress-certificate
helm upgrade --install nai-core nai-core/ \
-n nai-system
```

- 

If you used a custom secret name:

```bash
helm upgrade --install nai-core nai-core/ \
-n nai-system \
--set gateway.tlsSecretName=my-custom-cert
```

3. Verify the Gateway is serving HTTPS.

Confirm if the Envoy Gateway service is of type

and has an external IP:

```bash
LoadBalancer
kubectl get svc -n envoy-gateway-system -l "gateway.envoyproxy.io/owning-gateway-
name=nai-ingress-gateway,gateway.envoyproxy.io/owning-gateway-namespace=nai-system"
```

Expected output:

```bash
NAME                                            TYPE           EXTERNAL-IP
PORT(S)
envoy-nai-system-nai-ingress-gateway-*          LoadBalancer   198.51.100.10
80:31080/TCP,443:31443/TCP
```

4. Create a DNS record.

Create an A record (for IP) or CNAME record (for hostname) in your DNS provider that points your NAI hostname to the external IP or hostname from Step 3.

5. Access the NAI dashboard.

Open

in your browser.

```bash
https://<NAI_HOSTNAME>
```

### What to do next

Rotating a Manually

When you manage certificates manually, renew them before expiry. For more information, see

Managed TLS Certificate

on page 126.

Rotating a Manually Managed TLS Certificate Renew a manually managed TLS certificate before it expires and update the existing Kubernetes secret.

### About this task

To rotate a manually managed TLS certificate, follow these steps:

### Procedure

1. Obtain a renewed certificate and key from your certificate authority (CA).

### 2.  Update the existing secret:

```bash
kubectl create secret tls ingress-certificate \
--cert=/path/to/renewed-tls.crt \
--key=/path/to/renewed-tls.key \
-n nai-system \
--dry-run=client -o yaml | kubectl apply -f -
```

The gateway automatically uses the updated TLS certificate. You do not need to restart or patch.

Verifying TLS Configuration Verify that the NAI TLS configuration is correctly applied after setup.

### About this task

To confirm the setup is working correctly, follow these steps:

### Procedure

### 1.  Check the Gateway resource:

```yaml
kubectl get gateway nai-ingress-gateway -n nai-system -o yaml
```

Verify if the HTTPS listener references the expected secret:

```bash
listeners:
- name: https
protocol: HTTPS
port: 443
tls:
mode: Terminate
certificateRefs:
- kind: Secret
name: ingress-certificate      # must match gateway.tlsSecretName
namespace: nai-system
```

### 2.  Inspect the TLS secret:

```yaml
kubectl get secret ingress-certificate -n nai-system -o yaml
```

Confirm that

and

fields are present.

```bash
tls.crt
tls.key
```

### 3.  Test the HTTPS endpoint:

```bash
curl -v https://<NAI_HOSTNAME>
```

For self-signed certificates, add the

flag to skip verification:

```bash
-k
curl -vk https://nai.nutanixdev.local
```

### What to do next

Retrieve the IP address for the NAI dashboard. For more information, see

Accessing the NAI Dashboard IP

Address

on page  129.

Troubleshooting TLS certificate issues Troubleshoot common TLS certificate issues for NAI gateway access.

The following are common TLS certificate issues. To troubleshoot, follow the resolutions:

Gateway HTTPS listener is not ready

Possible cause: The TLS secret does not exist in the release namespace.

Resolution: Verify that the TLS secret exists.

```bash
kubectl get secret ingress-certificate -n nai-system
```

Browser shows a certificate name mismatch

Possible cause: The certificate Subject Alternative Names (SANs) do not include the hostname used to access NAI.

Resolution: Reissue the certificate with the correct DNS names.

cert-manager certificate remains in a False state

Possible cause: The issuer is not ready or DNS validation failed.

Resolution: Check

and

```bash
kubectl describe certificate <name> -n nai-system
kubectl get
```

or

for errors.

```bash
issuer
kubectl get clusterissuer
```

on port 443

```bash
ERR_CONNECTION_REFUSED
```

Possible cause: The Envoy Gateway service does not have an external IP address.

Resolution: Ensure that the service type is

and that the cloud provider or MetalLB assigned

```bash
LoadBalancer
```

an external IP address.

HTTP-01 challenge fails

Possible cause: The challenge traffic is not routed through the NAI gateway.

Resolution: Configure the

solver:

```bash
gatewayHTTPRoute
solvers:
- http01:
gatewayHTTPRoute:
parentRefs:
- group: gateway.networking.k8s.io
kind: Gateway
name: nai-ingress-gateway
namespace: nai-system
```

When the ClusterIssuer or Issuer uses an HTTP-01 ACME challenge solver (common with Let's Encrypt), cert-manager must route challenge traffic through the NAI Gateway. This instructs cert- manager to create a temporary

attached to the NAI Gateway so that ACME HTTP-01

```bash
HTTPRoute
```

challenge requests on port 80 are answered correctly during certificate issuance and renewal.

Certificate expired

Possible cause: The manually managed certificate was not renewed before expiration.

Resolution: Do one of the following:

- 

Rotate the certificate. For more information, see

Rotating a Manually Managed TLS Certificate

on

page 126.

- 

Consider switching to

for automatic certificate renewal.

```bash
cert-manager
```

Helm Values Reference The following table lists all gateway TLS-related Helm values.