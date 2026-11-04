# Public dashboard, report, and alert performance guidance

Use only the source that matches the supplied product and exact release. Cite the
content-bearing page beside the claim it supports. Documentation describes
supported surfaces and behavior; it does not prove what occurred in a deployment.
If product or release is unknown, keep the dependent conclusion blocked.

## Splunk Cloud Platform 10.5.2605

- [Check scheduler activity](https://help.splunk.com/en/splunk-cloud-platform/administer/admin-manual/10.5.2605/monitor-your-splunk-cloud-platform-deployment/use-the-search-dashboards/check-scheduler-activity)
  documents customer-visible panels for scheduled-search delay, completion,
  runtime, errors and warnings, and concurrency. Use only fields actually shown
  for the stated time range and filters.
- [Manage search jobs](https://help.splunk.com/en/splunk-cloud-platform/search/search-manual/10.5.2605/manage-jobs/manage-search-jobs)
  documents job-management visibility, filtering, status, ownership, expiration,
  dashboard-load jobs, report-open jobs, and scheduled jobs.
- [View search job properties](https://help.splunk.com/en/splunk-cloud-platform/search/search-manual/10.5.2605/manage-jobs/view-search-job-properties)
  documents Job Inspector and Job Details fields such as SID, dispatch state,
  progress, execution costs, and search-job properties.
- [Dispatch directory and search artifacts](https://help.splunk.com/en/splunk-cloud-platform/search/search-manual/10.5.2605/manage-jobs/dispatch-directory-and-search-artifacts)
  explains that search jobs use dispatch-directory artifacts and that artifact
  lifetime and access are distinct from job execution. Do not turn the page's
  management procedures into cleanup or retention advice.
- [Use the Workload Management Monitoring dashboard](https://help.splunk.com/en/splunk-cloud-platform/administer/admin-manual/10.5.2605/monitor-your-splunk-cloud-platform-deployment/use-the-workload-management-monitoring-dashboard)
  documents the customer-visible Cloud workload-management monitoring surface.
  Use only the supplied panels and filters; the page does not prove a triggered
  rule or authorize a configuration change.
- [Data source options and properties](https://help.splunk.com/en/splunk-cloud-platform/create-dashboards-and-reports/dashboard-studio/10.5.2605/use-data-sources/data-source-options-and-properties)
  documents Dashboard Studio data-source properties, including refresh interval
  and delay-versus-interval refresh timing.
- [Chain searches together with a base search and chain searches](https://help.splunk.com/en/splunk-cloud-platform/create-dashboards-and-reports/dashboard-studio/10.5.2605/use-data-sources/chain-searches-together-with-a-base-search-and-chain-searches)
  documents base and chain-search relationships and inheritance of refresh
  properties. It does not prove reuse, correctness, or performance for a given
  dashboard.
- [Manage dashboard resource consumption](https://help.splunk.com/en/splunk-cloud-platform/create-dashboards-and-reports/dashboard-studio/10.5.2605/manage-dashboards/manage-dashboard-resource-consumption)
  establishes that dashboard and auto-refresh search activity can affect resource
  consumption. Use it only to frame evidence collection; this skill never applies
  the management actions on that page.
- [Schedule reports](https://help.splunk.com/en/splunk-cloud-platform/create-dashboards-and-reports/reporting-manual/10.5.2605/report-management/schedule-reports)
  distinguishes cadence, report time range, run-as-owner context, priority, and
  schedule window. Configured intent is not dispatch or completion evidence.
- [Use cron expressions for alert scheduling](https://help.splunk.com/en/splunk-cloud-platform/alert-and-respond/alerting-manual/10.5.2605/create-alerts/use-cron-expressions-for-alert-scheduling)
  documents the Cloud timezone behavior and cron/time-range relationship. Read
  back the effective schedule and timezone; do not infer them from the page.
- [Alert scheduling tips](https://help.splunk.com/en/splunk-cloud-platform/alert-and-respond/alerting-manual/10.5.2605/create-alerts/alert-scheduling-tips)
  documents scheduling overlap, delayed-data considerations, and concurrent
  scheduled-search priority. Route alert schedule or semantics changes to the
  alert-workflow owner.
- [Use the Health dashboard](https://help.splunk.com/en/splunk-cloud-platform/administer/admin-manual/10.5.2605/monitor-your-splunk-cloud-platform-deployment/use-the-health-dashboard)
  documents customer-visible health indicators and freshness. A color or cleared
  indicator does not prove object recovery or root cause.
- [Use the Maintenance dashboard](https://help.splunk.com/en/splunk-cloud-platform/administer/admin-manual/10.5.2605/monitor-your-splunk-cloud-platform-deployment/use-the-maintenance-dashboard)
  documents customer-visible maintenance windows. Temporal overlap is context,
  not causal proof.
- [Splunk Cloud Platform Service Details](https://help.splunk.com/en/splunk-cloud-platform/get-started/service-terms-and-policies/10.5.2605/information-about-the-service/splunk-cloud-platform-service-details)
  documents customer-visible monitoring and service-limit boundaries, including
  scheduler and concurrent-search context. Use it to separate customer evidence
  from provider-owned service state, not to infer hidden capacity or action.

## Splunk Enterprise 10.4

- [Search: Scheduler Activity](https://help.splunk.com/en/splunk-enterprise/administer/monitor/10.4/monitoring-console-dashboard-reference/search-scheduler-activity)
  documents skip ratio, execution latency, skipped-report reasons, and scheduler
  failures associated with concurrency or workload.
- [Manage search jobs](https://help.splunk.com/en/splunk-enterprise/search/search-manual/10.4/manage-jobs/manage-search-jobs)
  and [View search job properties](https://help.splunk.com/en/splunk-enterprise/search/search-manual/10.4/manage-jobs/view-search-job-properties)
  document Enterprise job-management and job-detail evidence.
- [Dispatch directory and search artifacts](https://help.splunk.com/en/splunk-enterprise/search/search-manual/10.4/manage-jobs/dispatch-directory-and-search-artifacts)
  documents artifact behavior. This skill does not delete artifacts or alter
  lifetime, disk, or configuration settings.
- [Monitor workload management using the Monitoring Console](https://help.splunk.com/en/splunk-enterprise/administer/manage-workloads/10.4/monitor-workload-management/monitor-workload-management-using-the-monitoring-console)
  is the Enterprise workload-monitoring anchor. Use its matching readback rather
  than an adjacent Search Activity page for a decisive workload claim.
- [Data source options and properties](https://help.splunk.com/en/splunk-enterprise/create-dashboards-and-reports/dashboard-studio/10.4/use-data-sources/data-source-options-and-properties),
  [Chain searches together with a base search and chain searches](https://help.splunk.com/en/splunk-enterprise/create-dashboards-and-reports/dashboard-studio/10.4/use-data-sources/chain-searches-together-with-a-base-search-and-chain-searches),
  and [Manage dashboard resource consumption](https://help.splunk.com/en/splunk-enterprise/create-dashboards-and-reports/dashboard-studio/10.4/manage-dashboards/manage-dashboard-resource-consumption)
  provide the matching Enterprise Dashboard Studio anchors.
- [Schedule reports](https://help.splunk.com/en/splunk-enterprise/create-dashboards-and-reports/reporting-manual/10.4/report-management/schedule-reports),
  [Use cron expressions for alert scheduling](https://help.splunk.com/en/splunk-enterprise/alert-and-respond/alerting-manual/10.4/create-alerts/use-cron-expressions-for-alert-scheduling),
  and [Alert scheduling tips](https://help.splunk.com/en/splunk-enterprise/alert-and-respond/alerting-manual/10.4/create-alerts/alert-scheduling-tips)
  are the matching Enterprise cadence, schedule-window, timezone, time-range,
  delayed-data, and overlap anchors. They do not establish effective local
  settings or execution.

## Evidence discipline

Use a job, scheduler, dashboard, report, alert, health, workload, audit, or provider
surface only for the fields it directly establishes. Keep configuration intent,
current readback, historical record, provider statement, and observed user impact
separate. Never substitute a cited page for object identity, effective settings,
runtime evidence, a comparable observation, owner acknowledgement, or recovery.
Never use a Cloud page for an Enterprise claim, an Enterprise page for a Cloud
claim, or an adjacent or unlisted Search Activity page as decisive evidence.
Bind the matching content-bearing source beside each decisive claim. Public
documentation is supported-behavior context, never local proof.
