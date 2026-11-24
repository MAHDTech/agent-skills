+++
title = "framework-checklists"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "productionize-app"
+++

# Framework and deployment checklists

Reference material for the `productionize-app` skill. Apply the block that matches the app's framework (during Step 3) and the block that matches the deployment target (during Step 4).

## Framework-specific optimizations

### Flutter

- Store tokens and credentials in `flutter_secure_storage`; reserve `shared_preferences` for non-sensitive flags and cache metadata.
- Configure explicit `connectTimeout` and `receiveTimeout` on `Dio` or `http.Client`, mapping timeout and non-2xx responses to typed UI error states.
- Use `const` constructors, `ListView.builder` (or `SliverList`) for unbounded lists, and verify frame timings in `--profile` mode.
- Audit platform configs: iOS `Info.plist` privacy usage strings (`NS*UsageDescription`) and Android `AndroidManifest.xml` minimal permissions plus `isMinifyEnabled = true` (R8/ProGuard) for release builds.
- Wire route and form state restoration (`RestorationMixin` or router `restorationScopeId`) so OS background kills recover cleanly.

### React/Next.js

- Wrap route subtrees with React Error Boundaries (`error.tsx` in Next.js App Router or an explicit `ErrorBoundary`) and `Suspense` fallbacks so render crashes do not blank the app.
- Keep server state in a dedicated cache layer (TanStack Query, SWR, or Next.js `fetch` cache) with explicit `staleTime`, bounded `retry` counts, and `AbortController` cancellation on unmount; keep client UI state minimal and scoped.
- Code-split heavy routes/widgets via `React.lazy` or `next/dynamic` and verify bundle size with `@next/bundle-analyzer` or `vite-bundle-visualizer`.
- Confirm `tsc --noEmit` and production build (`next build` / `vite build`) pass with zero errors and no `console.log` noise left in hot render paths.
- Set page `title`, `description`, OpenGraph tags, canonical URLs, `robots.txt`, and `sitemap.xml` for public routes.

### Node.js

- Register a terminal Express/Fastify error middleware that logs structured error details internally and returns sanitized JSON (omitting stack traces when `NODE_ENV=production`).
- Enable `helmet` (or equivalent headers: `Strict-Transport-Security`, `X-Content-Type-Options: nosniff`, `Content-Security-Policy`), restrict CORS to an explicit origin allowlist, and apply request rate limiting (`express-rate-limit` or `@fastify/rate-limit`).
- Configure bounded database pool sizes (`max`, `idleTimeoutMillis`, `connectionTimeoutMillis`) and statement timeouts.
- Expose `GET /healthz` (process liveness) and `GET /readyz` (dependency check returning `200` or `503`), plus graceful `SIGTERM`/`SIGINT` shutdown draining open connections.
- Validate `process.env` at startup with `zod`, `envalid`, or `valibot`, exiting non-zero immediately if required keys are missing or malformed.

### Python

- Configure structured JSON logging (`structlog` or a JSON `logging.Formatter`) with request/correlation IDs, and handle uncaught exceptions in a single framework exception handler that hides tracebacks in production.
- Validate request payloads and startup environment config with `pydantic` / `pydantic-settings` so invalid inputs return `422`/`400` and missing env vars fail fast on boot.
- Set explicit timeouts on all `httpx`/`requests` calls and configure SQLAlchemy/DB pool limits (`pool_size`, `max_overflow`, `pool_pre_ping=True`); eliminate N+1 queries using `selectinload`/`joinedload` where needed.
- Add bounded TTL caching (Redis or `cachetools`) for expensive read-heavy queries with explicit invalidation keys.
- Run under a production ASGI/WSGI server (`uvicorn` / `gunicorn` with configured worker count, no `--reload`) and enforce `pytest`, `ruff`, and `mypy`/`pyright` in CI.

## Deployment preparation

### TestFlight (iOS)

- Increment `CFBundleVersion` (build number) and verify bundle identifier matches App Store Connect.
- Confirm distribution signing certificate and provisioning profile succeed in `xcodebuild -exportArchive` (or Fastlane `match`/`gym`).
- Verify `Info.plist` includes `ITSAppUsesNonExemptEncryption` and all required privacy usage descriptions.
- Archive and upload dSYM symbols for crash symbolication.
- Validate and upload the `.ipa` via `xcrun altool` / `xcrun notarytool` or Fastlane `pilot` with zero App Store Connect validation errors.

### Google Play (Android)

- Build a signed release Android App Bundle (`./gradlew bundleRelease`) using an upload keystore stored outside the repository.
- Increment `versionCode`, confirm `targetSdkVersion` meets current Play Store policy, and verify `android:debuggable="false"`.
- Audit `AndroidManifest.xml` to strip unused permissions and verify R8/ProGuard shrinking (`isMinifyEnabled = true`) with mapping files archived for crash deobfuscation.
- Smoke-test the release `.aab` (via `bundletool` or Play Internal Testing) across at least one `minSdkVersion` and one current-API device/emulator.

### Web deployment (Vercel/Netlify/AWS)

- Verify the production build command succeeds cleanly in CI with separate preview and production environment variable sets.
- Confirm custom domain DNS, valid TLS certificate, automatic HTTP-to-HTTPS redirect, and `Strict-Transport-Security` header.
- Set `Cache-Control: public, max-age=31536000, immutable` on content-hashed static assets and revalidation (`no-cache` or short `s-maxage`) on HTML entry points.
- Verify custom `404`/`500` pages, SPA/SSR routing rewrites, and uptime/error alerting (e.g., Sentry, CloudWatch, or provider alerts).

### Container deployment (Docker/Kubernetes)

- Use a multi-stage `Dockerfile` with pinned base image version/digest, `.dockerignore` excluding `.git`, `.env`, and tests, and a non-root runtime `USER`.
- Define Docker `HEALTHCHECK` and Kubernetes `livenessProbe`, `readinessProbe`, and explicit `resources.requests` and `resources.limits`.
- Inject secrets at runtime via Kubernetes `Secret` or an external secret manager (zero secrets baked into image layers or `ENV` defaults).
- Verify the container handles `SIGTERM` and exits `0` within `terminationGracePeriodSeconds`, and emits JSON logs to `stdout`/`stderr`.
- Validate horizontal scaling with at least 2 replicas (confirming stateless request handling or shared session/cache storage).

