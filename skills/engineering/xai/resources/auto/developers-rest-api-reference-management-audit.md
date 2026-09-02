#### Management API

# Audit Logs

***

## GET /audit/teams/\{teamId}/events

API endpoint for GET requests to /audit/teams/\{teamId}/events.

### Path Parameters

* `teamId` (string, required) — The team id to get events for.

### Query Parameters

* `pageSize` (integer) — The number of events to list per page.

* `pageToken` (string) — When querying the next page, the offset token.

* `eventFilter.userId` (string) — Filter events to this user only.

* `eventFilter.query` (string) — A general search term that is used for full-text description search.

* `eventFilter.eventId` (string) — Search for a specific event by id.

* `eventTimeFrom` (string) — Filter events from and/or to certain time (optional).

* `eventTimeTo` (string)

* `orderBy` ("TIME\_ASCENDING" | "TIME\_DESCENDING")

### Response Body

* `events` (array\\<object\>) — Requested events. Ordered by event\_time.

  * `eventTime` (string)

  * `eventId` (string) — Identifier to reference this log. Not bound to anything else in the system.

  * `description` (string) — Free form description of the event in English.

  * `user` (object)

    * `userId` (string) — User ID.

    * `email` (string) — User's email. May not always populated.

    * `profileImage` (string) — The key of the profile image under which it can be fetched from our assets server.

    * `givenName` (string) — User's given name.

    * `familyName` (string) — User's family name.

    * `profileImageUrl` (string) — The full URL path to the user's profile image.

* `nextPageToken` (string) — If there are more events, the token to be used to
  retrieve the next page of the search results.

\*\*Response example:\*\*

```json
{
  "events": [
    {
      "eventTime": "2025-01-15T10:30:00Z",
      "eventId": "550e8400-e29b-41d4-a716-446655440000",
      "description": "API key 'Production Key' was created",
      "user": {
        "userId": "user-123",
        "email": "admin@example.com",
        "givenName": "John",
        "familyName": "Doe"
      }
    }
  ],
  "nextPageToken": "550e8400-e29b-41d4-a716-446655440000"
}
```