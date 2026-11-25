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


# Productionize app

**Invocation:** Start only when the user explicitly requests this workflow.

Prepare an application for production deployment across configuration, reliability, security, performance, observability, packaging, and deployment readiness.

## Usage patterns

Optional arguments name the framework and deployment target:

```text
/productionize-app                          # Auto-detect framework, general production prep
/productionize-app flutter testflight       # Flutter app for TestFlight
/productionize-app react vercel             # React app for Vercel deployment
/productionize-app nodejs docker            # Node.js app for Docker deployment
```

Framework checklists cover Flutter, React/Next.js, Node.js, and Python; deployment checklists cover TestFlight, Google Play, web hosts (Vercel, Netlify, AWS), and containers. For any other stack or target, run the general workflow and state which checklist was unavailable.

## Workflow

### 1. Detect stack and baseline

- Detect the framework from repository files: Flutter (`pubspec.yaml`, `.dart`), React/Next.js (`package.json` with `react` or `next`), Node.js (`package.json` with `express`, `fastify`, `koa`, or `hono`), or Python (`pyproject.toml`, `requirements.txt`).
- Inspect existing packaging, deployment, and config files (`Dockerfile`, CI workflows, `.env.example`, manifest files) and identify build, lint, and test commands.
- Map external dependencies (databases, queues, third-party APIs) and entry points.

### 2. Audit and fix core production domains

Audit each domain, record concrete gaps with file paths, and apply fixes directly:

- **Configuration and secrets:** Move hardcoded credentials, API keys, and environment-specific URLs to environment variables or a secret store; validate required environment variables at startup so the app exits non-zero when required settings are missing; keep `.env.example` in sync and secrets out of Git.
- **Reliability and timeouts:** Set explicit connect/read timeouts on all outbound HTTP and database calls; add bounded retries with exponential backoff only for idempotent or safe transient failures; handle unhandled promise rejections / uncaught exceptions; add graceful `SIGTERM`/`SIGINT` shutdown draining in-flight requests.
- **Security:** Sanitize and schema-validate external inputs; remove or gate debug endpoints and dev flags in production builds; restrict CORS to explicit allowed origins; set security response headers; ensure error responses return safe messages without stack traces or internal paths.
- **Performance:** Bound pagination and payload limits; eliminate obvious N+1 queries and add missing database indexes for hot lookups; enable asset compression and explicit `Cache-Control` headers for static assets and cacheable responses.
- **Observability:** Emit structured JSON logs with explicit log levels (`debug`, `info`, `warn`, `error`) and request correlation IDs; expose a liveness/readiness health endpoint (e.g., `/healthz`, `/readyz`) that verifies critical dependencies; remove leftover debug logging and temporary mocks.
- **User experience (client apps):** Add top-level error boundaries, loading states, empty states, and offline error recovery for network requests.

### 3. Apply framework-specific optimizations

Apply the checklist for the detected framework (Flutter, React/Next.js, Node.js, or Python) from [resources/manual/framework-checklists.md](@/skills/engineering/productionize-app/resources/manual/framework-checklists.md#framework-specific-optimizations).

### 4. Prepare deployment and documentation

- Apply the target checklist (TestFlight, Google Play, Web, or Container) from [resources/manual/framework-checklists.md](@/skills/engineering/productionize-app/resources/manual/framework-checklists.md#deployment-preparation).
- Update `README.md` with required environment variables and secrets, exact build/test/deploy commands for the target platform, health check endpoints, and external service dependencies.

### 5. Verify production readiness

Run the stack's concrete verification commands and record exact results:

1. **Production build:** Run the release/production build command with optimization flags enabled (e.g., `npm run build`, `flutter build --release`, `docker build`, or `python -m build`) and confirm exit code `0`.
2. **Test and lint suite:** Run the automated test, typecheck, and lint suite and add regression tests for any newly added startup validation, health endpoints, or error handlers.
3. **Secret and config check:** Scan tracked files and build outputs for hardcoded secrets or `.env` leaks, and verify the app fails fast with a clear error when a required environment variable is unset.
4. **Health and deployment smoke check:** Start the built artifact locally (or run a staging/dry-run deploy) and verify the health endpoint returns HTTP `200` (and `503` when a required dependency is unavailable, if applicable).

## Completion criteria

- Every gap identified in Steps 2 through 4 is fixed or explicitly documented as a follow-up with owner/blocker.
- Production build, automated tests, secret/config fail-fast check, and health/smoke verification all pass with recorded command output.
- `README.md` and `.env.example` document all required runtime configuration and deployment steps.

