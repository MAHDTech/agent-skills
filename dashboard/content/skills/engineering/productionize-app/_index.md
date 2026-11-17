+++
title = "productionize-app"
description = "Prepare an entire application for production when explicitly requested. Audit and fix configuration, reliability, security, observability, packaging, and deployment readiness."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "engineering"
mermaid = false
+++


# Productionize App

**Invocation:** Start only when the user explicitly requests this workflow.

Prepare an application for production deployment across configuration, error handling, security, performance, and platform packaging.

## Usage patterns

The optional arguments name the framework and the deployment target:

```text
/productionize-app                          # Auto-detect framework, general production prep
/productionize-app flutter testflight       # Flutter app for TestFlight
/productionize-app react vercel             # React app for Vercel deployment
/productionize-app nodejs docker            # Node.js app for Docker deployment
```

Framework checklists cover Flutter, React/Next.js, Node.js, and Python; deployment checklists cover TestFlight, Google Play, web hosts (Vercel, Netlify, AWS), and containers. For any other stack or target, run the general audit and say which checklist was unavailable.

## Workflow

### 1. Initial analysis and planning

- List tasks needed to reach production readiness.
- Detect the framework from repository files: Flutter (`pubspec.yaml`, `.dart` files); React or Next.js (`package.json` with `react` or `next`); Node.js (`package.json` with a server such as `express`, `fastify`, `koa`, or `hono`); Python (`pyproject.toml`, `requirements.txt`). Identify the build tools and target deployment platform.
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

Update the README so a new operator can run, configure, and deploy the application: required environment variables and secrets, the build and deployment commands for the target platform, and the key external dependencies.

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

