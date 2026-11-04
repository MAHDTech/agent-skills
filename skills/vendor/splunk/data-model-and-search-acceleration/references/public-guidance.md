# Public data-model and acceleration guidance

Use this source map to bind each documented claim to the request. Record the
product, deployment type, exact release, topology or Cloud experience when
material, topic, direct content-bearing URL, and the claim it supports. Put that
citation beside the decisive conclusion. A generic, `latest`, unversioned,
adjacent-release, different-product, or navigation-only page is non-decisive.
If the named release is not represented below, retrieve the same topic for that
exact release before answering. If the product, release, experience, installed
CIM version, or topology is unknown, keep only the dependent conclusion
`decision-blocked` and request that discriminator.

Documentation establishes supported behavior, not deployment configuration,
summary health, semantic correctness, complete results, or performance.

## Splunk Cloud Platform 10.5.2605

Use these anchors only when Cloud release `10.5.2605` applies. For a different
Cloud release, replace every material anchor with that release's page; do not
reuse `10.5.2605` as an adjacent-version proxy.

### Data-model acceleration and summary behavior

- The Cloud `10.5.2605` [Accelerate data
  models](https://help.splunk.com/en/splunk-cloud-platform/manage-knowledge-objects/knowledge-management-manual/10.5.2605/use-data-summaries-to-accelerate-searches/accelerate-data-models)
  page is the direct source for the release's data-model acceleration,
  summarization, range, backfill, source-retention, storage, cardinality, and
  summary-consumption behavior. Use it at the point of each applicable claim.
- The Cloud `10.5.2605`
  [`tstats`](https://help.splunk.com/en/splunk-cloud-platform/spl-search-reference/10.5.2605/search-commands/tstats)
  page is the direct source for interpreting supplied `summariesonly=true` and
  mixed summarized/unsummarized observations. It documents that summary-only
  use returns summarized data and can omit data that has not been summarized.
  Do not use that observation as field/CIM validation or write `tstats` SPL.

### Report acceleration

- The Cloud `10.5.2605` [Accelerate
  reports](https://help.splunk.com/en/splunk-cloud-platform/create-dashboards-and-reports/reporting-manual/10.5.2605/report-management/accelerate-reports)
  page is the direct qualification and summary-range source. It requires the
  existing report definition and search mode; qualification does not prove
  semantic equivalence, complete coverage, or benefit.

### Managed service and provider ownership

- The Cloud `10.5.2605` [Splunk Cloud Platform Service
  Details](https://help.splunk.com/en/splunk-cloud-platform/get-started/service-terms-and-policies/10.5.2605/information-about-the-service/splunk-cloud-platform-service-details)
  page is the direct source for that release's service limits, self-service
  surfaces, maintenance responsibilities, lack of customer infrastructure/CLI
  access, and supported-task or service-ticket boundary. Pair it with the exact
  feature page whenever a conclusion assigns a customer-visible check or a
  provider-internal check. It does not identify the request's actual owner; the
  request still needs named ownership evidence.
- The Cloud `10.5.2605` [Share data model acceleration summaries among search
  heads](https://help.splunk.com/en/splunk-cloud-platform/manage-knowledge-objects/knowledge-management-manual/10.5.2605/use-data-summaries-to-accelerate-searches/share-data-model-acceleration-summaries-among-search-heads)
  page is the direct source for Cloud summary-sharing eligibility and its Splunk
  Support enablement/configuration route. Keep customer-visible model, app,
  writer/reader, semantic, and validation checks separate from the provider's
  inaccessible configuration and backend evidence.

Do not translate an Enterprise file-level procedure into a Cloud action. Broad
ACL, sharing-policy, orphaning, reassignment, packaging, deletion, and lifecycle
governance belongs to `knowledge-object-governance`, even when an acceleration
object is affected.

## Splunk Enterprise 10.4

Use these anchors only when self-managed Enterprise release `10.4` applies. For
another Enterprise release, retrieve that exact release's pages rather than
using `10.4`, a Cloud page, or a generic navigation page.

- Enterprise `10.4` [Accelerate data
  models](https://help.splunk.com/en/splunk-enterprise/manage-knowledge-objects/knowledge-management-manual/10.4/use-data-summaries-to-accelerate-searches/accelerate-data-models)
  is the direct source for data-model acceleration, summary generation, range,
  backfill, retention, storage, cardinality, and summary-consumption behavior.
- Enterprise `10.4`
  [`tstats`](https://help.splunk.com/en/splunk-enterprise/spl-search-reference/10.4/search-commands/tstats)
  is the direct source for interpreting supplied summary-only and mixed
  observations. Equivalent model, dataset, constraints, permissions, source
  population, time window, and result semantics remain required.
- Enterprise `10.4` [Accelerate
  reports](https://help.splunk.com/en/splunk-enterprise/create-dashboards-and-reports/reporting-manual/10.4/report-management/accelerate-reports)
  is the direct source for report qualification, search-mode behavior, summary
  range, and knowledge-object dependency cautions.
- Enterprise `10.4` [Share data model acceleration summaries among search
  heads](https://help.splunk.com/en/splunk-enterprise/manage-knowledge-objects/knowledge-management-manual/10.4/use-data-summaries-to-accelerate-searches/share-data-model-acceleration-summaries-among-search-heads)
  is the direct source for writer/reader roles, app namespace, model-equivalence,
  configuration, and topology requirements in eligible self-managed deployments.

Self-management can make more configuration and platform evidence customer-
owned, but it does not identify the object owner, approver/steward, consumers,
semantic-prerequisite evidence owner, or platform-evidence owner for a request.
Require those owners explicitly and route platform actions outside this skill.

## Fields and Common Information Model 8.6

Use these anchors only when installed CIM version `8.6` applies. If another CIM
or model version is installed, bind the decision to that exact version. Keep
this semantic evidence separate from acceleration summary generation,
persistence, coverage, and consumption health.

- CIM `8.6` [How to use the CIM data model reference
  tables](https://help.splunk.com/en/data-management/common-information-model/8.6/data-models/how-to-use-the-cim-data-model-reference-tables)
  is the direct source for required tags, event datasets, extracted and
  calculated fields, expected values, parent/child inheritance, dataset
  hierarchy, field-value or base-search constraints, and the need to inspect the
  actual data model. Use it to enumerate the exact semantic prerequisites rather
  than citing CIM generically.
- CIM `8.6` [Use the CIM to validate your
  data](https://help.splunk.com/en/data-management/common-information-model/8.6/using-the-common-information-model/use-the-cim-to-validate-your-data)
  is the direct source for representative validation evidence. Record the
  inspected population, expected and observed fields/values/tags/event types,
  applicable hierarchy and constraints, edge cases, result, source, timestamp,
  and semantic-prerequisite evidence owner.
- CIM `8.6` [Use the CIM to normalize data at search
  time](https://help.splunk.com/en/data-management/common-information-model/8.6/using-the-common-information-model/use-the-cim-to-normalize-data-at-search-time)
  is the direct implementation source for field aliases and expected values,
  event types, tags, calculated fields, and lookups. This acceleration skill does
  not perform that work: explicitly route extraction, alias, calculated-field,
  lookup, tag, event-type, or CIM implementation or repair to
  `field-extraction-and-cim-mapping`.

A summary status, size, progress label, summary-only result, or mixed result does
not prove these field/CIM prerequisites. A field/CIM gap blocks semantic and
acceleration-readiness conclusions but does not erase independently supported
summary-health observations.

## Point-of-use source check

Before returning a documented conclusion, verify all of the following:

1. the source URL contains the exact applicable product and release or installed
   CIM version;
2. the page body contains the behavior being claimed;
3. Cloud service/provider claims also cite the exact-release Service Details or
   feature-specific Support route;
4. report acceleration, summary-only/`tstats`, and field/CIM claims use their
   topic-specific pages rather than a neighboring acceleration page; and
5. any missing source or discriminator blocks only the dependent conclusion and
   is requested explicitly.
