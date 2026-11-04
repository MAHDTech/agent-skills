# Public Search Head Cluster guidance

Use this map to bind each documented claim to the request. Record the product,
deployment type, exact release, topology or Cloud experience when material,
topic, direct content-bearing URL, and the claim it supports. Put the citation
beside the decisive conclusion.

The anchors below are current exact-release examples. If the named release is
different, retrieve the equivalent topic for that exact release before
answering. A `latest`, unversioned, adjacent-release, different-product, search
result, sitemap, or navigation-only page is non-decisive. If product, release,
deployment, topology, or Cloud experience is unknown, keep only the dependent
conclusion blocked and request that discriminator.

Do not mix product branches. An Enterprise-only request uses Enterprise
evidence, Enterprise ownership, and exact-release Enterprise pages only. A
Cloud request uses current exact-release Cloud customer-visible CMC, admin,
maintenance, and service-boundary sources; Enterprise procedures cannot fill a
provider-evidence gap.

Public documentation establishes supported behavior, not deployment health,
member participation, captain stability, election success, Raft cause,
replication completion, KV Store readiness, deployer application, rolling
readiness, search completeness, or recovery.

## Splunk Enterprise 10.4

Use these anchors only when self-managed Splunk Enterprise release `10.4`
applies. Several pages describe commands or mutations that this advisory skill
must never perform or repeat. Cite only the documented behavior and read-only
evidence fields; keep every action with a separately authorized operator and
runbook.

### Architecture, captaincy, elections, and availability

