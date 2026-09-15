# Skills Dashboard

A live catalog of all available agent skills is published and hosted at <https://skills.mahdtech.com>.

It provides a searchable, browseable web interface for exploring the skill catalog, checking requirements, and discovering triggers.

## Local Development

You can build and serve the dashboard locally to preview changes or run the site offline.

All commands must be run within the `devenv` shell environment:

```bash
# Build the static site and Zola content
devenv --no-tui shell -- ask dashboard build

# Build and serve the site with live reloading (defaults to http://localhost:1111)
devenv --no-tui shell -- ask dashboard serve

# Compile Tailwind CSS stylesheet directly
devenv --no-tui shell -- ask dashboard css

# Verify dashboard content matches skill source files
devenv --no-tui shell -- ask dashboard lint
```
