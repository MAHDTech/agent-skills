#### Management API

# Billing Management

***

## GET /v1/billing/teams/\{team\_id}/billing-info

Get billing information of the team with given team ID.

### Path Parameters

* `team_id` (string, required) — Team ID of the team.

### Response Body

* `billingInfo` (object) — Billing info.

  * `name` (string) — The customer's full name or business name.

  * `address` (object) — Billing address.

    * `line1` (string) — Address line 1 (e.g., street, PO Box, or company name).

    * `line2` (string) — Address line 2 (e.g., apartment, suite, unit, or building).

    * `city` (string) — City, district, suburb, town, or village.

    * `country` (string) — Two-letter country code (\[ISO 3166-1 alpha-2]\(https://en.wikipedia.org/wiki/ISO\_3166-1\_alpha-2)).

    * `postalCode` (string) — ZIP or postal code.

    * `state` (string) — State, county, province, or region.

  * `email` (string) — The customer's email.

  * `taxIdType` (string)

  * `taxNumber` (string)

\*\*Response example:\*\*

```json
{
  "billingInfo": {
    "name": "Acme Inc.",
    "address": {
      "line1": "123 Main St.",
      "line2": "",
      "city": "New York",
      "country": "US",
      "postalCode": "12345",
      "state": "New York"
    },
    "email": "foo@example.com",
    "taxIdType": "us_ein",
    "taxNumber": "12-3456789"
  }
}
```

***

## POST /v1/billing/teams/\{team\_id}/billing-info

Set billing information of a team.

### Path Parameters

* `team_id` (string, required) — Team ID of the team.

### Request Body

* `billingInfo` (object) — Billing info.

  * `name` (string) — The customer's full name or business name.

  * `address` (object) — Billing address.

    * `line1` (string) — Address line 1 (e.g., street, PO Box, or company name).

    * `line2` (string) — Address line 2 (e.g., apartment, suite, unit, or building).

    * `city` (string) — City, district, suburb, town, or village.

    * `country` (string) — Two-letter country code (\[ISO 3166-1 alpha-2]\(https://en.wikipedia.org/wiki/ISO\_3166-1\_alpha-2)).

    * `postalCode` (string) — ZIP or postal code.

    * `state` (string) — State, county, province, or region.

  * `email` (string) — The customer's email.

  * `taxIdType` (string)

  * `taxNumber` (string)

\*\*Request example:\*\*

```json
{
  "billingInfo": {
    "name": "Acme Inc.",
    "address": {
      "line1": "123 Main St.",
      "line2": "",
      "city": "New York",
      "country": "US",
      "postalCode": "12345",
      "state": "New York"
    },
    "email": "foo@example.com",
    "taxIdType": "us_ein",
    "taxNumber": "12-3456789"
  }
}
```

\*\*Response example:\*\*

```json
{}
```

***

## GET /v1/billing/teams/\{team\_id}/invoices

List invoices that belong to a team.

### Path Parameters

* `team_id` (string, required) — Team ID of the team.

### Query Parameters

* `billingCycle.year` (integer)

* `billingCycle.month` (integer)

* `since.year` (integer)

* `since.month` (integer)

* `invoiceIds.invoiceIds` (array\\<string\>)

### Response Body

* `invoices` (array\\<object\>) — Invoices.

  * `teamId` (string) — The Team ID.

  * `invoiceId` (string) — The Invoice ID.

  * `invoiceNumber` (string) — The Invoice number.

  * `createTime` (string) — The creation time of the invoice.

  * `invoiceStatus` ("INVALID" | "PENDING" | "PAID" | "WILL\_NEVER\_BE\_CHARGED" | "FAILED") — Charging status of the invoice.

  * `firstDesiredNextCycleTs` (string) — When customer is expected to be charged.

  * `chargerAttempts` (array\\<object\>)

    * `ticket` (integer)

    * `successful` (boolean)

    * `paymentMethodId` (string) — The payment method that was tried in this attempt.

  * `lines` (array\\<object\>) — List of items composing the invoice.

    * `clusterName` (string) — The cluster on which the resource is consumed.

    * `description` (string) — The description of the line item.

    * `unitType` (string) — The unit in which the price is measured.

    * `unitPrice` (string) — The price per unit in ticks of 1e-10 USD (1/100\_000\_000 USD cents).

    * `numUnits` (string) — The number of units.

    * `amount` (string) — Total amount of the line item (USD cents).

  * `subtotal` (string)

  * `tax` (string) — without taxes in USD cents.

  * `total` (string) — Total due for the invoice taxes included in USD cents.

  * `invoicePdfAssetKeySuffix` (string) — The invoice pdf suffix.

  * `monthly` (object)

    * `billingCycle` (object)

      * `year` (integer)

      * `month` (integer)

    * `defaultCreditsIssued` (object) — Representation of USD Cents.

      * `val` (string)

    * `autoCreditsIssued` (object) — Representation of USD Cents.

      * `val` (string)

    * `prepaidTokensToSpend` (object) — Representation of USD Cents.

      * `val` (string)

    * `billingItemsCsvAssetKeySuffix` (string) — The billing items csv file suffix, used by xAI processes.

    * `correctionsCsvAssetKeySuffix` (string) — The billing items corrections csv file suffix, used by xAI processes.

  * `prepaid` (object)

  * `subscriptions` (object)

\*\*Response example:\*\*

```json
{
  "invoices": [
    {
      "teamId": "65c1e471-205f-4566-9c5a-07198bcdf4ce",
      "invoiceId": "aUa1nsnCQfxOFnWjqdimZczKKNJJ5xuwlkb-k0XiUOQ=",
      "invoiceNumber": "742-250-927-721",
      "createTime": "2025-04-01T21:19:48.569466Z",
      "invoiceStatus": "PAID",
      "firstDesiredNextCycleTs": "2025-04-11T02:46:58Z",
      "chargerAttempts": [
        {
          "ticket": 0,
          "successful": false,
          "paymentMethodId": ""
        },
        {
          "ticket": 1,
          "successful": true,
          "paymentMethodId": ""
        }
      ],
      "lines": [
        {
          "clusterName": "us-east-1",
          "description": "Chat grok-2-1212-1.0.0",
          "unitType": "Prompt text tokens",
          "unitPrice": "20000",
          "numUnits": "908",
          "amount": "0"
        },
        {
          "clusterName": "us-east-1",
          "description": "Chat grok-2-1212-1.0.0",
          "unitType": "Completion text tokens",
          "unitPrice": "100000",
          "numUnits": "534",
          "amount": "0"
        }
      ],
      "subtotal": "0",
      "tax": "0",
      "total": "0",
      "invoicePdfAssetKeySuffix": "teams/65c1e471-205f-4566-9c5a-07198bcdf4ce/billing/2025-2-aUa1nsnCQfxOFnWjqdimZczKKNJJ5xuwlkb-k0XiUOQ=.pdf",
      "monthly": {
        "billingCycle": {
          "year": 2025,
          "month": 2
        },
        "defaultCreditsIssued": {
          "val": "0"
        },
        "autoCreditsIssued": {
          "val": "0"
        },
        "prepaidTokensToSpend": {
          "val": "0"
        },
        "billingItemsCsvAssetKeySuffix": "teams/65c1e471-205f-4566-9c5a-07198bcdf4ce/billing/2025-2-aUa1nsnCQfxOFnWjqdimZczKKNJJ5xuwlkb-k0XiUOQ=-billing_items.csv.zstd",
        "correctionsCsvAssetKeySuffix": "teams/65c1e471-205f-4566-9c5a-07198bcdf4ce/billing/2025-2-aUa1nsnCQfxOFnWjqdimZczKKNJJ5xuwlkb-k0XiUOQ=-corrections.csv.zstd"
      }
    }
  ]
}
```

***

## GET /v1/billing/teams/\{team\_id}/payment-method

List payment methods of a team. You can add or delete the payment methods on https://console.x.ai.

### Path Parameters

* `team_id` (string, required) — Team ID of the team.

### Response Body

* `paymentMethods` (array\\<object\>) — Payment methods on file.

  * `paymentMethodId` (string)

  * `billingInfo` (object) — Billing info.

    * `name` (string) — The customer's full name or business name.

    * `address` (object) — Billing address.

      * `line1` (string) — Address line 1 (e.g., street, PO Box, or company name).

      * `line2` (string) — Address line 2 (e.g., apartment, suite, unit, or building).

      * `city` (string) — City, district, suburb, town, or village.

      * `country` (string) — Two-letter country code (\[ISO 3166-1 alpha-2]\(https://en.wikipedia.org/wiki/ISO\_3166-1\_alpha-2)).

      * `postalCode` (string) — ZIP or postal code.

      * `state` (string) — State, county, province, or region.

    * `email` (string) — The customer's email.

    * `taxIdType` (string)

    * `taxNumber` (string)

  * `cardDetails` (object) — A redacted version of card details. The actual card details are handled by third-party payment providers.

    * `brand` (string) — Card brand.
      Can be \`amex\`, \`diners\`, \`discover\`, \`eftpos\_au\`, \`jcb\`, \`mastercard\`, \`unionpay\`, \`visa\`, or \`unknown\`.

    * `expMonth` (string) — Two-digit number representing the card's expiration month.

    * `expYear` (string) — Four-digit number representing the card's expiration year.

    * `last4` (string) — The last four digits of the card.

  * `usBankAccountDetails` (object) — ACH details.

    * `bankName` (string)

    * `last4` (string)

    * `routingNumber` (string)

    * `blocked` (object) — ACH blocked reason.

      * `networkCode` (string)

      * `blockReason` (string)

  * `linkDetails` (object) — Link payment details.

    * `email` (string)

  * `paymentType` (string)

  * `addedTs` (string)

* `pendingPaymentMethod` (object) — Represents an incomplete attempt to add a payment method.
  Might resolve into a payment method.

  * `achMicrodepositHostedVerificationUrl` (string) — Verification URL for ACH micro deposits that verifies the account.

\*\*Response example:\*\*

```json
{
  "paymentMethods": [
    {
      "paymentMethodId": "pm_xxxxxxxxxxxxxxxxxxxxxxxx",
      "billingInfo": {
        "name": "Acme Inc.",
        "address": {
          "line1": "123 Main St.",
          "line2": "",
          "city": "New York",
          "country": "US",
          "postalCode": "12345",
          "state": "New York"
        },
        "email": "foo@example.com",
        "taxIdType": "us-ein",
        "taxNumber": "123-3456789"
      },
      "cardDetails": {
        "brand": "mastercard",
        "expMonth": "12",
        "expYear": "2030",
        "last4": "4444"
      },
      "usBankAccountDetails": null,
      "linkDetails": null,
      "paymentType": "card"
    }
  ],
  "pendingPaymentMethod": null
}
```

***

## POST /v1/billing/teams/\{team\_id}/payment-method/default

Set default payment method to an existing payment method on file.

### Path Parameters

* `team_id` (string, required) — Team ID of the team.

### Request Body

* `paymentMethodId` (string) — ID of the payment method that you want to set as default.

\*\*Request example:\*\*

```json
{
  "paymentMethodId": "pm_xxxxxxxxxxxxxxxxxxxxxxxx"
}
```

\*\*Response example:\*\*

```json
{}
```

***

## GET /v1/billing/teams/\{team\_id}/postpaid/invoice/preview

Preview the amount to pay for postpaid usage in the current billing period.

### Path Parameters

* `team_id` (string, required) — Team ID of the team.

### Response Body

* `coreInvoice` (object) — The invoice object.

  * `lines` (array\\<object\>) — Line items on the invoice.

    * `clusterName` (string) — The cluster on which the resource is consumed.

    * `description` (string) — The description of the line item.

    * `unitType` (string) — The unit in which the price is measured.

    * `unitPrice` (string) — The price per unit in ticks of 1e-10 USD (1/100\_000\_000 USD cents).

    * `numUnits` (string) — The number of units.

    * `amount` (string) — Total amount of the line item (USD cents).

  * `amountBeforeVatLimited` (object) — Representation of USD Cents.

    * `val` (string)

  * `amountBeforeVatUnlimited` (object) — Representation of USD Cents.

    * `val` (string)

  * `amountBeforeVatLimitedAndUnlimited` (object) — Representation of USD Cents.

    * `val` (string)

  * `amountBeforeVat` (string)

  * `vatCost` (string) — VAT (USD cents).

  * `amountAfterVat` (string) — Total amount after VAT (USD cents).

  * `autoCreditsIssued` (string) — Automatically issued credits (USD cents).

  * `defaultCreditsIssued` (string) — Default credit issued (USD cents).

  * `totalWithCorr` (object) — Representation of USD Cents.

    * `val` (string)

  * `prepaidCredits` (object) — Representation of USD Cents.

    * `val` (string)

  * `prepaidCreditsUsed` (object) — Representation of USD Cents.

    * `val` (string)

* `effectiveSpendingLimit` (string) — The effective current spending limit in USD cents.

* `defaultCredits` (string) — The current default credits in USD cents.

* `billingCycle` (object)

  * `year` (integer)

  * `month` (integer)

\*\*Response example:\*\*

```json
{
  "coreInvoice": {
    "lines": [],
    "amountBeforeVat": "0",
    "vatCost": "0",
    "amountAfterVat": "0",
    "autoCreditsIssued": "0",
    "defaultCreditsIssued": "0",
    "totalWithCorr": {
      "val": "0"
    },
    "prepaidCredits": {
      "val": "-4500"
    },
    "prepaidCreditsUsed": {
      "val": "0"
    }
  },
  "effectiveSpendingLimit": "20000",
  "defaultCredits": "0",
  "billingCycle": {
    "year": 2025,
    "month": 11
  }
}
```

***

## GET /v1/billing/teams/\{team\_id}/postpaid/spending-limits

Get the postpaid monthly spending limits. The API will stop functioning once the team has consumed all of the
prepaid credits, and the postpaid usage amount has reached the user-set soft spending limit.

### Path Parameters

* `team_id` (string, required) — Team ID of the team.

### Response Body

* `spendingLimits` (object) — Postpaid monthly spending limit.

  Override might not exist, hence optional (actually, this comes from
   either default hard\_spending\_limit override or monthly
   hard\_spending\_limit override).

  * `hardSlOverride` (object) — Representation of USD Cents.

    * `val` (string)

  * `hardSlAuto` (object) — Representation of USD Cents.

    * `val` (string)

  * `effectiveHardSl` (object) — Representation of USD Cents.

    * `val` (string)

  * `softSl` (object) — Representation of USD Cents.

    * `val` (string)

  * `effectiveSl` (object) — Representation of USD Cents.

    * `val` (string)

\*\*Response example:\*\*

```json
{
  "spendingLimits": {
    "hardSlAuto": {
      "val": "22500"
    },
    "effectiveHardSl": {
      "val": "22500"
    },
    "softSl": {
      "val": "20000"
    },
    "effectiveSl": {
      "val": "20000"
    }
  }
}
```

***

## POST /v1/billing/teams/\{team\_id}/postpaid/spending-limits

Set the postpaid monthly spending limit of a team. This can be used to restrict the maximum amount of postpaid API
usage. Note this will not limit the amount of prepaid credit usage, and prepaid credits will always be consumed
before accruing postpaid usage. To use only prepaid credits, you can set this limit to 0.

### Path Parameters

* `team_id` (string, required) — Team ID.

### Request Body

* `desiredSoftSpendingLimit` (object) — Representation of USD Cents.

  * `val` (string)

### Response Body

* `thisBpSoftSpendingLimit` (object) — Representation of USD Cents.

  * `val` (string)

\*\*Request example:\*\*

```json
{
  "desiredSoftSpendingLimit": {
    "val": "20000"
  }
}
```

\*\*Response example:\*\*

```json
{
  "thisBpSoftSpendingLimit": {
    "val": "20000"
  }
}
```

***

## GET /v1/billing/teams/\{team\_id}/prepaid/balance

List the prepaid credit balance and balance changes of a team.

### Path Parameters

* `team_id` (string, required) — The team ID of the team.

### Response Body

* `changes` (array\\<object\>) — The changes of the prepaid credit balance.

  * `teamId` (string) — The team ID.

  * `changeOrigin` ("INVALID\_ORIGIN" | "PURCHASE" | "SPEND" | "REFUND" | "MANUAL" | "AUTO\_PURCHASE") — The reason for the change.

     \- PURCHASE: Purchase by user. \`amount\` field will be negative.
     \- SPEND: Spending by user. \`amount\` field will be positive.
     \- REFUND: A refund issued to user. \`amount\` field will be negative.
     \- MANUAL: Can be either positive or negative, performed by xAI staff.
     \- AUTO\_PURCHASE: Can only be negative.

  * `topupStatus` ("INVALID\_STATUS" | "TO\_GENERATE\_INVOICE" | "FAILED\_TO\_GEMNERATE\_INVOICE" | "TO\_CHARGE" | "FAILED\_TO\_CHARGE" | "SUCCEEDED") — Status of the top up.

  * `amount` (object) — Representation of USD Cents.

    * `val` (string)

  * `invoiceId` (string) — Invoice ID.

  * `invoiceNumber` (string) — Invoice number.

  * `createTime` (string) — Creation time of the invoice.

  * `spendBpKeyYear` (integer) — Calendar year the purchase is made in.

  * `spendBpKeyMonth` (integer) — Calendar month the purchase is made in.

  * `createTs` (string) — Creation timestamp.

  * `paymentProcessor` (object)

    * `kind` ("UNKNOWN" | "STRIPE" | "CHECKOUT" | "EXTERNAL" | "MANUAL")

    * `externalParty` (string)

    * `externalInvoiceId` (string)

* `total` (object) — Representation of USD Cents.

  * `val` (string)

\*\*Response example:\*\*

```json
{
  "changes": [
    {
      "teamId": "65c1e471-205f-4566-9c5a-07198bcdf4ce",
      "changeOrigin": "PURCHASE",
      "topupStatus": "SUCCEEDED",
      "amount": {
        "val": "-1000"
      },
      "invoiceId": "7v7blf6c1G2g34OdI4N5tD1CKCdag4ZYLhn5vTBLIMM=",
      "invoiceNumber": "062-446-653-166",
      "createTime": "2025-02-24T15:28:02.308840Z",
      "paymentProcessor": {
        "kind": "STRIPE"
      }
    }
  ],
  "total": {
    "val": "-1000"
  }
}
```

***

## POST /v1/billing/teams/\{team\_id}/prepaid/top-up

Top up prepaid credit using the default payment method.

### Path Parameters

* `team_id` (string, required) — Team ID of the team to top up for.

### Request Body

* `amount` (object) — Representation of USD Cents.

  * `val` (string)

### Response Body

* `change` (object) — Change item on the prepaid credit balance.

  * `teamId` (string) — The team ID.

  * `changeOrigin` ("INVALID\_ORIGIN" | "PURCHASE" | "SPEND" | "REFUND" | "MANUAL" | "AUTO\_PURCHASE") — The reason for the change.

     \- PURCHASE: Purchase by user. \`amount\` field will be negative.
     \- SPEND: Spending by user. \`amount\` field will be positive.
     \- REFUND: A refund issued to user. \`amount\` field will be negative.
     \- MANUAL: Can be either positive or negative, performed by xAI staff.
     \- AUTO\_PURCHASE: Can only be negative.

  * `topupStatus` ("INVALID\_STATUS" | "TO\_GENERATE\_INVOICE" | "FAILED\_TO\_GEMNERATE\_INVOICE" | "TO\_CHARGE" | "FAILED\_TO\_CHARGE" | "SUCCEEDED") — Status of the top up.

  * `amount` (object) — Representation of USD Cents.

    * `val` (string)

  * `invoiceId` (string) — Invoice ID.

  * `invoiceNumber` (string) — Invoice number.

  * `createTime` (string) — Creation time of the invoice.

  * `spendBpKeyYear` (integer) — Calendar year the purchase is made in.

  * `spendBpKeyMonth` (integer) — Calendar month the purchase is made in.

  * `createTs` (string) — Creation timestamp.

  * `paymentProcessor` (object)

    * `kind` ("UNKNOWN" | "STRIPE" | "CHECKOUT" | "EXTERNAL" | "MANUAL")

    * `externalParty` (string)

    * `externalInvoiceId` (string)

\*\*Request example:\*\*

```json
{
  "amount": {
    "val": "500"
  }
}
```

\*\*Response example:\*\*

```json
{
  "change": {
    "teamId": "65c1e471-205f-4566-9c5a-07198bcdf4ce",
    "changeOrigin": "PURCHASE",
    "topupStatus": "TO_CHARGE",
    "amount": {
      "val": "-500"
    },
    "createTime": "2025-11-13T14:02:21.309537Z",
    "createTs": "2025-11-13T14:02:21.309537Z",
    "paymentProcessor": {
      "kind": "STRIPE"
    }
  }
}
```

***

## Daily token usage

`POST /v1/billing/teams/{team_id}/usage` returns your team's usage as time series, bucketed by time and optionally grouped by fields such as token type and model. It reads the same data as [Usage Explorer](/console/usage). Authenticate with a management key.

Choose what to measure with `values[].name`. Each value supports only `AGGREGATION_SUM`.

| Value | Measures |
| --- | --- |
| `usd` | Cost in USD |
| `units` | Tokens for token unit types; other units, such as images or audio seconds, for the rest |
| `items` | Billing items; one request produces an item for each priced dimension |
| `transactions` | Requests |

You can group and filter by `unit_type`, `description`, `api_key_id`, `product`, `request_ip`, `user_id`, and `unit_price`. `unit_type` is the token type, such as `Prompt text tokens`, `Cached prompt text tokens`, `Completion text tokens`, or `Reasoning text tokens`. `Prompt text tokens` counts only uncached input; cached input is reported as `Cached prompt text tokens`. `description` is the product name followed by the billed resource, usually the model.

The following request returns daily token counts for each token type and model:

```bash
curl https://management-api.x.ai/v1/billing/teams/{team_id}/usage \
  -X POST \
  -H "Authorization: Bearer $XAI_MANAGEMENT_KEY" \
  -H "Content-Type: application/json" \
  -d '{
        "analyticsRequest": {
          "timeRange": {
            "startTime": "2026-10-26 00:00:00",
            "endTime": "2026-11-03 00:00:00",
            "timezone": "Etc/GMT"
          },
          "timeUnit": "TIME_UNIT_DAY",
          "values": [{ "name": "units", "aggregation": "AGGREGATION_SUM" }],
          "groupBy": ["unit_type", "description"],
          "filters": []
        }
      }'
```

The response has one entry in `timeSeries` for each combination of group values. Its `group` array lists those values in `groupBy` order, so here the token type comes first and the description second. `dataPoints` has one entry per day in the range, and `values[0]` holds that day's token count.

* `startTime` and `endTime` use the format `YYYY-MM-DD HH:MM:SS` in the IANA time zone given by `timezone`, which also sets the day boundaries. `endTime` is exclusive.
* Keep `unit_type` in `groupBy` when you measure `units`. Grouping by `description` alone adds every unit type for a model into one number.
* If `limitReached` is `true`, the response hit its row limit and may be incomplete; shorten the time range or group by fewer fields.
* An unknown value or field name returns HTTP 400.

### Split long- vs short-context tokens

Long-context requests are billed at higher rates (see [Pricing](/developers/pricing)), but their tokens are reported under the same `unit_type` and `description` as other tokens. To separate them, add `unit_price` to `groupBy` in the request above so that each price gets its own time series:

```json
"groupBy": ["unit_type", "description", "unit_price"]
```

`unit_price` is measured in 1e-10 USD per unit, so dividing it by 10,000 gives USD per million tokens; `"25000"` is $2.50 per million tokens. Match each series to the model's standard and long-context rates to tell them apart. A token type and model can have other series too, for example from [US regional endpoint](/developers/advanced-api-usage/regions) rates, custom pricing, or a price change during the time range. To return only some prices, filter on `unit_price`, e.g., `"filters": ["unit_price:>12500"]`.

***

## POST /v1/billing/teams/\{team\_id}/usage

Get historical usage of the API (spend, token and other unit counts) over a time period,
bucketed by time and grouped by fields. For daily token counts, sum \`units\` per day grouped by
\`unit\_type\`.

### Path Parameters

* `team_id` (string, required) — The team whose billing records to analyze.

### Request Body

* `analyticsRequest` (object) — Request body for analytics.

  * `timeRange` (object) — Allows the user to specify a time range in their local timezone. Because of the way we aggregate
    logs, we can't rely on UTC timestamps.

    * `startTime` (string) — The from-time in the format YYYY-MM-DD HH:MM:SS.

    * `endTime` (string) — The to-time in the format YYYY-MM-DD HH:MM:SS (not including).

    * `timezone` (string) — The timezone that all timestamps are reported in.
      The timezone must be represented by the IANA time zone identifier (e.g. America/New\_York).

  * `timeUnit` ("TIME\_UNIT\_INVALID" | "TIME\_UNIT\_MONTH" | "TIME\_UNIT\_CALENDAR\_WEEK" | "TIME\_UNIT\_DAY" | "TIME\_UNIT\_HOUR" | "TIME\_UNIT\_QUARTER\_HOUR" | "TIME\_UNIT\_MINUTE" | "TIME\_UNIT\_SECOND" | "TIME\_UNIT\_NONE") — Time series are created by aggregating value into buckets we call \`TimeUnit\`.

     \- TIME\_UNIT\_NONE: None means having one single time bucket for all events.
    This can be used to count total number of events ever for example.

  * `values` (array\\<object\>) — Name of the fields to aggregate.

    * `name` (string) — Name of the value to measure. Valid names, and the aggregations each one supports, depend on
      the endpoint; see the endpoint's request description.

    * `aggregation` ("AGGREGATION\_NONE" | "AGGREGATION\_SUM" | "AGGREGATION\_AVG" | "AGGREGATION\_VAR" | "AGGREGATION\_STD" | "AGGREGATION\_MIN" | "AGGREGATION\_MAX" | "AGGREGATION\_P50" | "AGGREGATION\_P90" | "AGGREGATION\_P99" | "AGGREGATION\_P999" | "AGGREGATION\_COUNT" | "AGGREGATION\_COUNT\_DISTINCT") — Each value is an aggregate of the individual values in the time bucket. Note that not every field
      supports every aggregation method.

  * `groupBy` (array\\<string\>) — For each value of the group-by tuple, we return one time series.

  * `filters` (array\\<string\>) — All filter conditions are combined using AND.

### Response Body

* `timeSeries` (array\\<object\>) — For each value of the group-by clause, we return one time series.

  * `group` (array\\<string\>) — Values of the fields that were grouped by.

  * `groupLabels` (array\\<string\>) — Values to group the time series by.

  * `dataPoints` (array\\<object\>) — Data points ordered by timestamp. Data points are dense in the range provided (meaning we
    return one data point for every interval in the requested time range).

    * `timestamp` (string) — The timestamp (in UTC) when the data point was recorded.

    * `values` (array\\<number\>) — The values that were recorded at that datapoint.

* `limitReached` (boolean) — If this is true, the maximum cardinality of the query has been reached and only a subset of
  results is returned.

\*\*Request example:\*\*

```json
{
  "analyticsRequest": {
    "timeRange": {
      "startTime": "2025-10-01 00:00:00",
      "endTime": "2025-10-08 00:00:00",
      "timezone": "Etc/GMT"
    },
    "timeUnit": "TIME_UNIT_DAY",
    "values": [
      {
        "name": "units",
        "aggregation": "AGGREGATION_SUM"
      }
    ],
    "groupBy": [
      "unit_type",
      "description"
    ],
    "filters": []
  }
}
```

\*\*Response example:\*\*

```json
{
  "timeSeries": [
    {
      "group": [
        "Chat grok-4-0709"
      ],
      "groupLabels": [
        "Chat grok-4-0709"
      ],
      "dataPoints": [
        {
          "timestamp": "2025-10-01T00:00:00Z",
          "values": [
            0.75973725
          ]
        },
        {
          "timestamp": "2025-10-02T00:00:00Z",
          "values": [
            0
          ]
        },
        {
          "timestamp": "2025-10-03T00:00:00Z",
          "values": [
            0
          ]
        },
        {
          "timestamp": "2025-10-04T00:00:00Z",
          "values": [
            0
          ]
        },
        {
          "timestamp": "2025-10-05T00:00:00Z",
          "values": [
            0
          ]
        },
        {
          "timestamp": "2025-10-06T00:00:00Z",
          "values": [
            0
          ]
        },
        {
          "timestamp": "2025-10-07T00:00:00Z",
          "values": [
            0
          ]
        }
      ]
    },
    {
      "group": [
        "Chat grok-4-fast-non-reasoning"
      ],
      "groupLabels": [
        "Chat grok-4-fast-non-reasoning"
      ],
      "dataPoints": [
        {
          "timestamp": "2025-10-01T00:00:00Z",
          "values": [
            0
          ]
        },
        {
          "timestamp": "2025-10-02T00:00:00Z",
          "values": [
            0.0001037
          ]
        },
        {
          "timestamp": "2025-10-03T00:00:00Z",
          "values": [
            0
          ]
        },
        {
          "timestamp": "2025-10-04T00:00:00Z",
          "values": [
            0
          ]
        },
        {
          "timestamp": "2025-10-05T00:00:00Z",
          "values": [
            0
          ]
        },
        {
          "timestamp": "2025-10-06T00:00:00Z",
          "values": [
            0
          ]
        },
        {
          "timestamp": "2025-10-07T00:00:00Z",
          "values": [
            0
          ]
        }
      ]
    },
    {
      "group": [
        "grok-2-image-1212"
      ],
      "groupLabels": [
        "grok-2-image-1212"
      ],
      "dataPoints": [
        {
          "timestamp": "2025-10-01T00:00:00Z",
          "values": [
            0
          ]
        },
        {
          "timestamp": "2025-10-02T00:00:00Z",
          "values": [
            0
          ]
        },
        {
          "timestamp": "2025-10-03T00:00:00Z",
          "values": [
            0.14
          ]
        },
        {
          "timestamp": "2025-10-04T00:00:00Z",
          "values": [
            0
          ]
        },
        {
          "timestamp": "2025-10-05T00:00:00Z",
          "values": [
            0
          ]
        },
        {
          "timestamp": "2025-10-06T00:00:00Z",
          "values": [
            0
          ]
        },
        {
          "timestamp": "2025-10-07T00:00:00Z",
          "values": [
            0
          ]
        }
      ]
    }
  ],
  "limitReached": false
}
```

