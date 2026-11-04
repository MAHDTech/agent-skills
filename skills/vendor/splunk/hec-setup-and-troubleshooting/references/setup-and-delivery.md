# Documented HEC Setup and Delivery

Use only the branch matching the user's product and version. These links are
evidence anchors; verify current applicability when the user's version differs.

## Setup paths

### Splunk Cloud Platform

Use Splunk Web to create and manage HEC tokens. Confirm the target index already
exists, select only indexes the token may write, choose a default index, and set
source, sourcetype, and host metadata only when the integration requires fixed
values. Confirm deployment status before testing. Cloud HEC uses HTTPS, and
Cloud service constraints restrict global settings, output groups, and general
ACK use; verify the documented support for the specific sender rather than
assuming Enterprise behavior. See [Cloud HEC setup in Splunk
Web](https://help.splunk.com/en/splunk-cloud-platform/get-data-in/get-started-with-getting-data-in/10.4.2604/get-data-with-http-event-collector/set-up-and-use-http-event-collector-in-splunk-web)
and [Splunk Cloud Platform service
details](https://help.splunk.com/en/splunk-cloud-platform/get-started/service-terms-and-policies/9.3.2408/information-about-the-service/splunk-cloud-platform-service-details).

Do not redirect Cloud HEC setup to Enterprise CLI or configuration files. Route
an IP allowlist change to the Cloud administration owner; token and delivery
work remains here.

### Splunk Enterprise

Choose one documented administration surface:

- Splunk Web: enable HEC as needed, create the token, bind allowed/default
  indexes, and set metadata. See [Enterprise HEC setup in Splunk
  Web](https://help.splunk.com/en/splunk-enterprise/get-started/get-data-in/10.4/get-data-with-http-event-collector/set-up-and-use-http-event-collector-in-splunk-web).
- splunkctl: prefer `splunkctl hec list`, `add`, `edit`, `enable`, `disable`,
  and `remove` for supported token administration. Describe the exact command
  discovered from `splunkctl schema --group hec --compact` or
  `splunkctl hec --help`; do not run a mutating command without separate
  explicit authority, and only then use its `splunkctl --yes ...` form. See
  [Enterprise HEC from the
  CLI](https://help.splunk.com/en/splunk-enterprise/get-started/get-data-in/10.4/get-data-with-http-event-collector/set-up-and-use-http-event-collector-from-the-cli).
- Configuration files: describe the relevant global or per-token
  `inputs.conf` settings and, for distributed output, applicable `outputs.conf`
  groups and persistent queues; do not edit files or restart Splunk. Cover SSL,
  ports, queues, allowed/default indexes, and output groups only when material.
  See [Enterprise HEC configuration
  files](https://help.splunk.com/en/splunk-enterprise/get-started/get-data-in/10.4/get-data-with-http-event-collector/set-up-and-use-http-event-collector-with-configuration-files).

For every path, state who owns the admin action, what prerequisites must already
exist, and the expected post-action state. A documented path is not proof that
the user's token is deployed or healthy.

## Endpoint and payload selection

- Use the receiver URL and port documented for the deployment. Splunk Cloud
  Platform HEC uses HTTPS on port 443; Splunk Enterprise commonly starts with
  the configurable HEC port 8088. Do not guess when a proxy or load balancer
  exposes a different approved endpoint. See [Cloud HEC setup in Splunk
  Web](https://help.splunk.com/en/splunk-cloud-platform/get-data-in/get-started-with-getting-data-in/10.4.2604/get-data-with-http-event-collector/set-up-and-use-http-event-collector-in-splunk-web)
  and [Enterprise HEC setup in Splunk
  Web](https://help.splunk.com/en/splunk-enterprise/get-started/get-data-in/10.4/get-data-with-http-event-collector/set-up-and-use-http-event-collector-in-splunk-web).
- Use `/services/collector/event` for a JSON envelope with an `event` value and
  optional `time`, `host`, `source`, `sourcetype`, `index`, and `fields`.
- `splunkctl hec send` covers the event endpoint. Keep raw delivery, explicit
  request-channel handling, health, and ACK polling advisory unless an
  already-configured customer-preferred method safely supports them inside the
  same approval and secret boundaries.
- Keep the HEC token in environment-managed secret material outside the
  transcript. Never paste, echo, log, or embed it in a URL or command example.

See [Format events for
HEC](https://help.splunk.com/en/splunk-enterprise/get-started/get-data-in/10.4/get-data-with-http-event-collector/format-events-for-http-event-collector)
for envelope, raw, metadata, multi-event, authorization, and channel behavior,
and [HEC REST API endpoint
overview](https://help.splunk.com/en/splunk-enterprise/get-data-in/collect-http-event-data/http-event-collector-rest-api-endpoints)
for endpoint families.

## Bounded event test

Have the user substitute a non-production receiver and approved test index. Do
not run the request without separate explicit authority. Prefer `splunkctl`
when it is available:

```sh
command -v splunkctl
splunkctl --yes hec send \
  --event '{"message":"hec-validation-<unique-id>"}' \
  --index '<approved-test-index>' \
  --sourcetype '<test-sourcetype>' \
  --source 'hec-bounded-validation'
```

Use the already configured host and environment-managed HEC token. Capture the
structured response and request timestamp/timezone without capturing the token
or authorization header. If `splunkctl` is unavailable, say so and use an
already-configured customer-preferred HEC method only when it preserves the
same non-production target, approved index, one-event bound, secret handling,
and response capture.

Splunkctl `v0.1.0` requires `--yes` for the HEC delivery and search POSTs. Use
it only after the user approves the exact one-event delivery or read-only
validation query; the confirmation never authorizes token administration or a
wider mutation.

Verify only the unique marker in the approved time window:

```sh
splunkctl --yes search export 'index="<approved-test-index>" sourcetype="<test-sourcetype>" source="hec-bounded-validation" "hec-validation-<unique-id>" earliest=-15m latest=now | table _time host source sourcetype index _raw' --maxout 10 --output json
```

The test is validated only when the response is successful and the matching
event is observed. A successful submission response by itself is not indexed-
event proof; a missing search result by itself does not identify the failing
phase.
