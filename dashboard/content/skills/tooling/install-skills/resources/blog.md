+++
title = "blog"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "install-skills"
+++

# Blog

[](/blog/bun-in-rust)

Rewriting Bun in Rust

Why & how we rewrote Bun from Zig to Rust

Jarred Sumner · July 8, 2026

[](/blog/bun-v1.3.14)

Bun v1.3.14

Fixes 92 issues (addressing 380 👍). Bun.Image — a built-in image
processing API, 7x faster warm installs with the isolated linker's
global store, experimental HTTP/2 and HTTP/3 clients for fetch(), HTTP/3
(QUIC) in Bun.serve(), rewritten fs.watch() on Linux and macOS,
--no-orphans CLI flag, process.execve(), Bun.Terminal on Windows via
ConPTY, FreeBSD and Android builds, shared SSL_CTX cache, smaller
binaries, and many bugfixes and Node.js compatibility improvements.

Jarred Sumner · May 13, 2026

[](/blog/bun-v1.3.13)

Bun v1.3.13

Fixes 82 issues (addressing 381 👍). bun test --parallel, --isolate,
--shard, and --changed, bun install streams tarballs to disk using 17x
less memory, source maps use 8x less memory, 5.5x faster gzip with
zlib-ng, Range request support in Bun.serve(), SHA3 in node:crypto and
WebCrypto, ws+unix:// WebSocket client, and many bugfixes and Node.js
compatibility improvements

Jarred Sumner · April 20, 2026

[](/blog/bun-v1.3.12)

Bun v1.3.12

Fixes 120 issues (addressing 219 👍). Render Markdown in the terminal
with bun ./file.md, Bun.WebView headless browser automation, in-process
Bun.cron() scheduler, async stack traces for native errors, 2.3x faster
URLPattern, 2x faster Bun.Glob.scan, cgroup-aware parallelism on Linux,
and many bugfixes and Node.js compatibility improvements.

Jarred Sumner · April 9, 2026

[](/blog/bun-v1.3.11)

Bun v1.3.11

Fixes 105 issues (addressing 307 👍). 4 MB smaller on Linux. x64
Bun.cron for OS-level cron jobs and expression parsing, Bun.sliceAnsi
for ANSI/grapheme-aware string slicing, richer Bun.markdown list
metadata; bun test --path-ignore-patterns; native Windows ARM64 .bin
shim; dgram UDP fixed on macOS, fs.openSync numeric flags fixed on
Windows (unbreaks tar@7), http2 WINDOW_UPDATE fix; bundler
--bytecode/barrel/import-attribute fixes; CSS
unicode-range/mask/border-radius fixes; Proxy-array crash fixes, CRLF
injection fix, bun install hang fixes, and many Node.js compatibility
improvements.

Jarred Sumner · March 18, 2026

[](/blog/bun-v1.3.10)

Bun v1.3.10

Fixes 155 issues (addressing 642 👍). New native REPL, --compile
--target=browser for self-contained HTML, TC39 standard ES decorators,
Windows ARM64 support, barrel import optimization, and a faster event
loop. structuredClone up to 25x faster for arrays, Buffer.slice() 1.8x
faster, path.parse() up to 7x faster, fewer closures in bundled output,
and upgraded JavaScriptCore with 168x faster deep rope string slicing.
Fixes Python asyncio-based MCP servers breaking with Bun.spawn(), plus
numerous bugfixes across Node.js compatibility, bun install, the
bundler, shell, and Windows.

Jarred Sumner · February 26, 2026

[](/blog/bun-v1.3.9)

Bun v1.3.9

Fixes 23 issues (addressing 13 👍). Run multiple scripts in --parallel
or --sequential. bun:test mock & spyOn get Symbol.dispose support. ESM
bytecode compilation. Faster Bun.markdown. Faster RegExp,
String.prototype.trim, String.prototype.startsWith.

Jarred Sumner · February 8, 2026

[](/blog/bun-v1.3.8)

Bun v1.3.8

Fixes 8 issues (addressing 6 👍). Bun.markdown is a builtin
CommonMark-compliant Markdown parser written in Zig. Bun's bundler gets
bun build --metafile-md to write LLM-friendly module graph metadata.
Fixes a regression in 'npm install -g bun' on Windows.

Jarred Sumner · January 29, 2026

[](/blog/bun-v1.3.7)

Bun v1.3.7

Fixes 91 issues (addressing 243 👍). 50% faster `Buffer.from(array)`,
35% faster async/await, 3x faster `array.flat()`, 90% faster
`padStart`/`padEnd` via JSC upgrade. New: `Bun.JSON5`, `Bun.JSONL`,
`Bun.wrapAnsi()` (33-88x faster than wrap-ansi npm), `--heap-prof-md`,
`--cpu-prof-md`, `node:inspector` Profiler API, `replMode` for
`Bun.Transpiler`. Fetch now preserves header casing. HTTP/2 fixes for
AWS ALB/gRPC/Fauna.

Jarred Sumner · January 27, 2026

[](/blog/bun-v1.3.6)

Bun v1.3.6

Fixes 45 issues (addressing 125 👍). Bun.Archive API for creating &
extracting tarballs with gzip compression, Bun.JSONC for parsing JSON
with comments, metafile & files options in Bun.build for bundle analysis
and virtual files, Response.json() 3.5x faster, 15% faster async/await,
30% faster Promise.race, 20x faster Bun.hash.crc32, faster
Buffer.indexOf/includes with SIMD, HTTP/HTTPS proxy support for
WebSocket, S3 Requester Pays support, --grep flag for bun test, fake
timers now work with @testing-library/react, SQLite updated to 3.51.2,
and numerous Node.js compatibility improvements.

Jarred Sumner · January 13, 2026

[](/blog/bun-v1.3.5)

Bun v1.3.5

Fixes 32 issues (addressing 25 👍). Bun.Terminal API, compile-time
feature flags, improved Bun.stringWidth accuracy, V8 C++ value type
checking APIs, Content-Disposition support for S3 uploads, environment
variable expansion in .npmrc quoted values, and numerous bug fixes &
node.js compatibility improvements

Jarred Sumner · December 17, 2025

[](/blog/bun-v1.3.4)

Bun v1.3.4

Fixes 194 issues (addressing 412 👍). URLPattern API, Fake Timers for
bun:test, Custom Proxy Headers in fetch(),console.log %j format,
http.Agent connection pooling fix, Standalone executable config loading
changes, SQLite 3.51.1, and numerous bugfixes for bun:test, Node.js
compatibility, Windows, Web APIs, bun install, and more

Jarred Sumner · December 6, 2025

[](/blog/bun-joins-anthropic)

Bun is joining Anthropic

Bun has been acquired by Anthropic. Anthropic is betting on Bun as the
infrastructure powering Claude Code, Claude Agent SDK, and future AI
coding products & tools.

Jarred Sumner · December 2, 2025

[](/blog/bun-v1.3.3)

Bun v1.3.3

Fixes 95 issues (addressing 348 👍). CompressionStream and
DecompressionStream, Control .env and bunfig.toml loading in standalone
executables, `retry` and `repeats` in `bun:test`, --no-env-file, SQLite
3.51.0, Upgraded to Zig 0.15.2, Bugfixes for bundler, bun install,
Windows, Node.js compatibility, Web APIs, Bun.serve, networking,
Bun.YAML, Bun.Transpiler, Bun.spawn & TypeScript definitions

Jarred Sumner · November 21, 2025

[](/blog/bun-v1.3.2)

Bun v1.3.2

Fixes 287 issues (addressing 324 👍). Hoisted installs restored as
default for backward compatibility. CPU profiling with --cpu-prof,
faster installs for packages with post-install scripts, bun:test
onTestFinished hook, ServerWebSocket subscriptions getter, Alpine 3.22
in Docker images, improved Git dependency resolution, bun list alias,
and numerous bug fixes across install, HTTP/HTTPS, N-API, bun test, bun
build, and runtime.

Jarred Sumner & Lydia Hallie · November 8, 2025

[](/blog/vercel-adds-native-bun-support)

Vercel now supports the Bun Runtime

Deploy and run your applications with Bun's fast JavaScript runtime on
Vercel Functions, with full access to Bun APIs and improved performance

Jarred Sumner & Lydia Hallie · October 28, 2025

[](/blog/bun-v1.3.1)

