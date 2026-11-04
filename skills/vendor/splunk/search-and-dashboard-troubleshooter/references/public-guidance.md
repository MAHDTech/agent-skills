# Public functional-diagnosis guidance

Use these direct public Splunk sources only for documented expectations that
match the supplied product and version. Cite the direct page beside the claim at
its point of use. Documentation does not prove object identity, deployment state,
root cause, provider receipt, certificate trust, or recovery.

## Product and version boundary

This reference deliberately binds the two current supported contexts separately:

- **Splunk Cloud Platform 10.5.2605:** use only links whose path contains
  `/splunk-cloud-platform/` and `/10.5.2605/` for Cloud claims.
- **Splunk Enterprise 10.4:** use only links whose path contains
  `/splunk-enterprise/` and `/10.4/` for Enterprise claims.

Do not substitute Splunk Enterprise 9.4, a generic or `latest` page, or one
product's procedure for the other. If the supplied version is different, retrieve
an exact matching current page or keep the documented expectation conditional or
`unknown`. Keep every `[documented]` expectation separate from `[supplied]` and
`[observed]` deployment evidence.

## Search jobs

At the job-properties stage, the Search Job Inspector and Job Details dashboard
can provide bounded job facts, messages, costs, counts, and indexer metrics. Job
access also depends on an existing artifact and ownership permissions. Use only
privacy-safe job IDs when necessary; never request raw `search.log`, raw SPL, or
private topology. Cite the matching page:

