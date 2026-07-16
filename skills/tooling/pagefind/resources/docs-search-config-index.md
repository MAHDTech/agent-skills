> **Important: Pagefind 1.5.0 introduces the Component UI, which replaces the
> Default UI (pagefind-ui.js / PagefindUI). It includes a search modal, better
> accessibility and customization.** Full component guide:
> https://pagefind.app/llms-component-ui.txt

# Configuring the Pagefind search in the browser


The behaviour of the Pagefind search API can be configured in the browser.

**UI (declarative):**

If using the Component UI, options can be set declaratively with the `<pagefind-config>` element:

```html
+<pagefind-config base-url="/"></pagefind-config>

<pagefind-input></pagefind-input>
<pagefind-results></pagefind-results>
```

See the full list of available attributes in the [`<pagefind-config>` reference](/docs/components/config/).
**UI (programmatic):**

Options can be passed through to Pagefind via `configureInstance`. Import the function based on how you installed the Component UI:

```js
// If installed via a package manager:
import { configureInstance } from '@pagefind/component-ui';
// If loaded via script tag:
const { configureInstance } = window.PagefindComponents;

+configureInstance("default", {
+    baseUrl: "/",
+    // ... more search options
+});
```
**Search API:**

If interfacing with Pagefind directly, options can be passed via awaiting `pagefind.options()`:

```js
const pagefind = await import("/pagefind/pagefind.js");
+await pagefind.options({
+    baseUrl: "/",
+    // ... more search options
+});
```

## Available options

### Base URL

Defaults to "/". If hosting a site on a subpath, `baseUrl` can be provided, and will be appended to the front of all search result URLs.

**UI (declarative):**

```html
<pagefind-config base-url="/docs/"></pagefind-config>
```
**UI (programmatic):**

```js
configureInstance("default", {
+    baseUrl: "/docs/"
});
```
**Search API:**

```js
await pagefind.options({
+    baseUrl: "/docs/"
});
```

### Bundle path

Overrides the bundle directory. In most cases this should be automatically detected by the import URL. Set this if search isn't working and you are seeing a console warning that this path could not be detected.

**UI (declarative):**

```html
<pagefind-config bundle-path="/subpath/pagefind/"></pagefind-config>
```
**UI (programmatic):**

```js
configureInstance("default", {
+    bundlePath: "/subpath/pagefind/"
});
```
**Search API:**

```js
await pagefind.options({
+    basePath: "/subpath/pagefind/"
});
```

> Note: When using the Search API directly, the option is called `basePath`. It is usually auto-detected from the URL used to import `pagefind.js`, so you only need to set it if auto-detection fails.

### Excerpt length

Set the maximum length for generated excerpts. Defaults to `30`.

**UI (declarative):**

```html
<pagefind-config excerpt-length="15"></pagefind-config>
```
**UI (programmatic):**

```js
configureInstance("default", {
+    excerptLength: 15
});
```
**Search API:**

```js
await pagefind.options({
+    excerptLength: 15
});
```

### Highlight query parameter

If set, Pagefind will add the search term as a query parameter under the same name.

If using the [Pagefind highlight script](/docs/highlighting/), make sure this is configured to match.

**UI (declarative):**

```html
<pagefind-config highlight-param="highlight"></pagefind-config>
```
**UI (programmatic):**

```js
configureInstance("default", {
+    highlightParam: "highlight"
});
```
**Search API:**

```js
await pagefind.options({
+    highlightParam: "highlight"
});
```

### Exact Diacritics

Defaults to `false`. When set to `true`, diacritics (accents such as àéö) are treated as fully distinct characters.

By default, Pagefind normalizes diacritics so that searching for "cafe" will match pages containing "café" and vice versa. When diacritics are normalized, exact matches are still preferred via the [`ranking.diacriticSimilarity`](/docs/ranking/#configuring-diacritic-similarity) parameter.

When `exactDiacritics` is set to `true`:
- Searching for "café" will only match pages containing "café"
- Searching for "cafe" will only match pages containing "cafe"

**UI (declarative):**

```html
<pagefind-config exact-diacritics></pagefind-config>
```
**UI (programmatic):**

```js
configureInstance("default", {
+    exactDiacritics: true
});
```
**Search API:**

```js
await pagefind.options({
+    exactDiacritics: true
});
```

### Meta cache tag

By default, Pagefind appends a timestamp to the metadata request to ensure fresh data. If you're building a PWA or offline-capable site, set this to a fixed string so that your service worker can cache the request. Change this value each time you rebuild your site. A build timestamp or random string works well.

The search index itself is unaffected and can always be cached.

**UI (declarative):**

```html
<pagefind-config meta-cache-tag="abc123"></pagefind-config>
```
**UI (programmatic):**

```js
configureInstance("default", {
+    metaCacheTag: "abc123"
});
```
**Search API:**

```js
await pagefind.options({
+    metaCacheTag: "abc123"
});
```

### Ranking

See [customize ranking](/docs/ranking/)

### Index weight

See [multisite search > weighting](/docs/multisite/#changing-the-weighting-of-individual-indexes)

### Merge filter

See [multisite search > filtering](/docs/multisite/#filtering-results-by-index)

### Disable web worker

Defaults to `false`. If set to `true`, forces Pagefind to run all search operations on the main thread instead of using a web worker.

By default, Pagefind will attempt to use a web worker for search operations when available, which helps keep the main thread responsive during searches. If web workers are not supported or fail to initialize, Pagefind will automatically fall back to running on the main thread.

**UI (declarative):**

```html
<pagefind-config no-worker></pagefind-config>
```
**UI (programmatic):**

```js
configureInstance("default", {
+    noWorker: true
});
```
**Search API:**

```js
await pagefind.options({
+    noWorker: true
});
```