Bun v1.3.1

Fixes 103 issues (addressing 172 👍). 2× faster bun build on macOS for
symlink-heavy projects, source maps preserve legal comments, and
--format=cjs now inlines import.meta; bun test adds global vi,
--pass-with-no-tests, and --only-failures; .npmrc :email auth support,
faster installs when no peer deps, selective hoisting for isolated
installs (publicHoistPattern/hoistPattern), and fs/promises
FileHandle.readLines(); numerous bundler/transpiler fixes (external
sourcemaps in Bun.build({ compile:true }), --bytecode with import.meta,
Windows react-jsxdev); reliability fixes across install/pm/pack, bunx,
Node.js compatibility improvements, and numerous bug fixes.

Jarred Sumner · October 22, 2025

[](/blog/bun-v1.3)

Bun 1.3

Bun 1.3 introduces zero-config frontend development, unified SQL API,
built-in Redis client, security enhancements, package catalogs, async
stack traces, VS Code test integration, and Node.js compatibility
improvements.

Jarred Sumner & Lydia Hallie & Ciro Spaciari & Dylan Conway & Aidan
Bloore & Alistair Smith & Taylor & Marko Vejnovic & Michael H & Zack ·
October 10, 2025

[](/blog/bun-v1.2.23)

Bun v1.2.23

Fixes 119 issues (addressing 412 👍). pnpm-lock.yaml migration for
seamless switching from pnpm to bun install, Redis Pub/Sub support,
concurrent test execution with configurable parallelism,
platform-specific dependency filtering with --cpu and --os flags, system
CA certificates support with --use-system-ca, Windows code signing for
compiled executables, JSX configuration improvements in Bun.build,
sql.array helper for parameterized arrays, randomized test ordering,
stricter CI test defaults, process.report.getReport() on Windows, and
numerous Node.js compatibility improvements including http, dns,
worker_threads, crypto, http2, net, and tty modules.

Jarred Sumner · September 28, 2025

[](/blog/bun-v1.2.22)

Bun v1.2.22

Fixes 144 issues (addressing 257 👍). Async stack traces for better
debugging, RFC 6455 compliant WebSocket client subprotocol negotiation,
MySQL adapter improvements including affectedRows and lastInsertRowid
support, JSX side effects preservation during bundling, event loop delay
monitoring with perf_hooks, HTTP server idle connection management,
--workspaces in bun run, function name removal during minification, and
numerous Node.js compatibility fixes including child_process.spawnSync,
socket.write with Uint8Array, crypto.verify RSA defaults, N-API
improvements, HTMLRewriter error handling, fetch decompression fixes,
Bun.SQL, and Bun.\$ improvements.

Jarred Sumner · September 14, 2025

[](/blog/behind-the-scenes-of-bun-install)

Behind The Scenes of Bun Install

Learn how Bun is able to cut install times by up to 25×. Bun skips
Node.js's overhead with direct system calls, cache-friendly data
layouts, OS-level copy-on-write, and full-core parallelism.

Lydia Hallie · September 10, 2025

[](/blog/bun-v1.2.21)

Bun v1.2.21

Fixes 69 issues (addressing 204 👍). Bun.SQL now supports MySQL and
SQLite, alongside PostgreSQL. Native YAML support. 500x faster
postMessage(string). Bun.build() compile API, with cross-platform
targets. Reduced idle CPU usage. Bun.stripANSI for SIMD-accelerated ANSI
escape removal. bunx --package flag support. customizable User-Agent
headers. Windows executable metadata embedding. and extensive Node.js
compatibility improvements

Jarred Sumner · August 25, 2025

[](/blog/how-we-made-postMessage-string-500x-faster)

500x faster postMessage(string)

Zero-copy postMessage(string)

Jarred Sumner · August 20, 2025

[](/blog/bun-v1.2.20)

Bun v1.2.20

Fixes 141 issues (addressing 429 👍). Reduced idle CPU usage. Improved
`bun:test` diffing. Automatic `ETag` and `If-None-Match` in static
routes for `Bun.serve`. 40x faster `AbortSignal.timeout`. Windows long
path support. `WebAssembly.compileStreaming` and
`WebAssembly.instantiateStreaming`. Many, many reliability improvements.

Jarred Sumner · August 10, 2025

[](/blog/bun-v1.2.19)

Bun v1.2.19

Fixes 163 issues (addressing 1,043 👍). Introduces
`bun install --linker=isolated` for pnpm-style isolated node_modules,
`bun why` for dependency tree understanding, `bun pm pkg` commands for
package.json management, enhanced `node:fs.glob()` with array patterns
and exclude options, `node:module.SourceMap` class and
`findSourceMap()`, smarter `@types/bun` with automatic Node.js
compatibility detection, plus numerous Node.js compatibility
improvements, runtime bugfixes, bundler enhancements, and bun install
optimizations.

Jarred Sumner · July 19, 2025

[](/blog/bun-v1.2.18)

Bun v1.2.18

Fixes 52 issues (addressing 112 👍). ReadableStream text(), json(),
bytes(), blob(), WebSocket client compression with `permessage-deflate`,
`bun pm version`, reduced memory usage for large `fetch()` and `S3`
uploads, faster `napi_create_buffer`, faster sliced string handling in
native addons, `fs.glob` now matches directories by default,
`net.createConnection()` now validates `options.host`,
`http.ClientRequest#flushHeaders` now correctly sends the request body,
`net.connect` `keepAlive` and `keepAliveInitialDelay` options are now
correctly handled, `fs.watchFile` emits `stop` on next tick,
`net.Server` handles promise rejections in connection listeners,
`net.connect` throws `ERR_INVALID_IP_ADDRESS` for non-string lookup
results, `fs.watchFile` now ignores access time, `bun:sqlite` updated to
SQLite 3.50.2, Bun now reports as Node.js v24.3.0, `bun test` now fails
when no tests match a filter, `bun install` is faster for packages using
`node-gyp`, `Math.sumPrecise` is now available, Node.js compatibility
improvements.

Jarred Sumner · July 3, 2025

[](/blog/bun-v1.2.17)

Bun v1.2.17

Fixes 50 issues (addressing 80 👍), and adds +24 passing Node.js tests.
Ahead-of-time bundling for HTML imports, more reliable Bun Shell,
`setTimeout` and `setImmediate` use 8-15% less memory, `columnTypes` &
`declaredTypes` in `bun:sqlite`, `bun info` is the new `bun pm view`,
Node.js compatibility improvements to `child_process.fork`, `fs.glob`
options are now optional, `tls.getCACertificates()` is now implemented,
`bun init` generates `CLAUDE.md` when claude code is installed, and
more.

Dylan Conway · June 21, 2025

[](/blog/bun-v1.2.16)

Bun v1.2.16

Fixes 73 issues (addressing 124 👍), and adds +119 passing Node.js
tests. Serve files with `Bun.serve` through `routes`,
`install.linkWorkspacePackages` option in `bunfig.toml`, `bun outdated`
support for catalogs, `Bun.hash.rapidhash`, `node:net` compatibility,
`vm.SyntheticModule` support, `HTTPParser` binding, and memory leak
fixes.

Dylan Conway · June 11, 2025

[](/blog/bun-v1.2.15)

Bun v1.2.15

Fixes 11 issues (addressing 261 👍). bun audit scans dependencies for
security vulnerabilities, bun pm view shows package metadata from npm.
bun init adds a Cursor rule to use Bun instead of Node.js/Vite/npm/pnpm,
`node:vm SourceTextModule` and `node:perf_hooks createHistogram` are now
implemented.

Jarred Sumner · May 28, 2025

[](/blog/bun-v1.2.14)

Bun v1.2.14

Fixes 39 issues (addressing 179 👍). Adds support for catalogs in bun
install, --react flag for bun init, improved HTTP routing with
method-specific routes, better TypeScript default module settings, fixes
a HTTPS proxy issue impacting Codex, and several Node.js compatibility
improvements including worker_threads reliability enhancements and http2
server improvements.

Jarred Sumner · May 21, 2025

[](/blog/bun-v1.2.13)

Bun v1.2.13

Fixes 17 bugs (addressing 28 👍, adding +28 passing Node.js tests).
Improved --hot stability and Node.js compatibility. Better
`node:worker_threads` support, environment data, reduced memory usage
for child_process IPC and worker_threads. Fixes BOM handling in `.npmrc`
files. Fixes hot reloading on older versions of chrome