- [Cloud 10.5.2605: View search job properties](https://help.splunk.com/en/splunk-cloud-platform/search/search-manual/10.5.2605/manage-jobs/view-search-job-properties)
- [Enterprise 10.4: View search job properties](https://help.splunk.com/en/splunk-enterprise/search/search-manual/10.4/manage-jobs/view-search-job-properties)

A documented job surface does not prove that a particular job ran, completed,
returned complete results, or belongs to the affected object.

## Time ranges and default `_time`

At the time stage, bind the effective time range, timezone, event arrival window,
and object context. Both exact product pages document that indexed event time is
stored in `_time` as Unix time and rendered for the UI, while the UI applies a
default range when a search is created. Do not infer the affected object's range
or timestamp quality from those defaults:

- [Cloud 10.5.2605: About searching with time](https://help.splunk.com/en/splunk-cloud-platform/search/search-manual/10.5.2605/specify-time-ranges/about-searching-with-time)
- [Cloud 10.5.2605: Use default fields](https://help.splunk.com/en/splunk-cloud-platform/manage-knowledge-objects/knowledge-management-manual/10.5.2605/fields-and-field-extractions/use-default-fields)
- [Enterprise 10.4: About searching with time](https://help.splunk.com/en/splunk-enterprise/search/search-manual/10.4/specify-time-ranges/about-searching-with-time)
- [Enterprise 10.4: Use default fields](https://help.splunk.com/en/splunk-enterprise/manage-knowledge-objects/knowledge-management-manual/10.4/fields-and-field-extractions/use-default-fields)

If the exact UI default matters, the matching tutorials document **Last 24
hours** for their Search app example. Treat that as a documented UI default, not
as proof of the saved search, report, alert, dashboard, scheduled job, or API
range:

- [Cloud 10.5.2605: Specifying time ranges](https://help.splunk.com/en/splunk-cloud-platform/search/search-tutorial/10.5.2605/part-3-using-the-splunk-search-app/specifying-time-ranges)
- [Enterprise 10.4: Specifying time ranges](https://help.splunk.com/en/splunk-enterprise/search/search-tutorial/10.4/part-3-using-the-splunk-search-app/specifying-time-ranges)

## Permissions, knowledge objects, and fields

At the visibility and permission stages, bind object sharing, app context,
ownership, run-as identity, role/capability evidence, and the exact observed
surface. New knowledge objects can begin private, and permitted sharing or edits
depend on owner and role. Cite the matching permissions page:

- [Cloud 10.5.2605: Manage knowledge object permissions](https://help.splunk.com/en/splunk-cloud-platform/manage-knowledge-objects/knowledge-management-manual/10.5.2605/get-started-with-knowledge-objects/manage-knowledge-object-permissions)
- [Enterprise 10.4: Manage knowledge object permissions](https://help.splunk.com/en/splunk-enterprise/manage-knowledge-objects/knowledge-management-manual/10.4/get-started-with-knowledge-objects/manage-knowledge-object-permissions)

At the dependency stage, search-time field processing has a documented sequence.
Use the sequence only to order extraction, alias, lookup, event-type/tag, and
calculated-field checks; require object-specific evidence before attribution:

- [Cloud 10.5.2605: The sequence of search-time operations](https://help.splunk.com/en/splunk-cloud-platform/manage-knowledge-objects/knowledge-management-manual/10.5.2605/get-started-with-knowledge-objects/the-sequence-of-search-time-operations)
- [Enterprise 10.4: The sequence of search-time operations](https://help.splunk.com/en/splunk-enterprise/manage-knowledge-objects/knowledge-management-manual/10.4/get-started-with-knowledge-objects/the-sequence-of-search-time-operations)

Do not convert one-user visibility, administrator success, or a dependency name
into a permissions or field root cause. Ownership or implementation changes route
to the exact adjacent catalog role and remain outside this skill.

## Dashboard chain

At the Dashboard Studio stage, data sources feed visualizations and chain searches
depend on a base search. Keep input/token, base, chain/post-process, data-source,
refresh, visualization, and render stages separate:

- [Cloud 10.5.2605: Use data sources](https://help.splunk.com/en/splunk-cloud-platform/create-dashboards-and-reports/dashboard-studio/10.5.2605/use-data-sources)
- [Cloud 10.5.2605: Chain searches](https://help.splunk.com/en/splunk-cloud-platform/create-dashboards-and-reports/dashboard-studio/10.5.2605/use-data-sources/chain-searches-together-with-a-base-search-and-chain-searches)
- [Cloud 10.5.2605: Inputs](https://help.splunk.com/en/splunk-cloud-platform/create-dashboards-and-reports/dashboard-studio/10.5.2605/make-dashboards-dynamic-and-interactive/inputs)
- [Enterprise 10.4: Use data sources](https://help.splunk.com/en/splunk-enterprise/create-dashboards-and-reports/dashboard-studio/10.4/use-data-sources)
- [Enterprise 10.4: Chain searches](https://help.splunk.com/en/splunk-enterprise/create-dashboards-and-reports/dashboard-studio/10.4/use-data-sources/chain-searches-together-with-a-base-search-and-chain-searches)
- [Enterprise 10.4: Inputs](https://help.splunk.com/en/splunk-enterprise/create-dashboards-and-reports/dashboard-studio/10.4/make-dashboards-dynamic-and-interactive/inputs)

A blank or wrong panel does not by itself prove bad SPL, a token defect, or a
rendering defect.

## Reports and alerts

At the report stage, a report is a saved search. Keep definition/revision,
dispatch, results, render/export, schedule, action, transport, and receipt as
separate evidence stages:

- [Cloud 10.5.2605: Create and edit reports](https://help.splunk.com/en/splunk-cloud-platform/create-dashboards-and-reports/reporting-manual/10.5.2605/report-management/create-and-edit-reports)
- [Enterprise 10.4: Create and edit reports](https://help.splunk.com/en/splunk-enterprise/create-dashboards-and-reports/reporting-manual/10.4/report-management/create-and-edit-reports)

At the alert stage, keep search results, trigger conditions, throttling,
alert-action execution, transport, recipient/provider receipt, and response
record separate:

- [Cloud 10.5.2605: The alerting workflow](https://help.splunk.com/en/splunk-cloud-platform/alert-and-respond/alerting-manual/10.5.2605/alerting-overview/the-alerting-workflow)
- [Cloud 10.5.2605: Configure trigger conditions](https://help.splunk.com/en/splunk-cloud-platform/alert-and-respond/alerting-manual/10.5.2605/manage-alert-trigger-conditions-and-throttling/configure-alert-trigger-conditions)
- [Cloud 10.5.2605: Throttle alerts](https://help.splunk.com/en/splunk-cloud-platform/alert-and-respond/alerting-manual/10.5.2605/manage-alert-trigger-conditions-and-throttling/throttle-alerts)
- [Cloud 10.5.2605: Monitor triggered alerts](https://help.splunk.com/en/splunk-cloud-platform/alert-and-respond/alerting-manual/10.5.2605/configure-alert-actions/monitor-triggered-alerts)
- [Enterprise 10.4: The alerting workflow](https://help.splunk.com/en/splunk-enterprise/alert-and-respond/alerting-manual/10.4/alerting-overview/the-alerting-workflow)
- [Enterprise 10.4: Configure trigger conditions](https://help.splunk.com/en/splunk-enterprise/alert-and-respond/alerting-manual/10.4/manage-alert-trigger-conditions-and-throttling/configure-alert-trigger-conditions)
- [Enterprise 10.4: Throttle alerts](https://help.splunk.com/en/splunk-enterprise/alert-and-respond/alerting-manual/10.4/manage-alert-trigger-conditions-and-throttling/throttle-alerts)
- [Enterprise 10.4: Monitor triggered alerts](https://help.splunk.com/en/splunk-enterprise/alert-and-respond/alerting-manual/10.4/configure-alert-actions/monitor-triggered-alerts)

A triggered-alert record proves only that bounded stage, not every action result or
external receipt.

## PDF, certificate, and provider boundaries

At the PDF stage, keep search/report/dashboard result, render, attachment,
action, transport, and recipient/provider receipt separate. Integrated PDF
behavior includes product-documented time handling and limitations; the exact
pages do not prove that a particular PDF rendered or was delivered:

- [Cloud 10.5.2605: Generate PDFs of reports and dashboards](https://help.splunk.com/en/splunk-cloud-platform/create-dashboards-and-reports/reporting-manual/10.5.2605/report-management/generate-pdfs-of-your-reports-and-dashboards)
- [Enterprise 10.4: Generate PDFs of reports and dashboards](https://help.splunk.com/en/splunk-enterprise/create-dashboards-and-reports/reporting-manual/10.4/report-management/generate-pdfs-of-your-reports-and-dashboards)

At an email, webhook, certificate-trust, or external-provider boundary, pair the
customer-visible Splunk state with only a sanitized provider-visible receipt or
reference, timestamp, or redacted issuer/trust/result status. Never request a
private endpoint/hostname, certificate subject/SAN, credentials, or provider
internals. Cloud and Enterprise expose different administration boundaries:

- Cloud 10.5.2605 documents implementation-managed mail server settings and a
  Splunk Web webhook allow list:
  [Email notification action](https://help.splunk.com/en/splunk-cloud-platform/alert-and-respond/alerting-manual/10.5.2605/configure-alert-actions/email-notification-action) and
  [Configure webhook allow list using Splunk Web](https://help.splunk.com/en/splunk-cloud-platform/administer/admin-manual/10.5.2605/configure-your-splunk-cloud-platform-deployment/configure-webhook-allow-list-using-splunk-web).
- Enterprise 10.4 documents deployment-admin mail settings and a configuration
  boundary for webhook allow lists. This skill does not request or edit that
  configuration:
  [Email notification action](https://help.splunk.com/en/splunk-enterprise/alert-and-respond/alerting-manual/10.4/configure-alert-actions/email-notification-action) and
  [Configure webhook allow list](https://help.splunk.com/en/splunk-enterprise/alert-and-respond/alerting-manual/10.4/manage-alert-and-alert-action-permissions/configure-webhook-allow-list).

Assign customer-visible evidence only to the evidenced Cloud customer-visible
evidence owner. Assign provider- or Support-restricted state only to the evidenced
provider/Support restricted-state owner. If either owner or side of the boundary
is not supplied, keep it `unknown`; do not fabricate a generic security or
identity owner.

## Evidence boundary

If an exact page is unavailable or does not match the supplied product and
version, mark the documented expectation `unknown` or conditional. Do not replace
it with another deployment's procedure, private Support material, raw
configuration-file advice for Splunk Cloud Platform, or a historical resolution.
Documentation can define an expected surface or behavior, but cannot set a local
stage to `supported` or `failed` and cannot close the issue.
