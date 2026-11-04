# Public indexer-cluster guidance

Use this map to bind each documented claim to the request. Record the product,
deployment type, exact release, topology or Cloud experience when material,
topic, direct content-bearing URL, and the claim it supports. Put the citation
beside the decisive conclusion.

The anchors below are current exact-release examples. If the named release is
different, retrieve the equivalent topic for that exact release before
answering. A `latest`, unversioned, adjacent-release, different-product, search
result, sitemap, or navigation-only page is non-decisive. If product, release,
deployment, topology, or Cloud experience is unknown, keep only the dependent
conclusion `decision-blocked` and request that discriminator.

Do not mix product branches. An Enterprise-only request uses Enterprise
evidence, Enterprise ownership, and exact-release Enterprise pages only; it must
not receive Cloud scope, CMC behavior, Service Details, or Cloud Support routing.
A Cloud request uses current exact-release Cloud CMC and Service Details sources
for customer-visible/provider boundaries; Enterprise procedures cannot fill a
provider-evidence gap.

Public documentation establishes supported behavior, not deployment health,
peer membership, factor state, bucket integrity, bundle application, completed
fix-up, maintenance readiness, search completeness, or recovery.

## Splunk Enterprise 10.4

Use these anchors only when self-managed Splunk Enterprise release `10.4`
applies. These pages can describe actions that this advisory skill must not
perform. Cite the relevant behavior or evidence field; keep every mutation with
a separately authorized operator and runbook.

### Cluster manager, peers, and health surfaces