Aidan Bloore · May 9, 2025

[](/blog/bun-v1.2.12)

Bun v1.2.12

Stream browser console logs to your terminal, dev server uses less
memory. Node.js compatibility improvements for timers, vm, net, http,
and bugfixes for TextDecoder, hot reloading reliability bugfixes

Jarred Sumner · May 4, 2025

[](/blog/bun-v1.2.11)

Bun v1.2.11

Fixes 62 bugs (addressing 77 👍). `process.ref` and `process.unref`,
`util.parseArgs` negative options, `BUN_INSPECT_PRELOAD`, Highway SIMD,
Node.js compatibility improvements for `node:http`, `node:https`,
`node:tls`, and `node:readline`

Dylan Conway · April 29, 2025

[](/blog/bun-v1.2.10)

Bun v1.2.10

Fixes 33 bugs (addressing 33 👍). setImmediate gets faster. Reliability
improvements for filesystem operations. Fixes test.failing with done
callbacks. Fixes default idle timeout in Redis client. Fixes importing
from 'bun' module with bytecode compilation. Default Docker image
updated to Debian Bookworm.

Jarred Sumner · April 17, 2025

[](/blog/bun-v1.2.9)

Bun v1.2.9

Fixes 48 bugs (addressing 116 👍). Bun.redis is a builtin Redis client
for Bun. `ListObjectsV2` support in `Bun.S3Client`, more `libuv`
symbols, `require.extensions` & require.resolve paths, bugfixes in
`node:http`, `AsyncLocalStorage`, and `node:crypto`. Shell reliability
improvements.

Dylan Conway · April 9, 2025

[](/blog/debugging-memory-leaks)

Debugging JavaScript Memory Leaks

Use v8 heap snapshot API + Chrome DevTools to compare heap snapshots

Jarred Sumner · April 2, 2025

[](/blog/bun-v1.2.8)

Bun v1.2.8

Fixes 18 bugs (addressing 18 👍). napi improvements make node-sdl 100x
faster. Headers.get() is 2x faster. Several node:http bugfixes. node:fs
improvements. `bun install --frozen-lockfile` now works with
`overrides`. `bun pack` now handles directory-specific pattern
exclusion.

Jarred Sumner · March 31, 2025

[](/blog/bun-v1.2.7)

Bun v1.2.7

Fixes 35 bugs (addressing 219 👍). Bun.CookieMap is a Map-like API for
getting & setting cookies. Bun's TypeScript type declarations have been
rewritten to eliminate conflicts with Node.js and DOM type definitions.
Several bugfixes for node:http, node:crypto, and node:vm.

Jarred Sumner · March 27, 2025

[](/blog/bun-v1.2.6)

Bun v1.2.6

Fixes 74 bugs (addressing 36 👍). Faster, more compatible node:crypto.
`timeout` option in Bun.spawn. Support for `module.children` in
`node:module`. Connect to PostgreSQL via unix sockets with `Bun.SQL`.
Dev Server stability improvements. vm.compileFunction. Initial support
for node:test. Faster Express & Fastify.

Jarred Sumner · March 25, 2025

[](/blog/bun-v1.2.5)

Bun v1.2.5

Fixes 75 bugs (addressing 162 👍), and adds +69 passing Node.js tests.
Improvements to the frontend dev server, CSS modules support, a full
rewrite of Node-API, faster `Sign`, `Verify`, `Hash`, `Hmac` from
`node:crypto`, and bug fixes for `node:net` and Bun's bundler

Jarred Sumner · March 11, 2025

[](/blog/bun-v1.2.4)

Bun v1.2.4

Up to 60% faster Bun.build on macOS, codesigning support for single-file
executables on macOS, dev server stability improvements, fixes a
regression from v1.2.3 affecting Hono, fixes up/down buttons in
`bun init` on Windows, and improves Node.js compatibility

Jarred Sumner · February 26, 2025

[](/blog/bun-v1.2.3)

Bun v1.2.3

Fixes 128 bugs (addressing 349 👍). Bun gets a full-featured frontend
development toolchain with incredibly fast hot reloading and bundling.
Built-in routing for Bun.serve() makes it simpler to build web
applications. Bun.SQL gets sql.array, sql fragments, sql.file, and many
bugfixes. Node.js compatibility improvements for Buffer & Node-API
(napi).

Jarred Sumner · February 22, 2025

[](/blog/bun-v1.2.2)

Bun v1.2.2

Fixes 38 bugs (addressing 129 👍). JavaScript idle memory usage drops by
10–30%. Fixes regression impacting vite build. Reliability improvements
to Bun.SQL, Bun.S3Client, Bun's CSS parser. Node.js compatibility
improvements: fs.glob, fs.globSync, fs.promises.glob, bugfixes for
fs.Dir, fs.accessSync, node:http WebSocket exports

Jarred Sumner · February 1, 2025

[](/blog/bun-v1.2.1)

Bun v1.2.1

Fixes 32 bugs. S3 storage class support, X25519 support in
crypto.generateKeyPair. fs.stat uses less memory. node:fs,
node:child_process, and node:process compatibility improvements. CSS
parser bugfixes. Zero-filled buffers with `--zero-fill-buffers` flag.
Indented inline snapshots in bun test. Improved: String GC Reporting
Accuracy. Fixed memory leaks in Bun.serve(), Bun.pathToFileURL, and with
some strings. Disable minification in HTML imports.

Jarred Sumner · January 27, 2025

[](/blog/bun-v1.2)

Bun 1.2

Built-in Postgres client with Bun.sql, built-in S3 object support with
Bun.s3, a new text-based lockfile: bun.lock, Express is 3x faster, and a
major update on Node.js compatibility.

Ashcon Partovi · January 22, 2025

[](/blog/bun-v1.1.45)

Bun v1.1.45

Fixes 13 bugs (addressing 50 👍) 82% of the 'streams' module tests from
Node.js' test suite now pass in Bun. node:crypto now implements
diffieHellman, getCipherInfo, and X509Certificate. The
checkServerIdentity object now matches Node.js' format. Bugfixes in
Bun's fs.mkdirSync method, importing with query string

Jarred Sumner · January 17, 2025

[](/blog/bun-v1.1.44)

Bun v1.1.44

Fixes 43 bugs. \> 90% of the 'dns' module tests from Node.js' test suite
now pass in Bun. You can now build frontend applications on-demand in
Bun.serve(). Adds xxHash support to Bun.hash(). Adds support for reading
bun.lock using import. Adds --no-install flag to bunx. Improves
console.warn output. Fixes bugs in Bun's S3 client, HTMLRewriter, and
node:fs compatibility improvements, and more.

Ben Grant · January 16, 2025

[](/blog/bun-v1.1.43)

Bun v1.1.43

Bun v1.1.43 introduces 1st-class S3 support, HTML bundling, bun install
--filter, V8 heap snapshots, bun install --lockfile-only, bun add
--peer, 100% of `node:path` module Node.js test pass, 98% of `node:zlib`
tests pass, Bun.file(path).stat(), Bun.file(path).delete(), along with
many reliability improvements and bugfixes.

Jarred Sumner · January 8, 2025

[](/blog/bun-v1.1.42)

Bun v1.1.42

Fixes 1 regression from Bun v1.1.41 involving 'use strict' in empty
files that impacted Prisma

Jarred Sumner · December 21, 2024

[](/blog/bun-v1.1.41)

Bun v1.1.41

Fixes 12 bugs (addressing 196 👍). JSDOM & happy-dom work more reliably
due to node:vm compatibility improvements. Single-file executables on
Windows now support custom icons and hiding the console. throw: true in
Bun.build throws an error instead of returning errors. bun install gets
the --omit=dev\|optional\|peer options and in .npmrc. A hoisting
edgecase in bun install is fixed

Jarred Sumner · December 20, 2024

[](/blog/bun-v1.1.40)

Bun v1.1.40

Fixes 14 bugs. Adds `install.saveTextLockfile` option in bunfig.toml to
make the text lockfile format the default. Fixes a bug in
package-lock.json migration when manually changing npm URLs to git URLs.
Fixes a regression in process.on from Bun v1.1.39. Fixes an issue with
BigInt in napi APIs impacting DuckDB. Improves reliability of installing
git/github dependencies in bun install. Upgrades WebKit with security &
reliability improvements.

