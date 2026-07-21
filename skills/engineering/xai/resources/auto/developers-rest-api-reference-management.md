#### Management API

# Management REST API Overview

The Management API allows you to perform operations on your team programmatically. You
need a [management key](https://console.x.ai/team/default/management-keys) in
order to use this API. The base URL for all endpoints is `https://management-api.x.ai`.

The Management API serves as a dedicated interface to the xAI platform, empowering developers and teams to
programmatically manage their xAI API teams.

For example, users can provision their API key, handle access controls,
and perform team-level operations like creating, listing, updating, or deleting keys and associated access control lists
(ACLs). This API also facilitates oversight of billing aspects, including monitoring prepaid credit balances and usage
deductions, ensuring seamless scalability and cost transparency for Grok model integrations.

To get started, go to [xAI Console](https://console.x.ai). On users page, make sure your xAI account has
`Management Keys` Read + Write permission, and obtain your Management API key on the settings page. If you don't see
any of these options, please ask your team administrator to enable the appropriate permissions.

* [Accounts and Authorization](https://docs.x.ai/developers/rest-api-reference/management/auth)
* [Billing Management](https://docs.x.ai/developers/rest-api-reference/management/billing)
* [Audit Logs](https://docs.x.ai/developers/rest-api-reference/management/audit)