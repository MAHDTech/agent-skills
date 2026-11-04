# Public report authoring guidance

Use this map to bind each documented claim to the request's exact product,
deployment type, release and topic. Cite the direct content-bearing page beside
the decisive conclusion. A `latest`, unversioned, adjacent-release,
different-product, search-result, sitemap or navigation-only page is
non-decisive. If the release differs from the examples below, retrieve the same
topic for that exact release before answering.

Documentation establishes supported behavior, not a deployment's saved state,
dispatch, execution, results, rendering, transport, receipt, permissions,
acceleration, root cause or acceptance.

## Splunk Cloud Platform 10.5.2605

Use these anchors only when Cloud release `10.5.2605` applies.

- [Create and edit reports](https://help.splunk.com/en/splunk-cloud-platform/create-dashboards-and-reports/reporting-manual/10.5.2605/report-management/create-and-edit-reports)
  documents report creation/editing, saved time behavior and time range picker
  behavior. Use it to describe supported configuration only; this skill never
  creates, edits or runs the report.
- [Schedule reports](https://help.splunk.com/en/splunk-cloud-platform/create-dashboards-and-reports/reporting-manual/10.5.2605/report-management/schedule-reports)
  documents the `schedule_search` capability, scheduled owner context, cadence
  and cron controls, report time range, schedule window and actions. It does not
  prove that a report was due, dispatched, completed or delivered.
- [How time zones are processed by the Splunk platform](https://help.splunk.com/en/splunk-cloud-platform/search/search-manual/10.5.2605/specify-time-ranges/how-time-zones-are-processed-by-the-splunk-platform)
  documents exact-release indexed/search-time time-zone interpretation. It does
  not prove a report's configured schedule timezone, effective bounds or
  completeness boundary.
- [Set report permissions](https://help.splunk.com/en/splunk-cloud-platform/create-dashboards-and-reports/reporting-manual/10.5.2605/report-management/set-report-permissions)
  documents role read/write behavior and owner/user execution behavior. A
  displayed permission does not prove source-data access, effective action
  permission, recipient authorization or runtime execution.
- [Generate PDFs of your reports and dashboards](https://help.splunk.com/en/splunk-cloud-platform/create-dashboards-and-reports/reporting-manual/10.5.2605/report-management/generate-pdfs-of-your-reports-and-dashboards)
  documents on-demand PDF generation and scheduled PDF email delivery as
  distinct uses. It does not prove rendering or recipient receipt.
- [Email notification action](https://help.splunk.com/en/splunk-cloud-platform/alert-and-respond/alerting-manual/10.5.2605/configure-alert-actions/email-notification-action)
  documents configured mail prerequisites, role/capability prerequisites,
  email-domain restrictions, result content, CSV/PDF attachments and empty
  attachment behavior. Route report-action workflow analysis to
  `alerting-and-notable-workflows`; never configure or invoke the action here.
- [Check scheduler activity](https://help.splunk.com/en/splunk-cloud-platform/administer/admin-manual/10.5.2605/monitor-your-splunk-cloud-platform-deployment/use-the-search-dashboards/check-scheduler-activity)
  is the exact-release customer-visible scheduler source. Use only fields the
  page documents and record the visible surface and observation time. It does
  not expose or prove provider-internal scheduler, cluster or service state.
- [Manage search jobs](https://help.splunk.com/en/splunk-cloud-platform/search/search-manual/10.5.2605/manage-jobs/manage-search-jobs)
  documents customer-visible job status, progress and result access. A listed
  job or terminal status does not by itself prove result completeness,
  rendering, action, transport, receipt or business acceptance.
- [Accelerate reports](https://help.splunk.com/en/splunk-cloud-platform/create-dashboards-and-reports/reporting-manual/10.5.2605/report-management/accelerate-reports)
  documents exact-release report-acceleration qualification and summary-range
  behavior. Route applicability, design, build, coverage, completeness and
  troubleshooting to `data-model-and-search-acceleration`.
- [Splunk Cloud Platform Service Details](https://help.splunk.com/en/splunk-cloud-platform/get-started/service-terms-and-policies/10.5.2605/information-about-the-service/splunk-cloud-platform-service-details)
  is the exact-release source for service capabilities and customer/provider
  responsibilities. Pair it with the feature page when assigning a
  customer-visible check or provider-owned evidence/action route. It does not
  identify the request's actual owner.

Do not translate Enterprise file, CLI, service, mail-server, cluster or internal
API procedures into Cloud customer actions. If a decisive report-stage field is
not available from a documented customer-visible surface, describe only the
sanitized field needed and route it to the documented Cloud owner or Splunk
Support.

## Splunk Enterprise 10.4

Use these anchors only when self-managed Enterprise release `10.4` applies.
Retrieve the same topic for another exact Enterprise release rather than using
Cloud or adjacent-version guidance.

- [Create and edit reports](https://help.splunk.com/en/splunk-enterprise/create-dashboards-and-reports/reporting-manual/10.4/report-management/create-and-edit-reports)
  documents report definition and time range picker behavior.
- [Schedule reports](https://help.splunk.com/en/splunk-enterprise/create-dashboards-and-reports/reporting-manual/10.4/report-management/schedule-reports)
  documents scheduling capability, scheduled owner context, cadence/cron,
  report time range, schedule window and actions.
- [How time zones are processed by the Splunk platform](https://help.splunk.com/en/splunk-enterprise/search/search-manual/10.4/specify-time-ranges/how-time-zones-are-processed-by-the-splunk-platform)
  documents indexed/search-time time-zone interpretation for this release.
- [Set report permissions](https://help.splunk.com/en/splunk-enterprise/create-dashboards-and-reports/reporting-manual/10.4/report-management/set-report-permissions)
  documents role read/write and owner/user behavior.
- [Generate PDFs of your reports and dashboards](https://help.splunk.com/en/splunk-enterprise/create-dashboards-and-reports/reporting-manual/10.4/report-management/generate-pdfs-of-your-reports-and-dashboards)
  documents on-demand and scheduled PDF uses for this release.
- [Email notification action](https://help.splunk.com/en/splunk-enterprise/alert-and-respond/alerting-manual/10.4/configure-alert-actions/email-notification-action)
  documents mail configuration, role/capability prerequisites, domain controls,
  result content, CSV/PDF attachments and empty attachment behavior.
- [Manage search jobs](https://help.splunk.com/en/splunk-enterprise/search/search-manual/10.4/manage-jobs/manage-search-jobs)
  documents customer-operated job status, progress and result access for this
  release; deployment readback is still required for a particular job.
- [Accelerate reports](https://help.splunk.com/en/splunk-enterprise/create-dashboards-and-reports/reporting-manual/10.4/report-management/accelerate-reports)
  documents report-acceleration qualification and summary-range behavior.

Self-management can make more saved-definition, scheduler, role, mail, API,
service and topology evidence customer-owned. It does not authorize this skill
to run SPL, call an API, edit configuration or objects, change permissions or
schedules, send output, restart a service or remediate an incident. Require the
actual report, search, governance, action-workflow and platform owners rather
than inferring them from the deployment type.

## Stage-specific citation rules

Bind each claim to the narrowest source:

1. report definition and time range picker -> create/edit page;
2. capability, owner context, cadence, report range or schedule window ->
   schedule page;
3. indexed/search-time time-zone interpretation -> time-zone page, while
   configured schedule timezone remains deployment readback;
4. report read/write or owner/user behavior -> permissions page;
5. on-demand versus scheduled PDF behavior -> PDF page;
6. mail setup, destination restrictions, result body, CSV/PDF attachment or
   empty-attachment behavior -> email-action page;
7. job status, progress or result access -> manage-search-jobs page;
8. Cloud customer-visible scheduler fields -> scheduler-activity page;
9. report-acceleration qualification -> accelerate-reports page; and
10. Cloud customer/provider ownership -> feature page plus Service Details.

The email-action source includes alerting and report-delivery behavior. Cite
only the specific prerequisite or format behavior that applies, retain the
report contract here, and route action-workflow design or diagnosis to
`alerting-and-notable-workflows`.

## Point-of-use check

Before returning a documented conclusion, verify:

- URL product and release match the supplied deployment;
- the page body contains the exact claimed behavior or field;
- documentation is not presented as runtime proof;
- time range, cadence, timezone, schedule window and execution time remain
  distinct;
- configured recipient/domain/format intent is not treated as delivery or
  receipt;
- report permissions are not treated as source-data, action or recipient proof;
- Cloud feature and provider claims use exact-release Cloud sources, with no
  Enterprise procedure substituted for inaccessible state;
- acceleration claims stay with the acceleration sibling; and
- a missing source or discriminator blocks only the dependent conclusion.