Jarred Sumner · December 18, 2024

[](/blog/bun-lock-text-lockfile)

Bun's new text-based lockfile

We're introducing a new text-based lockfile format for bun install. It's
GitHub & git-friendly, and makes it easier for teams to migrate from
npm, yarn, and pnpm to bun install

Jarred Sumner · December 17, 2024

[](/blog/bun-v1.1.39)

Bun v1.1.39

Fixes 61 bugs (addressing 99 👍). bun.lock is bun's new text-based
lockfile. Cached bun install gets 30% faster. fetch() request body
streams. 100% of string_decoder, punycode and querystring Node.js tests
pass. Native plugin API for Bun.build() and Rust crate. Implemented
expect().toMatchInlineSnapshot(). More accurate heap snapshots.
WebSocket server memory reduction. WebKit Upgrade with Error.isError,
faster String.prototype.at.

Jarred Sumner · December 17, 2024

[](/blog/bun-v1.1.38)

Bun v1.1.38

Fixes 7 bugs (addressing 14 👍). Fixes debugger bug causing Bun to hang
in VSCode terminal. Fixes crash in `postgres` npm package. Fixes
TypeScript minification bug, console.log improvements, updated SQLite
and c-ares. Adds `reusePort` option to `Bun.listen` and `node:net`, and
fixes a bug in `Bun.FileSystemRouter.reload()`. Fixes rare crash in
`fetch()`. Fixes assertion failure when re-assigning global modules in
`--print` or `--eval`

Jarred Sumner · November 29, 2024

[](/blog/bun-v1.1.37)

Bun v1.1.37

Fixes 62 bugs (addressing 109 👍). Realtime error reporting & test
runner in VSCode, self-referencing `import` & `require`, package.json
`config` environment variables, improved CSS parser errors, fixes for a
Prisma memory leak, a `bun install` regression, `expect.toMatchSnapshot`
quote escaping, `node:net` regression impacting `http2-wrapper`, and
several other bugfixes and improvements.

Dylan Conway · November 26, 2024

[](/blog/bun-v1.1.35)

Bun v1.1.35

Fixes 46 bugs (addressing 570 👍). Musl LibC support, JUnit XML test
output, 6 MB binary size reduction, `Worker` `preload` option, faster
`fs.readFile` for small files, deep equality support for `Headers` and
`URLSearchParams`, `node:net` compatibility improvements, unicode import
bugfixes, several `bun install` fixes

Dylan Conway · November 19, 2024

[](/blog/how-bun-supports-v8-apis-without-using-v8-part-2)

How Bun supports V8 APIs without using V8 (part 2)

Rearranging JavaScriptCore types to match V8 code's assumptions

Ben Grant · November 5, 2024

[](/blog/bun-v1.1.34)

Bun v1.1.34

Fixes 64 bugs (addressing 62 👍). `Bun.randomUUIDv7`, reduced memory
usage for long-running processes, `napi_type_tag_object` and
`napi_check_object_type_tag`, redacted secrets from config logs,
`ReadableStream` and HTTP/1.1 spec fixes, several `bun install` and
Node.js compatibility improvements

Dylan Conway · November 2, 2024

[](/blog/bun-v1.1.33)

Bun v1.1.33

Fixes 8 bugs. Fixes JSX symbol collisions with imports, fixes a
`bun install` slowdown, skips optional dependency, fixes optional
dependency postinstall failures, and fixes an assertion failure in the
CSS parser.

Dylan Conway · October 24, 2024

[](/blog/bun-v1.1.32)

Bun v1.1.32

Fixes 7 bugs and adds crypto.hash() in node:crypto. Warnings in Next.js
15 are fixed. A regression in ESM \<\> CJS interop impacting Vite config
objects is fixed. A crash in Bun.serve() when passing invalid TLS
certificates at start is fixed.

Jarred Sumner · October 21, 2024

[](/blog/bun-v1.1.31)

Bun v1.1.31

Fixes 41 bugs (addressing 595 👍). node:http2 server and gRPC server
support, ca and cafile support in bun install, Bun.inspect.table, bun
build --drop, iterable SQLite queries, iterator helpers, Promise.try,
Buffer.copyBytesFrom, and several Node.js compatibility improvements and
bugfixes.

Ashcon Partovi · October 18, 2024

[](/blog/bun-v1.1.30)

Bun v1.1.30

Fixes 57 bugs (addressing 150 👍) Bun's CSS bundler is here (and very
experimental). `bun publish` is a drop-in replacement for `npm publish`.
`bun build --bytecode` compiles to bytecode leading to 2x faster start
time. Bun.color() formats and normalizes colors. `bun pm whoami` prints
your npm username. bun install reads \$HOME/.npmrc. bun build
--format=cjs outputs CommonJS modules. bun build --target=node now
supports Node.js native addons. 30x faster crypto.privateEncrypt() &
crypto.publicDecrypt(). Zombie process killer in bun test. --banner and
--footer options in bun build. Bun.serve().stop() returns a promise.
CryptoHasher now supports HMAC. Bugfixes for node:zlib, node:module, and
node:buffer. Lots more bugfixes and Node.js compatibility improvements.

Jarred Sumner · October 8, 2024

[](/blog/how-bun-supports-v8-apis-without-using-v8-part-1)

How Bun supports V8 APIs without using V8 (part 1)

Bun is built on JavaScriptCore, not V8. Here's how we're implementing
V8's C++ API on top of it.

Ben Grant · September 30, 2024

[](/blog/bun-v1.1.29)

Bun v1.1.29

Fixes 21 bugs (addressing 22 👍). Fixes issue with proxying requests
with axios. N-API (napi) support in compiled C. Faster typed arrays in
`bun:ffi` with 'buffer' type. Fixes issue with bundling imported .wasm
files in `bun build`. Fixes regression with `bun build` from v1.1.28.
Fixes snapshot regression in `bun test`. Fixes bug with argument
validation in `Bun.build` plugins.

Jarred Sumner · September 20, 2024

[](/blog/bun-v1.1.28)

Bun v1.1.28

This release fixes 40 bugs (addressing 51 👍). Compile & run C from
JavaScript. 30x faster path.resolve. Named pipes on Windows. Several
Node.js compatibility improvements and bugfixes.

Dylan Conway · September 18, 2024

[](/blog/compile-and-run-c-in-js)

Compile and run C in JavaScript

Bun now supports compiling and running C from JavaScript to make using
systems libraries easier.

Jarred Sumner · September 18, 2024

[](/blog/bun-v1.1.27)

Bun v1.1.27

Fixes 130 bugs (addressing 250 👍). `bun pm pack`, faster `node:zlib`.
Static routes in Bun.serve(). ReadableStream support in response.clone()
& request.clone(). Per-request timeouts. Cancel method in ReadableStream
is called. `bun run` handles CTRL + C better. Faster buffered streams.
`bun outdated` package filtering. Watch arbtirary file types in --hot
and --watch. Fixes to proxies on Windows. Several Node.js compatibility
improvements.

Chloe Caruso · September 7, 2024

[](/blog/bun-v1.1.26)

Bun v1.1.26

Fixes 9 bugs (addressing 652 👍). `bun outdated` shows outdated
dependencies. Bun.serve() gets a new `idleTimeout` option and
`subscriberCount(topic: string)` for a count of websocket clients
subscribed to a topic. `test.only` in bun:test now skips tests before
and after instead of only after. expect.assertions(n) in async functionn
is fixed. "use strict" in CommonJS modules is now preserved. Plus, more
node.js compatibility improvements and bug fixes.

Jarred Sumner · August 24, 2024

[](/blog/bun-v1.1.25)

Bun v1.1.25

Fixes 42 bugs (addressing 470 👍). Support for `node:cluster` and V8's
C++ API. Faster WebAssembly on Windows with OMGJIT. 5x faster S3 uploads
with @aws-sdk/client-s3, Worker in standalone executables, and more
Node.js compatibility improvements and bug fixes.

Ashcon Partovi · August 21, 2024

[](/blog/bun-v1.1.23)

Bun v1.1.23

Support for `TextEncoderStream` and `TextDecoderStream`, 50% faster
`console.log(string)`, support for `Float16Array`, Node.js\<\>Bun IPC on
Windows, truncate large arrays in `console.log()`, and many more bug
fixes and Node.js compatibility improvements.

