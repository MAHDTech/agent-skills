# Auto-publishing skills to a public marketplace on every commit

## Why this is out of scope

This repository is a curated, personal-plus-public catalog. Publishing every commit to a third-party marketplace would push half-finished and experimental skills (the ones under `in-progress/`) to users, and it would tie release cadence to an external service the project does not control.

Distribution already works well through the `skills.sh` CLI (`npx skills add MAHDTech/agent-skills`), which pulls from the repository on demand. That keeps the source of truth here and lets consumers opt into exactly the skills they want. If a marketplace becomes worthwhile later, it should be a deliberate, versioned release step, not an automatic side effect of every commit.

## Prior requests

- <https://github.com/MAHDTech/agent-skills/issues/000> - asked for a GitHub Action to publish on push to `trunk`
