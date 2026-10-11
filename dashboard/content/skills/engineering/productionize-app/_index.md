+++
title = "productionize-app"
description = "Harden and prepare an application for production deployment across configuration, reliability, security, observability, and target platforms."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "engineering"
mermaid = false
+++


# Productionize App

Prepare an application for production deployment across configuration, error handling, security, performance, and platform packaging.

## Usage patterns

```bash
/productionize-app                          # Auto-detect framework, general production prep
/productionize-app flutter testflight       # Flutter app for TestFlight
/productionize-app react vercel             # React app for Vercel deployment
/productionize-app nodejs docker            # Node.js app for Docker deployment
/productionize-app python heroku            # Python app for Heroku
/productionize-app vue netlify              # Vue app for Netlify
```

## Workflow

### 1. Initial analysis and planning

- List tasks needed to reach production readiness.
- Identify the framework, build tools, and target deployment platform.
- Review existing configuration files (`package.json`, `pubspec.yaml`, `Dockerfile`, `.env.example`).
- Map integration points, external dependencies, and failure modes.

### 2. Codebase audit

Audit the codebase for production readiness gaps:

- **Configuration:** Hardcoded credentials, URLs, or secrets; missing environment separation (development, staging, production).
- **Error handling:** Uncaught rejections, missing fallback paths, missing timeouts on network calls.
- **Performance:** Missing database indexes, unbounded pagination, uncompressed assets, missing cache headers.
- **Security:** Missing input sanitization, exposed debug endpoints, overly broad CORS headers.
- **Observability:** Missing request IDs, absence of structured logging, missing health-check endpoints.
- **Resilience:** Unhandled offline states, missing retry policies on transient network calls.

### 3. Implementation

Apply fixes systematically:

#### Configuration and environment

- Separate runtime config by environment using environment variables.
- Add configuration validation on startup so invalid settings fail fast.
- Move API keys and secrets to secret stores or environment variables.

#### Performance and reliability

- Add caching headers and in-memory or storage caching where appropriate.
- Configure request timeouts and retries with backoff for remote dependencies.
- Add loading states, empty states, and offline fallbacks.

#### User experience

- Replace stack traces or raw errors with clear, actionable user messages.
- Test error boundary components and edge-case inputs.

#### Code quality and observability

- Add structured logs with log levels (debug, info, warn, error).
- Write regression and integration tests for critical business paths.
- Remove debug flags, dead prototypes, and temporary mocks.

### 4. Framework-specific optimizations

Apply the checklist for the detected framework (Flutter, React/Next.js, Node.js, Python) from [resources/manual/framework-checklists.md](@/skills/engineering/productionize-app/resources/manual/framework-checklists.md#framework-specific-optimizations).

### 5. Documentation

Keep project documentation operational and direct:

#### README structure

```markdown
# Project Name

Summary of the application.

## Quick start

Steps to run the application locally.

## Configuration

Required environment variables and secret configuration.

## Deployment

Build and deployment commands for the target platform.

## Architecture

High-level component layout and key external dependencies.
```

### 6. Deployment preparation

Apply the checklist for the chosen deployment target (TestFlight, Google Play, Web, or Container) from [resources/manual/framework-checklists.md](@/skills/engineering/productionize-app/resources/manual/framework-checklists.md#deployment-preparation).

### 7. Quality assurance checklist

Before finishing:

- [ ] Configurations load from environment variables and fail fast when missing
- [ ] Network calls set timeouts and handle failures gracefully
- [ ] Secrets and credentials stay out of source control
- [ ] Build artifact produces clean output with optimization flags enabled
- [ ] Health checks, monitoring, and structured logs report operational status
- [ ] Target platform deployment succeeds in a staging or dry-run environment

### 8. Framework detection

Auto-detect framework from repository files:

- **Flutter:** `pubspec.yaml`, `.dart` files
- **React / Next.js:** `package.json` with `react` or `next` dependencies
- **Vue / Nuxt:** `package.json` with `vue` or `nuxt` dependencies
- **Node.js:** `package.json` with runtime servers (`express`, `fastify`, `koa`, `hono`)
- **Python:** `pyproject.toml`, `requirements.txt`, `.py` files
- **Go:** `go.mod`, `.go` files
- **Rust:** `Cargo.toml`, `src/main.rs`