Ashcon Partovi · August 14, 2024

[](/blog/bun-v1.1.22)

Bun v1.1.22

Fixes 79 bugs (addressing 93 👍). Express is now 3x faster in Bun, ES
modules load faster on Windows, 10% faster Bun.serve() at POST requests,
source maps in `bun build --compile`. Memory usage reductions to
`--hot`, Bun.serve(), and imports. `Uint8Array.toBase64()`,
`Uint8Array.toHex()`, and lots of Node.js compatibiltiy improvements and
bugfixes.

Ashcon Partovi · August 7, 2024

[](/blog/bun-v1.1.21)

Bun v1.1.21

Fixes 72 bugs (addressing 63 👍). 30% faster fetch() decompression, New
--fetch-preconnect flag, improved Remix support, Bun gets 4 MB smaller
on Linux, bundle packages excluding dependencies, many bundler fixes and
node compatibility improvements.

Ashcon Partovi · July 27, 2024

[](/blog/bun-v1.1.19)

Bun v1.1.19

Fixes 54 bugs (addressing 248 👍). JavaScript gets faster on Windows.
Raspberry Pi 4 support returns. `_auth` support in `.npmrc`.
`bun install` preserves package.json indentation. `aws-cdk-lib` support.
Memory leak fixes in `new Response(request)` and `fs.readdir`. Several
reliability improvements. Statically linked UCRT on Windows.

Dylan Conway · July 12, 2024

[](/blog/bun-v1.1.18)

Bun v1.1.18

Fixes 55 bugs (addressing 493 👍). Support for npmrc. Improved constant
folding & enum inlining. Improved console.log output for functions.
Fixes patching dependencies in workspaces. Fixes several bugs in
`bun install` when linking binaries. Fixes a crash in `dns.lookup`, and
normalizes DNS errors to better match Node. Fixes an assertion failure
in `Bun.escapeHTML`. Upgrades JavaScriptCore.

Dylan Conway · July 3, 2024

[](/blog/bun-v1.1.17)

Bun v1.1.17

Fixes 4 bugs. Fixes a crash in bun repl. Makes fs.readdirSync 5% faster
on macOS in small directories. Fixes "ws" module not calling the
callback in "send" method. Fixes a crash when reading a corrupted
lockfile in bun install. Makes macOS event loop a smidge faster.

Jarred Sumner · June 25, 2024

[](/blog/bun-v1.1.16)

Bun v1.1.16

Fixes 41 bugs (addressing 221 👍). lcov code coverage reports in bun
test, Install dependencies from private git repositories including
Gitlab and Bitbucket. `Buffer.from(string, "base64")` gets 6x - 30x
faster on large input. Bug fixes to `bun patch`, `bun install`, N-AIi
addons, transpiler bugfixes and Node.js compatibility improvements.

Chloe Caruso · June 23, 2024

[](/blog/bun-v1.1.14)

Bun v1.1.14

Fixes 63 bugs (addressing 519 👍). Patch node_modules with bun patch.
ORM-less object mapping in bun:sqlite. Log fetch() calls as curl
commands. Node.js compatibility improvements. bun install bugfixes,
bun:sqlite bugfixes, WebSocket fixes, Windows bugfixes

Chloe Caruso · June 19, 2024

[](/blog/bun-v1.1.13)

Bun v1.1.13

Fixes 97 bugs (addressing 211 👍). Reliability improvements to bun
install, WebSocket server, fetch, worker_threads. worker_threads now
supports eval. `URL.createObjectURL`, stack trace error location
improvements, several `bun install` fixes, and many crashes and bugs
fixed.

Jarred Sumner · June 5, 2024

[](/blog/bun-v1.1.11)

Bun v1.1.11

Fixes 36 bugs (addressing 362 👍). bun update saves dependency updates
to package.json. A peer dependencies resolution bug in bun install is
fixed. setTimeout gets 5x faster throughput on Linux. Node.js
compatibility improvements to fs, zlib, timers, http, dgram and module.
Event loop improvements on macOS. Windows reliability improvements.

Jarred Sumner · June 1, 2024

[](/blog/bun-v1.1.10)

Bun v1.1.10

Fixes 20 bugs. 2x faster uncached bun install on Windows. fetch() uses
up to 2.8x less memory. Several bugfixes to bun install, sourcemaps,
Windows reliability improvements and Node.js compatibility improvements

Jarred Sumner · May 24, 2024

[](/blog/bun-v1.1.9)

Bun v1.1.9

Fixes 67 bugs (addressing 150 👍). Fixes to: workspaces in
`bun install`, sourcemaps in bun build, IPv6 & VPN connectivity, Loading
UNC paths, junctions, symlinks, and pnpm on Windows, `fetch()` gets
faster, added `dns.prefetch()` API, `atob()` gets 8x faster,
`toString('base64url')` gets 5x faster, expect().toBeReturned() matcher,
Node.js compatibility improvements, Bun Shell fixes, and lots more
bugfixes.

Ashcon Partovi · May 22, 2024

[](/blog/bun-v1.1.8)

Bun v1.1.8

Fixes 54 bugs (addressing 184 👍). Support for
`process.on("uncaughtException")` and
`process.on("unhandledRejection")`, Brotli support in `node:zlib`,
Faster JSON.parse, `[Symbol.dispose]` in Bun APIs, fixes lots of crashes
on Windows, and many other bugfixes.

Ashcon Partovi · May 10, 2024

[](/blog/bun-v1.1.7)

Bun v1.1.7

Fixes 28 bugs (addressing 11 👍). Glob workspace names in bun install.
Asymmetric matcher support in expect.extends() equals. bunx --version.
Bugfixes to JSX transpilation, sourcemaps, cross-compilation of
standalone executables, bun shell, RegExp, Worker on Windows, and
Node.js compatibility improvements.

Jarred Sumner · May 3, 2024

[](/blog/bun-v1.1.6)

Bun v1.1.6

Fixes 10 bugs (addressing 512 👍). UDP sockets! node:dgram! DataDog and
ClickHouseDB! Fixes a bug impacting SvelteKit. Array#sort gets 15% -
135% faster. Fixes a regression in node:http from v1.1.5. Node.js
compatibiltiy improvements and bugfixes.

Jarred Sumner · April 28, 2024

[](/blog/bun-v1.1.5)

Bun v1.1.5

Fixes 64 bugs (addressing 101 👍). Cross-compile standalone JavaScript &
TypeScript executables to other platforms with bun build --compile.
import any file as text via the `type: 'text'` import attribute.
Introduces a new crash reporter. package.json with comments and trailing
commas. Fixes a bug where `bun run --filter` exited with 0. Fixes a bug
in bun install with file: dependencies. Fixes bugs in `node:tls`,
`node:crypto`, `node:readline`, `node:http`, `node:worker_threads`

Ashcon Partovi · April 26, 2024

[](/blog/bun-report-is-buns-new-crash-reporter)

bun.report is Bun's new crash reporter

How we built an anonymous Zig/C++ crash reporter that doesn't require
debug symbols to be shipped with the application.

Chloe Caruso · April 26, 2024

[](/blog/bun-v1.1.4)

Bun v1.1.4

Fixes 40 bugs (addressing 84 👍 reactions).
`bun run --filter <workspace> <script>` lets you run multiple workspace
scripts in parallel. Reinstalls get up to 50% faster in bun install.
Important reliability fixes to bun install. bun:sqlite supports `using`
for resource cleanup and has a few bugfixes. Memory leak impacting
Next.js Standalone & Web Streams is fixed. Node.js compatibility
improvements to fs and child_process. Fix for 'Connection closed' in
fetch(). A few bugfixes for Bundows.

Jarred Sumner · April 16, 2024

[](/blog/bun-v1.1.3)

Bun v1.1.3

Fixes 5 bugs. bun install gets 50% faster on Windows. A bug that could
cause bun install to hang in rare cases has been fixed. A bug where some
errors in bun install would not produce a non-zero exit code has been
fixed. A bug that could cause specific certain combinations of
dependencies to fail to install has been fixed. A bug on Windows where
CTRL + C on cmd.exe after exiting bun would not behave as expected has
been fixed. A bug where missing permissions on Windows for reading a
directory could lead to a crash has been fixed.

Jarred Sumner · April 8, 2024

[](/blog/bun-v1.1.2)

