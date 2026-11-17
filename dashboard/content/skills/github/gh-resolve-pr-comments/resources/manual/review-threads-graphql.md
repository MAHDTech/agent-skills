+++
title = "review-threads-graphql"
[extra]
skill = false
category = "github"
mermaid = false
skill_name = "gh-resolve-pr-comments"
+++

# Review thread GraphQL

REST review comments carry no resolved state and no link to their parent thread. Use these GraphQL calls to read thread state and to resolve threads.

## Fetch review threads

Run in step 3. Keep each thread's `id` (for resolving), `isResolved` (for filtering), `isOutdated`, and the `databaseId` of its comments, which equals the REST comment `id`:

```bash
gh api graphql --paginate -f query='
query($owner: String!, $repo: String!, $pr: Int!, $endCursor: String) {
  repository(owner: $owner, name: $repo) {
    pullRequest(number: $pr) {
      reviewThreads(first: 100, after: $endCursor) {
        nodes {
          id
          isResolved
          isOutdated
          comments(first: 100) {
            nodes {
              id
              databaseId
            }
          }
        }
        pageInfo {
          hasNextPage
          endCursor
        }
      }
    }
  }
}' -f owner="{owner}" -f repo="{repo}" -F pr={pr_number}
```

A REST inline comment belongs to the thread whose `comments.nodes[].databaseId` contains its `id`.

## Resolve a thread

Run in step 8, only after the thread's reply URL is confirmed:

```bash
gh api graphql -f query='
mutation($threadId: ID!) {
  resolveReviewThread(input: {threadId: $threadId}) {
    thread {
      isResolved
    }
  }
}' -f threadId="{thread_id}"
```

The response returns `isResolved: true` when the thread is resolved.