- [View the manager node
  dashboard](https://help.splunk.com/en/splunk-enterprise/administer/manage-indexers-and-indexer-clusters/10.4/view-indexer-cluster-status/view-the-manager-node-dashboard)
  is the direct source for interpreting the manager dashboard's cluster, peer,
  index, bucket, and search-factor or replication-factor status fields. A
  dashboard state remains an observation and must be tied to identity and time.
- [View the peer
  dashboard](https://help.splunk.com/en/splunk-enterprise/administer/manage-indexers-and-indexer-clusters/10.4/view-indexer-cluster-status/view-the-peer-dashboard)
  is the peer-local companion source. Use it to compare peer-local and manager
  views without assuming that either proves distributed-search reachability.
- [Use the Monitoring Console to view indexer cluster
  status](https://help.splunk.com/en/splunk-enterprise/administer/manage-indexers-and-indexer-clusters/10.4/view-indexer-cluster-status/use-the-monitoring-console-to-view-indexer-cluster-status)
  is the direct Monitoring Console source. Preserve monitoring freshness,
  authentication, and identity separately from actual membership.
- [Indexer cluster
  states](https://help.splunk.com/en/splunk-enterprise/administer/manage-indexers-and-indexer-clusters/10.4/how-indexer-clusters-work/indexer-cluster-states)
  is the direct state-model source. A state label does not by itself establish
  cause, impact, or successful convergence.
- [What happens when a peer node goes
  down](https://help.splunk.com/en/splunk-enterprise/administer/manage-indexers-and-indexer-clusters/10.4/how-indexer-clusters-work/what-happens-when-a-peer-node-goes-down)
  and [what happens when a peer node comes back
  up](https://help.splunk.com/en/splunk-enterprise/administer/manage-indexers-and-indexer-clusters/10.4/how-indexer-clusters-work/what-happens-when-a-peer-node-comes-back-up)
  are the direct failure-and-return behavior sources. Use them to frame current
  evidence needs, never to claim a specific deployment recovered.

### Replication factor, search factor, buckets, and fix-up

- [Replication
  factor](https://help.splunk.com/en/splunk-enterprise/administer/manage-indexers-and-indexer-clusters/10.4/how-indexer-clusters-work/replication-factor)
  is the direct source for replication-copy behavior.
- [Search
  factor](https://help.splunk.com/en/splunk-enterprise/administer/manage-indexers-and-indexer-clusters/10.4/how-indexer-clusters-work/search-factor)
  is the direct source for searchable-copy behavior. Do not use replication
  factor as a proxy for search factor or vice versa.
- [Buckets and indexer
  clusters](https://help.splunk.com/en/splunk-enterprise/administer/manage-indexers-and-indexer-clusters/10.4/how-indexer-clusters-work/buckets-and-indexer-clusters)
  is the direct bucket-copy and cluster-role source.
- [Bucket replication
  issues](https://help.splunk.com/en/splunk-enterprise/administer/manage-indexers-and-indexer-clusters/10.4/troubleshoot-indexers-and-clusters-of-indexers/bucket-replication-issues)
  is the direct troubleshooting source for supported replication evidence and
  conditions. It may include repair procedures; this skill uses only its
  read-only interpretation and evidence requirements.
- [Anomalous bucket
  issues](https://help.splunk.com/en/splunk-enterprise/administer/manage-indexers-and-indexer-clusters/10.4/troubleshoot-indexers-and-clusters-of-indexers/anomalous-bucket-issues)
  is the direct anomalous-bucket source. Never translate it into bucket repair,
  deletion, movement, metadata, or primary-selection execution.

### Cluster-manager configuration bundles

- [Update common peer configurations and
  apps](https://help.splunk.com/en/splunk-enterprise/administer/manage-indexers-and-indexer-clusters/10.4/configure-the-peers/update-common-peer-configurations-and-apps)
  is the direct source for the documented cluster-manager bundle workflow. Use
  it to distinguish source, validation, distribution, application or restart
  requirement, and effective readback. Do not execute any phase.
- [Configuration bundle
  issues](https://help.splunk.com/en/splunk-enterprise/administer/manage-indexers-and-indexer-clusters/10.4/troubleshoot-indexers-and-clusters-of-indexers/configuration-bundle-issues)
  is the direct troubleshooting source for bundle evidence. Keep cluster-manager
  bundles separate from search-head knowledge bundles and deployer bundles.

### Rolling maintenance

- [Perform a rolling restart of an indexer
  cluster](https://help.splunk.com/en/splunk-enterprise/administer/manage-indexers-and-indexer-clusters/10.4/manage-the-indexer-cluster/perform-a-rolling-restart-of-an-indexer-cluster)
  is the direct release-specific rolling-restart source. Use it only to verify
  documented prerequisites, sequencing assumptions, and health checks that
  inform a readiness assessment. Do not perform, advance, or approve the
  restart.
- [Use maintenance
  mode](https://help.splunk.com/en/splunk-enterprise/administer/manage-indexers-and-indexer-clusters/10.4/manage-the-indexer-cluster/use-maintenance-mode)
  is the direct source for maintenance-mode behavior and cautions. This skill
  never enables, disables, or recommends executing maintenance mode.

Upgrade path, compatibility, sequencing, backup, recovery, and upgrade go/no-go
remain with `upgrade-planning-and-execution-readiness`; cite its exact target-
release upgrade sources for those decisions.

### Multisite indexer clusters

- [Multisite indexer cluster
  architecture](https://help.splunk.com/en/splunk-enterprise/administer/manage-indexers-and-indexer-clusters/10.4/overview-of-indexer-clusters-and-index-replication/multisite-indexer-cluster-architecture)
  is the direct architecture source for sites and failure domains.
- [Configure the site replication
  factor](https://help.splunk.com/en/splunk-enterprise/administer/manage-indexers-and-indexer-clusters/10.4/deploy-and-configure-a-multisite-indexer-cluster/configure-the-site-replication-factor)
  and [configure the site search
  factor](https://help.splunk.com/en/splunk-enterprise/administer/manage-indexers-and-indexer-clusters/10.4/deploy-and-configure-a-multisite-indexer-cluster/configure-the-site-search-factor)
  are the direct policy sources. Use them to interpret supplied configured
  policy, not to propose a configuration or infer that current site-level
  requirements are met.
- [Implement search affinity in a multisite indexer
  cluster](https://help.splunk.com/en/splunk-enterprise/administer/manage-indexers-and-indexer-clusters/10.4/deploy-and-configure-a-multisite-indexer-cluster/implement-search-affinity-in-a-multisite-indexer-cluster)
  is the direct source when site-specific search behavior or affinity is
  material. Keep search-tier reachability separate from cluster membership.

### SmartStore cluster interaction

- [SmartStore architecture
  overview](https://help.splunk.com/en/splunk-enterprise/administer/manage-indexers-and-indexer-clusters/10.4/implement-smartstore-to-reduce-local-storage-requirements/smartstore-architecture-overview)
  is the direct source for hot-bucket, remote warm master-copy, and local cache
  roles.
- [Indexer cluster operations and
  SmartStore](https://help.splunk.com/en/splunk-enterprise/administer/manage-indexers-and-indexer-clusters/10.4/how-smartstore-works/indexer-cluster-operations-and-smartstore)
  is the direct source for cluster-operation interactions with SmartStore.
- [Troubleshoot
  SmartStore](https://help.splunk.com/en/splunk-enterprise/administer/manage-indexers-and-indexer-clusters/10.4/manage-smartstore/troubleshoot-smartstore)
  is the direct source for supported SmartStore evidence and troubleshooting
  boundaries. Use only read-only evidence guidance; do not change cache,
  endpoint, credentials, storage, or bucket state.

Route cache sizing, capacity, endpoint design, retention, migration, and storage
choice to `index-and-storage-management-advisor`.

## Splunk Cloud Platform 10.5.2605

Use these anchors only when Cloud release `10.5.2605` and the documented CMC
experience apply. For a different Cloud release or experience, retrieve that
exact release's pages. Public Enterprise procedures do not authorize or prove a
Splunk Cloud backend action.

- [Select the correct documentation version for your
  deployment](https://help.splunk.com/en/splunk-cloud-platform/administer/admin-manual/10.5.2605/monitor-your-splunk-cloud-platform-deployment/introduction-to-the-cloud-monitoring-console/select-the-correct-documentation-version-for-your-deployment)
  is the direct source for binding CMC guidance to the applicable Cloud
  experience and release.
- [Use the health
  dashboard](https://help.splunk.com/en/splunk-cloud-platform/administer/admin-manual/10.5.2605/monitor-your-splunk-cloud-platform-deployment/use-the-health-dashboard)
  is the direct source for customer-visible CMC health evidence. A supplied
  health status is not provider-internal topology, root cause, or proof of every
  peer and bucket.
- [Use the indexing
  dashboards](https://help.splunk.com/en/splunk-cloud-platform/administer/admin-manual/10.5.2605/monitor-your-splunk-cloud-platform-deployment/use-the-indexing-dashboards)
  is the direct source for customer-visible indexing evidence. Route collection
  and dashboard navigation to
  `splunk-health-monitoring-and-diagnostic-collection`.
- [Use the maintenance
  dashboard](https://help.splunk.com/en/splunk-cloud-platform/administer/admin-manual/10.5.2605/monitor-your-splunk-cloud-platform-deployment/use-the-maintenance-dashboard)
  is the direct source for customer-visible maintenance-window evidence. It
  does not prove internal peer state, factor state, fix-up completion, or
  successful recovery.
- [Splunk Cloud Platform Service
  Details](https://help.splunk.com/en/splunk-cloud-platform/get-started/service-terms-and-policies/10.5.2605/information-about-the-service/splunk-cloud-platform-service-details)
  is the direct source for release-specific customer/provider service and
  access boundaries. Pair it with the exact feature page whenever assigning a
  customer-visible check or provider-internal evidence/action. It does not name
  the request's actual owner; require supplied ownership evidence.

For Cloud, separate customer-visible CMC, search, ingest, index, alert, and
maintenance observations from provider-owned cluster topology, filesystem,
services, SmartStore backend, repair, rebalance, restart, rolling work, and
restricted APIs. Route unavailable state through the documented support path
with a sanitized request; never substitute an Enterprise command.

## Point-of-use source check

Before returning any documented conclusion, verify:

1. the URL names the exact applicable product and release or Cloud experience;
2. the page body contains the behavior or evidence field being claimed;
3. factor claims use the specific replication-factor or search-factor page;
4. bundle, rolling, multisite, and SmartStore claims use their topic-specific
   pages rather than a neighboring cluster page;
5. Cloud ownership claims pair the feature page with exact-release Service
   Details when the customer/provider boundary matters;
6. documentation is not presented as runtime proof; and
7. a missing exact source or discriminator blocks only the dependent conclusion
   and is requested explicitly; and
8. Enterprise-only output contains no Cloud source or ownership route, while
   Cloud customer/provider claims use Cloud sources rather than Enterprise
   procedures.