Bun v1.1.2

Fixes 4 bugs (addressing 44 👍 reactions). EBUSY on Windows in vite dev,
next dev, and saving bun.lockb has been fixed. Object cloning gets 6x
faster. Bun Shell gets support for seq, yes, basename and dirname. A
TypeScript parsing edgecase has been fixed. A bug causing 'unreachable
code' errors has been fixed. fs.watch on Windows has been rewritten to
improve performance and reliability.

Jarred Sumner · April 6, 2024

[](/blog/bun-v1.1.1)

Bun v1.1.1

Fixes 20 bugs (addressing 60 👍). Add subshell and positional argument
support to Bun Shell, fixing an issue with bun install + sharp on
Windows. Printed source code in errors no longer fill up your terminal.
Upgrades JavaScriptCore, which includes performance improvements to
RegExp, typed arrays, String indexOf and String replace. Error objects
and JIT'd function calls use less memory. Fixes several bugs with bun
install on Windows. Fixes a bug with Bun.serve() on Windows. Fixes a
TOML parser bug impacting escape sequences and windows paths in .toml
files

Jarred Sumner · April 4, 2024

[](/blog/bun-v1.1)

Bun 1.1

Bun now supports Windows!

Jarred Sumner & Chloe Caruso & Ashcon Partovi & Dylan Conway & Meghan
Denny & Georgijs Vilums & Ciro Spaciari · April 1, 2024

[](/blog/bun-v1.0.36)

Bun v1.0.36

Fixes 13 bugs. Adds support for `fs.openAsBlob` and `fs.opendir`, fixes
edgecase in bun install with multiple `bin` entries in `package.json`,
fixes `bun build --target bun` encoding, fixes bug with process.stdin &
process.stdout, fixes bug in http2, fixes bug with .env in bun run,
fixes various Bun shell bugs and improves Node.js compatibility.

Ashcon Partovi · March 29, 2024

[](/blog/bun-v1.0.34)

Bun v1.0.34

Fixes 7 bugs, bunx uses less memory, bun install gets faster on Docker &
WSL, a conflicting devDependency hoisting bug is fixed, a cyclical
CommonJS & ESM import crash is fixed, cross-mount fs.cp & fs.copyFile
get 50% faster on Linux, and reliability improvements for bun install &
bun's runtime

Ashcon Partovi · March 22, 2024

[](/blog/bun-v1.0.32)

Bun v1.0.32

Fixes 13 bugs, including 3 I/O regressions from v1.0.31. Improves
Node.js compatibility. 'ws' package can now send & receive ping/pong
events. util.promisify'd setTimeout setInterval, setImmediate work now.
FileHandle methods have been implemented

Jarred Sumner · March 17, 2024

[](/blog/bun-v1.0.33)

Bun v1.0.33

Fixes 2 bugs, including a bug with the mv command in Bun Shell and an
error in node:crypto when verifying signatures

Jarred Sumner · March 17, 2024

[](/blog/bun-v1.0.31)

Bun v1.0.31

Fixes 54 bugs (addresses 113 👍 reactions), introduces `bun --print`,
`<stdin> | bun run -`, `bun add --trust`, `fetch()` with Unix sockets,
fixes macOS binary size regression, fixes high CPU usage bug in spawn()
on older linux, adds `util.styleText`, Node.js compatibiltiy
improvements, bun install bugfixes, and bunx bugfixes

Ashcon Partovi · March 14, 2024

[](/blog/bun-v1.0.30)

Bun v1.0.30

Fixes 27 bugs (addressing 103 👍 reactions), fixes an 8x perf regression
in Bun.serve(), adds a new `--conditions` flag to `bun build` and Bun's
runtime, adds support for `expect.assertions()` and
`expect.hasAssertions()` in Bun's test runner, fixes crashes and
improves Node.js compatibility

Jarred Sumner · March 4, 2024

[](/blog/bun-v1.0.29)

Bun v1.0.29

Fixes 8 bugs. Bun.stringWidth(a) is a ~6,756x faster drop-in replacement
for the popular 'string-width' package. bunx checks for updates more
frequently. Adds expect().toBeOneOf() in bun:test. Memory leak impacting
Prisma is fixed. Shell now supports advanced redirects like '2\>&1',
'&\>'. Reliability improvements to bunx, bun install, WebSocket client,
and Bun Shell

Jarred Sumner · February 23, 2024

[](/blog/bun-v1.0.28)

Bun v1.0.28

Fixes 6 bugs (addressing 26 👍 reactions). Fixes bugs impacting Prisma
and Astro, 'events', 'readline', and 'http2'. Fixes a bug in Bun Shell
involving stdin redirection and fixes bugs in bun:test with test.each
and describe.only

Jarred Sumner · February 19, 2024

[](/blog/bun-v1.0.27)

Bun v1.0.27

Fixes 72 bugs (addressing 192 👍 reactions), Bun Shell supports throwing
on non-zero exit codes, stream Response bodies using async generators,
improves reliability of fetch(), http2 client, Bun.Glob and Bun Shell.
Fixes a regression with bun --watch on Linux. Improves Node.js
compatibility

Jarred Sumner · February 17, 2024

[](/blog/bun-v1.0.26)

Bun v1.0.26

Fixes 30 bugs (addressing 60 👍 reactions), adds support for
multi-statement queries in bun:sqlite, makes bun --watch more reliable
in longer-running sessions, Bun.FileSystemRouter now supports more than
64 routes, fixes a bug with expect().toStrictEqual(), fixes 2 bugs with
error.stack, improves Node.js compatibility

Jarred Sumner · February 3, 2024

[](/blog/bun-v1.0.25)

Bun v1.0.25

Fixes 4 bugs, adds vm.createScript. Fixes a crash in fs.readFile, a
crash in Bun.file().text(), a crash in IPC, and a transpiler bug
involving loose equals

Jarred Sumner · January 21, 2024

[](/blog/bun-v1.0.24)

Bun v1.0.24

Fixes 9 bugs and adds Bun Shell, a fast cross-platform shell with
seamless JavaScript interop. Fixes a socket timeout bug, a potential
crash when socket closes, a Node.js compatibility issue with Hapi, a
process.exit bug, and bun install binlinking bug, bun inspect
regression, and bun:test expect().toContain bug

Jarred Sumner · January 20, 2024

[](/blog/the-bun-shell)

The Bun Shell

The Bun Shell is a cross-platform shell that allows you to run shell
scripts in JavaScript & TypeScript

Jarred Sumner · January 20, 2024

[](/blog/bun-v1.0.23)

Bun v1.0.23

Fixes 40 bugs (addressing 194 👍 reactions). import & embed sqlite
databases in Bun, Resource Management ('using' TC39 stage3) support,
bundler improvements when building for Node.js, bugfix for peer
dependency resolution, semver bugfix, 4% faster TCP on linux, Node.js
compatibility improvements and more

Jarred Sumner · January 16, 2024

[](/blog/bun-v1.0.22)

Bun v1.0.22

Fixes `bun install` issues on Vercel, adds `performance.mark()` APIs,
adds `child_process` support for extra pipes, makes `Buffer.concat`
10% - 400% faster, adds `toBeEmptyObject` and `toContainKeys` matchers,
fixes `console.table` width using emojis, and adds support for `argv`
and `execArgv` options in `worker_threads`.

Ashcon Partovi · January 9, 2024

[](/blog/bun-v1.0.21)

Bun v1.0.21

Fixes 33 bugs (addressing 80 👍 reactions). console.table() support.
Bun.write, Bun.file, and bun:sqlite use less memory. Large file uploads
with FormData use less memory. bun:sqlite error messages get more
detailed. Memory leak in errors from node:fs fixed. Node.js
compatibility improvements, and many crashes fixed

Jarred Sumner · January 2, 2024

[](/blog/bun-v1.0.20)

Bun v1.0.20

Reduced memory usage for fs.readlink, fs.writeFile, fs.stat and
HTMLRewriter. Fixes a regression where setTimeout caused high CPU usage
on Linux. Bun.spawn shows how much CPU & memory the process used.
HTMLRewriter.transform now supports strings and ArrayBuffer.
fs.writeFile() and fs.readFile() now support hex & base64 encodings

Jarred Sumner · December 24, 2023

[](/blog/bun-v1.0.19)

Bun v1.0.19

