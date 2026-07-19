# Skills Dashboard

A live catalog of all available agent skills is published and hosted at <https://skills.mahdtech.com>.

It provides a searchable, browseable web interface for exploring the skill catalog, checking requirements, and discovering triggers.

## Local Development

You can build and serve the dashboard locally to preview changes or run the site offline.

All commands must be run within the `devenv` shell environment:

```bash
# Build the static site and Zola content
devenv shell -- dashboard --action build

# Build and serve the site with live reloading (defaults to http://localhost:1111)
devenv shell -- dashboard --action serve
```
