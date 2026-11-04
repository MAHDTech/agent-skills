# Public Splunk ingest guidance

Use these public sources at point of use. Confirm that the page applies to the
user's exact product and version; do not transplant a configuration or limit
from another release. These links support documented behavior, not claims about
a particular environment.

## Pipeline and configuration evidence

Splunk documents the data pipeline as input, parsing, indexing, and search, with
the input tier handling input, the indexing tier handling parsing and indexing,
and the search tier handling search. Map evidence to the actual component roles
before deciding where a setting belongs: [How data moves through Splunk
deployments: the data pipeline](https://help.splunk.com/en/splunk-enterprise/administer/distributed-deployment-manual/10.4/overview-of-splunk-enterprise-distributed-deployments/how-data-moves-through-splunk-deployments-the-data-pipeline).

Use effective configuration plus source provenance instead of treating one file
as complete state. Splunk's `btool` guidance documents listing merged
configuration and `--debug` source paths, while configuration precedence
explains why several files can contribute to the effective result: [Use btool
to troubleshoot configurations](https://help.splunk.com/en/splunk-enterprise/administer/troubleshoot/10.4/first-steps/use-btool-to-troubleshoot-configurations) and [Configuration file
precedence](https://help.splunk.com/en/splunk-enterprise/administer/admin-manual/10.4/administer-splunk-enterprise-with-configuration-files/configuration-file-precedence).

The frozen qualification contract also binds decisive merged-configuration
claims to the exact [Splunk Enterprise 9.4 btool
source](https://help.splunk.com/en/splunk-enterprise/administer/troubleshoot/9.4/first-steps/use-btool-to-troubleshoot-configurations).
It documents that `btool` simulates configuration-file merging and reports
on-disk effective configuration, which can differ from configuration currently
loaded in memory when a restart is required. Use it to interpret supplied
effective-state bundles, not to claim runtime activation or authorize a restart.

The configuration-file references define supported settings; interpret only the
stanzas material to the path: [inputs.conf](https://help.splunk.com/en/splunk-enterprise/administer/admin-manual/10.4/configuration-file-reference/10.4.2-configuration-file-reference/inputs.conf), [outputs.conf](https://help.splunk.com/en/splunk-enterprise/administer/admin-manual/10.4/configuration-file-reference/10.4.2-configuration-file-reference/outputs.conf), [props.conf](https://help.splunk.com/en/splunk-enterprise/administer/admin-manual/10.4/configuration-file-reference/10.4.2-configuration-file-reference/props.conf), and [transforms.conf](https://help.splunk.com/en/splunk-enterprise/administer/admin-manual/10.4/configuration-file-reference/10.4.2-configuration-file-reference/transforms.conf).

## Forwarder transport and inputs

Splunk's connection troubleshooting page documents receiver-port checks,
receiver queue blocking, forwarder load-balancing/failover behavior, and common
management-port versus receiving-port confusion. These are candidate checks,
not proof of a specific cause: [Troubleshoot forwarder/receiver
connection](https://help.splunk.com/en/splunk-enterprise/forward-and-process-data/forwarding-and-receiving-data/10.4/troubleshoot-forwarding/troubleshoot-forwarderreceiver-connection).

For file and directory inputs, inspect the effective monitor stanza, enabled
state, whitelist/blacklist behavior, metadata, and file identity/tracking
settings. Splunk documents that CRC behavior is used to avoid indexing the same
file twice and also describes the cases where files with matching initial
content need careful treatment; therefore CRC or tracking changes carry
duplicate risk and are not read-only diagnostics: [Monitor files and
directories with inputs.conf](https://help.splunk.com/en/splunk-enterprise/get-started/get-data-in/10.4/get-data-from-files-and-directories/monitor-files-and-directories-with-inputs.conf).

Deployment-server receipt is a distinct state surface from effective client
configuration or ingest success. Use the fleet sibling for policy and
assignment, and cite [View configurations installed on your
forwarders](https://help.splunk.com/en/splunk-enterprise/administer/update-your-deployment/10.4/manage-the-agent-management/view-configurations-installed-on-your-forwarders)
when comparing those surfaces.

## Parsing, timestamps, and routing

Splunk documents event-boundary processing as line breaking followed by line
merging, controlled by settings such as `LINE_BREAKER` and
`SHOULD_LINEMERGE`. Apply the setting at the component that performs parsing and
validate against representative raw events: [Configure event line
breaking](https://help.splunk.com/en/splunk-enterprise/get-started/get-data-in/10.4/configure-event-processing/configure-event-line-breaking).

Timestamp recognition can depend on `TIME_PREFIX`,
`MAX_TIMESTAMP_LOOKAHEAD`, `TIME_FORMAT`, timezone, and related settings.
Compare raw event time, `_time`, `_indextime`, and the effective stanza before
calling late data a transport delay: [Configure timestamp
recognition](https://help.splunk.com/en/splunk-enterprise/get-started/get-data-in/10.4/configure-timestamps/configure-timestamp-recognition).

For event-level routing or filtering, Splunk documents `props.conf` plus
`transforms.conf` on a parsing component. Heavy forwarders can perform
event-level filtering/routing, while Universal Forwarders are limited to
input-, host-, source-, or sourcetype-based routes except for documented
structured-data behavior. Routing to `nullQueue` discards data, so never propose
it as an incident diagnostic: [Route and filter
data](https://help.splunk.com/en/splunk-enterprise/forward-and-process-data/forwarding-and-receiving-data/10.4/perform-advanced-configuration/route-and-filter-data).

The frozen qualification contract binds Heavy Forwarder routing limits to the
exact [Splunk Enterprise 9.4 route and filter data
source](https://help.splunk.com/en/splunk-enterprise/forward-and-process-data/forwarding-and-receiving-data/9.4/perform-advanced-configuration/route-and-filter-data)
and finite buffering behavior to [Use persistent queues to help prevent data
loss](https://help.splunk.com/en/splunk-enterprise/get-started/get-data-in/9.4/improve-the-data-input-process/use-persistent-queues-to-help-prevent-data-loss).
Use these at point of use when a Heavy Forwarder route comparison or bounded
buffer is decisive. Neither rerouting nor configured buffering proves source
delivery, add-on checkpoint correctness, or complete indexing.

Treat `host`, `source`, and `sourcetype` as indexed/default metadata and verify
the observed values rather than searching only the expected value: [About
default fields](https://help.splunk.com/en/splunk-enterprise/get-started/get-data-in/10.4/configure-indexed-field-extraction/about-default-fields-host-source-sourcetype-and-more).
Per-event sourcetype overrides are parsing-time behavior and require the
relevant transform at the parsing tier: [Override source types on a per-event
basis](https://help.splunk.com/en/splunk-enterprise/get-started/get-data-in/10.4/configure-source-types/override-source-types-on-a-per-event-basis).

## Queues, delay, and clustered receivers

Splunk's indexing-performance triage starts with indexing status, indexing rate,
and queue fill pattern and distinguishes normal, saturated, throttled, and
blocked states. Use a bounded trend and adjacent-stage evidence; one full queue
locates pressure but does not identify its cause: [Identify and triage indexing
performance problems](https://help.splunk.com/en/splunk-enterprise/administer/troubleshoot/10.4/data-acquisition-problems/identify-and-triage-indexing-performance-problems).

For a blocked `splunktcp` receiver, cite the lifecycle behavior exactly where it
supports a finding. Splunk documents that after the receiver cannot insert data
into its configured queue for `stopAcceptorAfterQBlock`, it closes the receiving
port to new connections; after the queue unblocks and processing can continue,
it starts listening again. Connected forwarders detect the closed port during
test connections, and load-balanced forwarders can move to an eligible
receiver. The troubleshooting page
also names the corresponding blocked and unblocked listener messages. A broken
connection alone therefore does not establish this queue lifecycle: [inputs.conf
`stopAcceptorAfterQBlock`](https://help.splunk.com/en/splunk-enterprise/administer/admin-manual/10.4/configuration-file-reference/10.4.2-configuration-file-reference/inputs.conf)
and [Troubleshoot forwarder/receiver
connection](https://help.splunk.com/en/splunk-enterprise/forward-and-process-data/forwarding-and-receiving-data/10.4/troubleshoot-forwarding/troubleshoot-forwarderreceiver-connection).

Persistent queues are bounded input buffers, not acknowledgement or end-to-end
delivery evidence. The `inputs.conf` reference defines `persistentQueueSize` as
the maximum persistent queue-file size, says it can help prevent loss of
transient data, requires it to exceed the applicable in-memory queue size when
enabled, and defaults it to zero. Cite that exact-version reference when this
behavior is decisive. A configured maximum does not establish current
occupancy, the identities retained, successful drain, or indexing: [inputs.conf
`persistentQueueSize`](https://help.splunk.com/en/splunk-enterprise/administer/admin-manual/10.4/configuration-file-reference/10.4.2-configuration-file-reference/inputs.conf).

Measure recovery over a declared cadence and time window: queue occupancy must
decrease across consecutive samples toward its prior baseline, blocked-output
or listener evidence must clear, and bounded indexed counts and `_indextime`
must advance for a stable selector. Stop and escalate when the receiver does not
resume listening after an observed unblock, the queue is flat or rising across
that window, a finite persistent queue approaches or reaches its observed
limit, or source-to-index accounting cannot exclude a gap. In that last case,
report possible loss as unresolved rather than treating finite buffering as
proof of delivery.

Indexer discovery lets forwarders poll a cluster manager for available peer
receivers and load balance across the returned set. In multisite deployments,
the configured site scope matters; discovery does not prove that an unrelated
static output group or local route follows the same failover behavior: [Use
indexer discovery to connect forwarders to peer
nodes](https://help.splunk.com/en/splunk-enterprise/administer/manage-indexers-and-indexer-clusters/10.4/get-data-into-the-indexer-cluster/use-indexer-discovery-to-connect-forwarders-to-peer-nodes).

## HEC and Splunk Cloud boundaries

For a HEC path, interpret the sanitized HTTP status together with its response
body, token/index authority, health, queues, ACK state when used, and the
indexed-event search. Splunk documents response classes and queue/ACK health
signals; a healthy endpoint or accepted request is not by itself end-to-end
indexing evidence: [Troubleshoot HTTP Event
Collector](https://help.splunk.com/en/splunk-enterprise/get-started/get-data-in/10.4/get-data-with-http-event-collector/troubleshoot-http-event-collector).
Use `hec-setup-and-troubleshooting` for HEC-only setup, token administration,
endpoint formatting, or ACK design.

Splunk Cloud Platform documents customer self-service ingest and Cloud
Monitoring Console capabilities. Keep diagnosis on documented customer-visible
surfaces; when the next step requires managed receiver/indexer internals,
service-side configuration, restart, repair, or queue tuning, preserve the
boundary and hand off to Splunk Support or the documented Cloud owner: [Splunk
Cloud Platform Service
Details](https://help.splunk.com/en/splunk-cloud-platform/get-started/service-terms-and-policies/10.5.2605/information-about-the-service/splunk-cloud-platform-service-details).

For fallback-index or cross-tier index-visibility diagnosis, cite the exact
frozen [Cloud index-management
surface](https://help.splunk.com/en/splunk-cloud-platform/administer/admin-manual/10.4.2604/manage-your-indexes-and-data-in-splunk-cloud-platform/manage-indexes-on-splunk-cloud-platform-classic-experience).
It supports customer-visible index verification only. Search-tier visibility
does not prove that the ingest tier received the index definition, and this
documentation never proves propagation or live provider state.

For broad blocked forwarder or HEC ingestion, blocked indexing, or broad login
failure, use Splunk's exact [potential Cloud service-failure reporting
guidance](https://help.splunk.com/en/splunk-cloud-platform/administer/recover-from-a-disaster/10.4.2604/recover-from-a-disaster/report-a-potential-splunk-cloud-platform-service-failure).
Those are indicators to confirm before immediate provider escalation; they do
not establish provider cause. Preserve a sanitized severity, impact window,
customer-visible evidence, evidence owner, and next-update trigger in the
handoff, and stop further customer-side discrimination when it could delay the
incident route.