Fixes 26 bugs (addressing 92 👍 reactions). Use @types/bun instead of
bun-types. Fixes --frozen-lockfile bug. bcrypt & argon2 packages now
work. setTimeout & setInterval get 4x higher throughput. module mocks in
bun:test resolve specifiers. Optimized spawnSync() for large stdio on
Linux. Bun.peek() gets 90x faster, expect(map1).toEqual(map2) gets 100x
faster. Bugfixes to NAPI, bun install, and Node.js compatibility
improvements

Jarred Sumner · December 22, 2023

[](/blog/bun-v1.0.18)

Bun v1.0.18

Fixes 27 bugs (addressing 28 👍 reactions). A hang impacting create-vite
& create-next & stdin has been fixed. Lifecycle scripts reporting "node"
or "node-gyp" not found has been fixed. expect().rejects works like Jest
now, and more bug fixes

Jarred Sumner · December 14, 2023

[](/blog/bun-v1.0.17)

Bun v1.0.17

Fixes 15 bugs (addressing 152 👍 reactions). bun install postinstall
scripts run for top 500 packages, `bunx supabase` starts 30x faster than
`npx supabase`, `bunx esbuild` starts 50x faster than `npx esbuild` and
bugfixes to bun install

Jarred Sumner · December 12, 2023

[](/blog/bun-v1.0.16)

Bun v1.0.16

Fixes 49 bugs (addressing 38 👍 reactions). Concurrent IO for Bun.file &
Bun.write gets 3x faster and now supports Google Cloud Run & Vercel,
Bun.write auto-creates the parent directory if it doesn't exist,
`expect.extend` inside of preload works, `napi_create_object` gets 2.5x
faster, bugfix for module resolution impacting Astro v4 and p-limit,
console.log bugfixes

Jarred Sumner · December 10, 2023

[](/blog/bun-v1.0.15)

Bun v1.0.15

Fixes 23 bugs (addressing 117 👍 reactions), `tsc` starts 2x faster,
prettier 40% faster. Stable `WebSocket` client, syntax-highlighted
errors, cleaner stack traces, add custom test matchers with
`expect.extend()`, TensorFlow.js support, fs.readdir recursive is 40x
faster than in Node, and more.

Ashcon Partovi · December 2, 2023

[](/blog/bun-v1.0.14)

Bun v1.0.14

Introduces `Bun.Glob` - a fast glob matcher, fixes a race condition when
extracting dependencies during `bun install`, improves TypeScript module
resolution in `node_modules`, makes error messages easier to read, and
more fixes.

Ashcon Partovi · November 22, 2023

[](/blog/bun-v1.0.13)

Bun v1.0.13

Fixes 6 bugs (addressing 317 👍 reactions). 'http2' module & gRPC.js
work now. Vite 5 & Rollup 4 work. Implements process.report.getReport(),
improves support for ES5 'with' statements, fixes a regression in bun
install, fixes a crash when printing exceptions, fixes a Bun.spawn bug,
and fixes a peer dependencies bug

Jarred Sumner · November 18, 2023

[](/blog/bun-v1.0.12)

Bun v1.0.12

Fixes 24 bugs, makes bun's CLI help menus easier to read, adds support
for `bun -e`, `bun --env-file`, `server.url`, `import.meta.env`,
`expect.unreachable()`, improves support for module mocks, allows
importing builtin modules at bundle-time, and improves Node.js
compatibility

Ashcon Partovi · November 16, 2023

[](/blog/bun-v1.0.11)

Bun v1.0.11

Fixes 5 bugs, adds Bun.semver, fixes a bug in bun install and fixes bugs
impacting astro and @google-cloud/storage

Jarred Sumner · November 8, 2023

[](/blog/bun-v1.0.10)

Bun v1.0.10

Fixes 14 bugs (addressing 102 👍 reactions), node:http gets 14% faster,
stability improvements to Bun for Linux ARM64, bun install bugfix, and
node:http bugfixes

Jarred Sumner · November 7, 2023

[](/blog/bun-v1.0.9)

Bun v1.0.9

Fixes a glibc symbol version error, illegal instruction error, a
Bun.spawn bug, an edgecase with peer dependency installs, and a JSX
transpiler bugfix

Jarred Sumner · November 5, 2023

[](/blog/bun-v1.0.8)

Bun v1.0.8

Fixes 138 bugs (addressing 257 👍 reactions), makes `require()` use 30%
less memory, adds module mocking to `bun test`, and improves reliability
of `bun install` and Bun.spawn

Ashcon Partovi · November 2, 2023

[](/blog/bun-v1.0.7)

Bun v1.0.7

Fixes 59 bugs (addressing 78 👍 reactions), support for optional
`peerDependencies`, improvements to Node.js compatibility, and more
stability for `bun install`.

Jarred Sumner · October 20, 2023

[](/blog/bun-v1.0.6)

Bun v1.0.6

Fixes 3 bugs (addressing 85 👍 reactions), implements support for the
'overrides' & 'resolutions' fields in package.json and a regression
impacting Docker usage with Bun

Jarred Sumner · October 11, 2023

[](/blog/bun-v1.0.5)

Bun v1.0.5

Fixes 41 bugs + a memory leak in `fetch()`, `KeyObject` support in
`crypto` module, import package-lock.json, install peer dependencies,
bun pm migrate subcommand, bun install bugfixes, and
`toEqualIgnoringWhitespace` matcher in `bun:test`

Ashcon Partovi · October 11, 2023

[](/blog/bun-v1.0.4)

Bun v1.0.4

Bun v1.0.4 fixes 62 bugs, adds `server.requestIP`, supports virtual
modules in runtime plugins, fixes many bun install bugs, and reduces
memory consumption in `Bun.serve()`

Jarred Sumner · October 3, 2023

[](/blog/bun-v1.0.3)

Bun v1.0.3

emitDecoratorMetadata, Nest.js support, better private registry support,
bunx bugfix, console.log() depth 8 -\> 2, silence .env loaded message,
and many bugfixes

Colin McDonnell · September 22, 2023

[](/blog/bun-v1.0.2)

Bun v1.0.2

Faster watch mode, bunx @latest bugfix, Bun uses V8's date parser,
bugfixes to node:readline, node:dns, bun run, http streaming,
transpiler, node:tty and more

Jarred Sumner · September 15, 2023

[](/blog/bun-v1.0.1)

Bun v1.0.1

Named imports for .json & .toml files, bugfixes to bun install,
node:path, Buffer. Support for NODE_TLS_REJECT_UNAUTHORIZED=0

Jarred Sumner · September 12, 2023

[](/blog/bun-v1.0)

Bun 1.0

Bun is stable and ready for production.

Jarred Sumner & Ashcon Partovi & Colin McDonnell · September 8, 2023

[](/blog/bun-v0.8.1)

Bun v0.8.1

Bun.serve() gets unix domain socket support, a performance regression
has been fixed, and bugs in bun install, node:http and napi have been
fixed

Jarred Sumner · August 26, 2023

[](/blog/bun-v0.8.0)

Bun v0.8.0

Bun gets debugger support, fetch streaming, and `bun update`. ReadStream
and WriteStream from `node:tty` are implemented, including raw mode on
`process.stdin`. SvelteKit now works in Bun. Plus bug fixes and
stability improvements.

Colin McDonnell · August 24, 2023

[](/blog/bun-v0.7.3)

Bun v0.7.3

bun test gets code coverage reporting. bun test -t pattern lets you
filter tests to run by regex pattern. Fixes a crash in bun:sqlite,
node.js compatiblity improvements, and more.

Jarred Sumner · August 6, 2023

[](/blog/bun-v0.7.2)

Bun v0.7.2

worker_threads, BroadcastChannel, diagnostics_channel, Node.js
compatibility improvements, bun ., bugfixes and 2 memory leaks fixed

Jarred Sumner · August 3, 2023

[](/blog/bun-v0.7.1)

Bun v0.7.1

Node.js compatibility improvements for Vite & frameworks. Better
workspace support in bun installl & a bugfix for private registries. ES
Modules load 30% faster. `vite dev` starts 90% faster on Linux.
Transpiler bugfixes. fetch() file urls. MessagePort & MessageChannel.

Jarred Sumner · July 29, 2023

[](/blog/bun-v0.7.0)

Bun v0.7.0

vite dev support, Workers, bun --smol, WebSocket.ping(),
structuredClone(), serialize() and better Node.js compatibility