- [Search head clustering
  architecture](https://help.splunk.com/en/splunk-enterprise/administer/distributed-search/10.4/overview-of-search-head-clustering/search-head-clustering-architecture)
  is the direct source for member, captain, dynamic-election, majority,
  scheduling, configuration-replication, and artifact-replication roles. A
  documented election model does not establish the current captain, cause of an
  election, majority state, or successful failover.
- [Use the search head clustering
  dashboard](https://help.splunk.com/en/splunk-enterprise/administer/distributed-search/10.4/troubleshoot-search-head-clustering/use-the-search-head-clustering-dashboard)
  is the direct source for dashboard member, status, captain, and heartbeat
  fields. The page also exposes restart and captain-transfer actions; this skill
  never uses them.
- [Use the Monitoring Console to view SHC status and troubleshoot
  issues](https://help.splunk.com/en/splunk-enterprise/administer/distributed-search/10.4/troubleshoot-search-head-clustering/use-the-monitoring-console-to-view-search-head-cluster-status-and-troubleshoot-issues)
  is the direct source for the Status and Configuration, Configuration
  Replication, Artifact Replication, Scheduler Delegation, and App Deployment
  dashboards. Use their read-only fields with identity and time alignment.
- [Handle Raft
  issues](https://help.splunk.com/en/splunk-enterprise/administer/distributed-search/10.4/troubleshoot-search-head-clustering/handle-raft-issues)
  is the direct source for applicable member-status and `SHCRaftConsensus`
  indicators. The page contains destructive and service-changing procedures.
  Never clean Raft state, bootstrap a captain, stop or start members, or convert
  a historical action into advice.

### Member heartbeats and configuration consistency

The Monitoring Console SHC page above documents heartbeat sent/received fields,
member state from the captain perspective, baseline-consistency comparisons,
unpublished changes, election activity, scheduler delegation, artifact
replication, and app-deployment status. Keep these distinct:

- process, web, management, and user-endpoint reachability;
- SHC registration, heartbeat, and participation;
- captain identity, election state, and Raft evidence;
- configuration baseline and unpublished changes;
- artifact replication and scheduler delegation; and
- observed user availability and result completeness.

A heartbeat or Up state is not proof of KV Store readiness, effective
configuration, every artifact, complete results, or cluster-wide recovery.

### Runtime replication and deployer-managed configuration

- [How configuration changes propagate across the
  SHC](https://help.splunk.com/en/splunk-enterprise/administer/distributed-search/10.4/update-search-head-cluster-members/how-configuration-changes-propagate-across-the-search-head-cluster)
  is the direct source for the automatic runtime-replication versus deployer
  boundary. Use it to classify the supplied change type, not to infer that a
  particular object or setting converged.
- [Use the deployer to distribute apps and configuration
  updates](https://help.splunk.com/en/splunk-enterprise/administer/distributed-search/10.4/update-search-head-cluster-members/use-the-deployer-to-distribute-apps-and-configuration-updates)
  is the direct source for deployer ownership, configuration bundles, staging,
  package and distribution behavior, member delivery, and the App Deployment
  dashboard. It also establishes that the deployer is not a complete source of
  truth for runtime configuration.

For a deployer finding, keep source and ownership, staging and packaging,
validation/readiness, push initiation, distribution and member receipt,
application, documented restart requirement, effective readback, and cluster
impact separate. The deployer page includes commands, filesystem paths,
configuration edits, app deletion behavior, and restart-producing work. This
skill executes or recommends none of them.

Keep these bundle families separate:

- deployer configuration bundles for SHC members;
- runtime configurations replicated among SHC members;
- search knowledge bundles sent from the captain to search peers;
- indexer-cluster manager configuration bundles; and
- deployment-server apps.

### Rolling restart and current health readiness

- [Restart the search head
  cluster](https://help.splunk.com/en/splunk-enterprise/administer/distributed-search/10.4/manage-search-head-clustering/restart-the-search-head-cluster)
  is the direct source for majority behavior, phased rolling restart,
  searchable rolling restart, health checks, detention, captain behavior, and
  search-availability cautions. Use it only to support health prerequisites,
  availability assumptions, and stop criteria.

The restart page describes actions, force options, detention, captain transfer,
and configuration that this skill must not perform, recommend, or authorize.
A documented health check is not deployment evidence. Require current
captain/member, majority, KV, replication, deployer, scheduler, search-impact,
and external-dependency evidence before returning ready for owner review.

Upgrade path, target-release compatibility, maintenance-plan design, backup and
recovery planning, and upgrade go/no-go remain with
`upgrade-planning-and-execution-readiness`.

### KV Store status, replicas, backup, and restore

- [KV Store troubleshooting
  tools](https://help.splunk.com/en/splunk-enterprise/administer/admin-manual/10.4/administer-the-app-key-value-store/kv-store-troubleshooting-tools)
  is the direct source for service-status and replica-status definitions and
  supported read-only evidence surfaces. Use status, role, heartbeat,
  operation-time, version, and sanitized log evidence per member. The page also
  includes resync and other actions; this skill uses none of them.
- [Back up and restore KV
  Store](https://help.splunk.com/en/splunk-enterprise/administer/admin-manual/10.4/administer-the-app-key-value-store/back-up-and-restore-kv-store)
  is the direct source for backup readiness, consistency distinctions,
  archive/target prerequisites, restore overwrite risk, and completion checks.
  Use it only to separate evidence states and route ownership.
- [Back up and restore SHC
  settings](https://help.splunk.com/en/splunk-enterprise/administer/distributed-search/10.4/manage-search-head-clustering/back-up-and-restore-search-head-cluster-settings)
  is the direct source when catastrophic SHC-state and deployer-bundle recovery
  boundaries are material. It explicitly couples several distinct backup sets
  and recommends testing the scheme. It does not prove a supplied backup valid
  or restorable.

Never equate KV `ready` with every member synchronized, every collection or
consumer correct, or backup restoreability. Never equate archive existence with
successful, consistent, compatible, tested, or correct restoration. This skill
does not enter maintenance mode, disable scheduling, back up, restore, resync,
repair, compact, delete, migrate, change read-only mode, manipulate captaincy,
or run a recovery procedure.

### Search availability boundaries

The architecture and Monitoring Console sources above document captain
scheduling, artifact replication, scheduler delegation, and member activity.
Apply them narrowly:

- successful delegation is not proof of successful dispatch;
- successful dispatch is not proof of completion;
- completion is not proof that an artifact remains available from every route;
- an available artifact is not proof of complete results;
- ad hoc availability does not prove scheduled-search or alert continuity; and
- SHC availability does not prove search-peer connectivity, indexed-data
  completeness, federated-provider availability, or healthy SPL.

Route SPL execution to `splunk-search`, one-search tuning to
`search-performance-optimizer`, and indexer-cluster factors/buckets/fix-up to
`indexer-cluster-health-and-troubleshooting`.

## Splunk Cloud Platform 10.5.2605

Use these anchors only when Cloud release `10.5.2605` and the documented CMC
experience apply. For a different Cloud release or experience, retrieve that
exact release's pages. Public Enterprise procedures do not authorize or prove a
Splunk Cloud backend action.

- [Use the Health
  dashboard](https://help.splunk.com/en/splunk-cloud-platform/administer/admin-manual/10.5.2605/monitor-your-splunk-cloud-platform-deployment/use-the-health-dashboard)
  is the direct source for customer-visible CMC health categories, indicators,
  validation criteria, health-check results, `Last updated` values, and detailed
  views. The main dashboard updates every three hours while indicator detail
  pages update at page load, so record the exact surface and timestamp and
  preserve discrepancies. A supplied color or indicator remains a time-scoped
  observation, not internal member topology, captain or Raft state, KV replica
  state, deployer phase, root cause, or recovery proof.
- [Use the Maintenance
  dashboard](https://help.splunk.com/en/splunk-cloud-platform/administer/admin-manual/10.5.2605/monitor-your-splunk-cloud-platform-deployment/use-the-maintenance-dashboard)
  is the direct source for customer-visible upcoming and past maintenance
  windows for Splunk-initiated and customer-requested changes, plus documented
  experience limitations. Record the exact visible entry and time. Maintenance
  intent or elapsed time does not prove member participation, scheduler
  continuity, search availability, or recovery.
- [Splunk Cloud Platform Service
  Details](https://help.splunk.com/en/splunk-cloud-platform/get-started/service-terms-and-policies/10.5.2605/information-about-the-service/splunk-cloud-platform-service-details)
  is the direct source for service capabilities, customer/provider
  responsibilities, CMC context, and supported service boundaries. Pair it with
  the exact feature page whenever assigning a customer-visible check or a
  provider-inaccessible evidence/action route.

For Cloud, separate customer-visible CMC, search, scheduler, maintenance, app,
and permitted configuration observations from provider-owned or restricted
member topology, filesystem, services, captaincy, Raft, KV backend, deployer
operations, restart, resync, repair, and internal APIs. Route inaccessible state
through the documented product-scoped owner or Splunk Support with a sanitized
request; never substitute an Enterprise command.

At every Cloud point of use, cite only the exact page above that contains the
named visible field. Do not cite an adjacent CMC dashboard or generic Cloud page
as decisive and do not invent a hidden SHC equivalent. Always name both the
customer-visible evidence owner and the Splunk Cloud Platform provider/Splunk
Support evidence-and-action owner. Keep the product scope Cloud even if one
owner, topology detail, or provider-only field is unavailable.

## Point-of-use source check

Before returning a documented conclusion, verify:

1. the URL names the exact applicable product, release, and Cloud experience;
2. the page body contains the behavior or evidence field being claimed;
3. captain/election and Raft claims use their topic-specific pages;
4. member, replication, artifact, scheduler, and deployer claims use the exact
   dashboard or propagation source rather than a neighboring cluster page;
5. KV service, replica, backup, and restore claims use their specific sources;
6. rolling-readiness claims use the exact rolling source without turning an
   execution procedure into authorization;
7. Cloud customer/provider claims pair the feature page with exact-release
   Service Details when the boundary matters;
8. Cloud claims name the exact visible field and surface, including timestamp or
   freshness where the page documents it;
9. documentation is not presented as runtime proof;
10. a missing exact source or discriminator blocks only the dependent conclusion;
   and
11. Enterprise-only output contains no Cloud source or Support route, while Cloud
    output does not substitute Enterprise procedures for provider evidence.