Jarred Sumner · July 21, 2023

[](/blog/bun-v0.6.14)

Bun v0.6.14

Better support for Node.js `process` and lots of fixed bugs.

Ashcon Partovi · July 12, 2023

[](/blog/bun-v0.6.13)

Bun v0.6.13

Mock `Date`, 10x faster base64 encoding, WebSocket client reliability
improvements, support for postgres tls and nodemailer, and various bug
fixes.

Ashcon Partovi · July 4, 2023

[](/blog/commonjs-is-not-going-away)

CommonJS is not going away

CommonJS is here to stay, and that's okay. Better tooling will solve
today's developer experience issues.

Jarred Sumner · June 30, 2023

[](/blog/bun-v0.6.12)

Bun v0.6.12

Sourcemapped stack traces at runtime, Node.js compatibility
improvements, Bun.file(path).exists, minifier and bundler bugfixes

Jarred Sumner · June 30, 2023

[](/blog/bun-v0.6.10)

Bun v0.6.10

Reduced memory usage, fs.watch(), better CommonJS support, improvements
to `bun test`, `bun install`, and many bugfixes.

Ashcon Partovi · June 26, 2023

[](/blog/bun-v0.6.9)

Bun v0.6.9

Memory usage reductions in Bun.serve(), bun install, module imports, and
crypto hashing. Crash in CommonJS modules fixed, plus additional
bugfixes.

Jarred Sumner · June 13, 2023

[](/blog/bun-v0.6.8)

Bun v0.6.8

Password hashing, function mocking, debugging, and bug squashing

Colin McDonnell · June 9, 2023

[](/blog/bun-v0.6.7)

Bun v0.6.7

Discord.js, Prisma, and Puppeteer support, faster node:crypto hashing,
and lots of bug fixes

Jarred Sumner · June 2, 2023

[](/blog/bun-v0.6.6)

Bun v0.6.6

Lots of improvements to `bun test`, including Github Actions support,
`bun test --only`, `test.if()`, `describe.skip()`, and more matchers

Ashcon Partovi · May 31, 2023

[](/blog/bun-macros)

JavaScript Macros in Bun

Call JavaScript functions at bundle-time and inline results into the AST

Jarred Sumner · May 31, 2023

[](/blog/bun-v0.6.5)

Bun v0.6.5

Native CommonJS support, \$npm_lifecycle_event, bugfix for Bun.serve()
with streaming files, bugfix for Node.js createConnection and a bundler
bugfix

Jarred Sumner · May 29, 2023

[](/blog/bun-v0.6.4)

Bun v0.6.4

Up to 80% faster bun test, test timeouts, timezone support, bundler
bugfixes, console.log() improvements, node:http bugfixes, and more

Ashcon Partovi · May 26, 2023

[](/blog/bun-v0.6.3)

Bun v0.6.3

Introducing `node:vm` support, support for `socket.io` and `mongodb`,
`test.todo()`, and many bug fixes for the bundler.

Ashcon Partovi · May 22, 2023

[](/blog/bun-v0.6.2)

Bun v0.6.2

Up to 20% faster JSON.parse, 15% faster JSON.stringify, and Proxy gets
faster. A couple bugfixes to the minifier and bundler

Jarred Sumner · May 17, 2023

[](/blog/bun-bundler)

The Bun Bundler

The Bun toolkit gets a new addition. Bun's fast native bundler is now in
beta.

Jarred Sumner · May 16, 2023

[](/blog/bun-v0.6.0)

Bun v0.6.0

A new JavaScript bundler, standalone executables, and lots of new
features and improvements

Jarred Sumner · May 16, 2023

[](/blog/bun-v0.5.9)

Bun v0.5.9

Introducing watch mode using `bun --watch`, tarball dependencies in
`bun install`, and lots of bug fixes and compatibility improvements.

Ashcon Partovi · April 4, 2023

[](/blog/bun-v0.5.8)

Bun v0.5.8

Introducing `expect()` snapshots in `bun test`, preloading modules using
`bun --preload`, basic glob star support in bun install, improvements to
Web & Node.js compatibility, lots of bug fixes, and a teaser of what's
next.

Ashcon Partovi · March 18, 2023

[](/blog/bun-has-docs-now)

Bun has docs now

Bun has docs now? Bun has docs now.

Colin McDonnell · February 24, 2023

[](/blog/bun-v0.5.7)

Bun v0.5.7

Support for `FormData` and `git` dependencies, better `setTimeout()`
compatibility, `bun wiptest` is now `bun test`, and better support on
AWS Lambda and GitHub Actions.

Ashcon Partovi · February 24, 2023

[](/blog/bun-v0.5.6)

Bun v0.5.6

Support for `Bun.sleep`, improved Docker support, and improvements to
Node.js and npm compatibility

Ashcon Partovi · February 9, 2023

[](/blog/bun-v0.5.5)

Bun v0.5.5

Improved compatibility for `node:http`, support for `http.request()` and
`stripe-node`, and fixed various bugs.

Ashcon Partovi · February 2, 2023

[](/blog/bun-v0.5.2)

Bun v0.5.2

MongoDB support, `github:` dependencies in `bun install`, faster
`Buffer`, `bun run` for WebAssembly, express' `body-parser` package, and
many bug fixes.

Jarred Sumner · January 31, 2023

[](/blog/bun-v0.5.1)

Bun v0.5.1

Support for aliasing package.json dependencies, a better error message
when a workspace is not found, and a few important bug fixes.

Jarred Sumner · January 19, 2023

[](/blog/bun-v0.5.0)

Bun v0.5

Support for npm workspaces, DNS lookups, database drivers, and lots of
improvements to bun install.

Ashcon Partovi · January 18, 2023

[](/blog/bun-v0.4.0)

Bun v0.4

Introducing bunx. Install and run an executable from npm, 100x faster
than npx.

Ashcon Partovi · December 23, 2022

[](/blog/bun-v0.3.0)

Bun v0.3.0

Massive reduction in memory usage, faster text encoding, improved Jest
compatibility, file-system routing, automatic package installs, and a
lot more

Ashcon Partovi · December 7, 2022

[](/blog/bun-v0.2.2)

Bun v0.2.2

Release notes for Bun v0.2.2

Jarred Sumner · October 27, 2022

[](/blog/bun-v0.2.1)

Bun v0.2.1

Release notes for Bun v0.2.1

Jarred Sumner · October 19, 2022

[](/blog/bun-v0.2.0)

Bun v0.2.0

Release notes for Bun v0.2.0

Jarred Sumner · October 13, 2022

[](/blog/bun-v0.1.12)

Bun v0.1.12

Release notes for Bun v0.1.12

Jarred Sumner · September 18, 2022

[](/blog/bun-v0.1.11)

Bun v0.1.11

Release notes for Bun v0.1.11

Jarred Sumner · September 7, 2022

[](/blog/bun-v0.1.10)

Bun v0.1.10

Release notes for Bun v0.1.10

Jarred Sumner · August 19, 2022

[](/blog/bun-v0.1.9)

Bun v0.1.9

Release notes for Bun v0.1.9

Jarred Sumner · August 18, 2022

[](/blog/bun-v0.1.8)

Bun v0.1.8

Release notes for Bun v0.1.8

Jarred Sumner · August 11, 2022

[](/blog/bun-v0.1.7)

Bun v0.1.7

Release notes for Bun v0.1.7

Jarred Sumner · August 6, 2022

[](/blog/bun-v0.1.6)

Bun v0.1.6

Release notes for Bun v0.1.6

Jarred Sumner · August 1, 2022

[](/blog/bun-v0.1.5)

Bun v0.1.5

Release notes for Bun v0.1.5

Jarred Sumner · July 23, 2022

[](/blog/bun-v0.1.4)

Bun v0.1.4

Release notes for Bun v0.1.4

Jarred Sumner · July 13, 2022

[](/blog/bun-v0.1.3)

Bun v0.1.3

Release notes for Bun v0.1.3

Jarred Sumner · July 11, 2022

[](/blog/bun-v0.1.2)

Bun v0.1.2

Release notes for Bun v0.1.2

Jarred Sumner · July 7, 2022

[](/blog/bun-v0.1.1)

Bun v0.1.1

Release notes for Bun v0.1.1

Jarred Sumner · July 5, 2022

