<!doctype html>
<html lang="en" dir="ltr" class="docs-wrapper plugin-docs plugin-id-default docs-version-current docs-doc-page docs-doc-id-resources/reference/cli-reference/cli-reference" data-has-hydrated="false">
\<head\>
<meta charset="UTF-8">
<meta name="generator" content="Docusaurus v3.9.2">
<title data-rh="true">CLI Reference | SpacetimeDB docs</title><meta data-rh="true" name="viewport" content="width=device-width,initial-scale=1"><meta data-rh="true" name="twitter:card" content="summary_large_image"><meta data-rh="true" property="og:url" content="https://spacetimedb.com/docs/cli-reference"><meta data-rh="true" property="og:locale" content="en"><meta data-rh="true" name="docusaurus_locale" content="en"><meta data-rh="true" name="docsearch:language" content="en"><meta data-rh="true" name="docusaurus_version" content="current"><meta data-rh="true" name="docusaurus_tag" content="docs-default-current"><meta data-rh="true" name="docsearch:version" content="current"><meta data-rh="true" name="docsearch:docusaurus_tag" content="docs-default-current"><meta data-rh="true" property="og:title" content="CLI Reference | SpacetimeDB docs"><meta data-rh="true" name="description" content="This document contains the help content for the spacetime command-line program."><meta data-rh="true" property="og:description" content="This document contains the help content for the spacetime command-line program."><link data-rh="true" rel="icon" href="https://spacetimedb.com/favicon-32x32.png"><link data-rh="true" rel="canonical" href="https://spacetimedb.com/docs/cli-reference"><link data-rh="true" rel="alternate" href="https://spacetimedb.com/docs/cli-reference" hreflang="en"><link data-rh="true" rel="alternate" href="https://spacetimedb.com/docs/cli-reference" hreflang="x-default"><script data-rh="true" type="application/ld+json">{"@context":"https://schema.org","@type":"BreadcrumbList","itemListElement":[{"@type":"ListItem","position":1,"name":"CLI Reference","item":"https://spacetimedb.com/docs/cli-reference"}]}</script><link rel="alternate" type="text/markdown" title="SpacetimeDB docs for agents" href="/docs/llms.txt">
<link rel="alternate" type="text/markdown" title="Full SpacetimeDB docs for agents" href="/docs/llms-full.txt">
<link rel="sitemap" type="application/xml" href="/docs/sitemap.xml">
<link rel="preload" as="font" type="font/woff2" href="/docs/fonts/inter-latin-wght-normal.woff2" crossorigin="anonymous">
<link rel="preload" as="font" type="font/woff2" href="/docs/fonts/source-code-pro-latin-wght-normal.woff2" crossorigin="anonymous"><link rel="stylesheet" href="/docs/assets/css/styles.4f000462.css">
<script src="/docs/assets/js/runtime~main.00ab2442.js" defer="defer"></script>
<script src="/docs/assets/js/main.67c34654.js" defer="defer"></script>
</head>
<body class="navigation-with-keyboard">
<svg style="display: none;">\<defs\>
<symbol id="theme-svg-external-link" viewBox="0 0 24 24"><path fill="currentColor" d="M21 13v10h-21v-19h12v2h-10v15h17v-8h2zm3-12h-10.988l4.035 4-6.977 7.07 2.828 2.828 6.977-7.07 4.125 4.172v-11z"/></symbol>
</defs></svg>
\<script\>document.documentElement.setAttribute("data-theme","light"),document.documentElement.setAttribute("data-theme-choice","light"),function(){try{const c=new URLSearchParams(window.location.search).entries();for(var[t,e]of c)if(t.startsWith("docusaurus-data-")){var a=t.replace("docusaurus-data-","data-");document.documentElement.setAttribute(a,e)}}catch(t){}}()</script><div id="__docusaurus"><div role="region" aria-label="Skip to main content"><a class="skipToContent_M6kL" href="#__docusaurus_skipToContent_fallback">Skip to main content</a></div><nav aria-label="Main" class="theme-layout-navbar navbar navbar--fixed-top"><div class="navbar__inner"><div class="theme-layout-navbar-left navbar__items"><button aria-label="Toggle navigation bar" aria-expanded="false" class="navbar__toggle clean-btn" type="button"><svg width="30" height="30" viewBox="0 0 30 30" aria-hidden="true"><path stroke="currentColor" stroke-linecap="round" stroke-miterlimit="10" stroke-width="2" d="M4 7h22M4 15h22M4 23h22"></path></svg></button><a href="https://spacetimedb.com" target="_self" rel="noopener noreferrer" class="navbar__brand"><div class="navbar__logo"><img src="https://spacetimedb.com/images/brand.svg" alt="Spacetime Logo" class="themedComponent_J5mh themedComponent--light_Du4S" height="32" width="133"><img src="https://spacetimedb.com/images/brand.svg" alt="Spacetime Logo" class="themedComponent_J5mh themedComponent--dark_W6YC" height="32" width="133"></div></a><div class="navbarSearchContainer_NRqO"><div id="inkeep-shadowradix-:Rnb5a5:" style="display:contents"></div><span></span></div><div class="navbar__item dropdown dropdown--hoverable"><a aria-current="page" class="navbar__link active" aria-haspopup="true" aria-expanded="false" role="button" href="/docs/cli-reference">2.0.0</a><ul class="dropdown__menu">\<li\><a aria-current="page" class="dropdown__link dropdown__link--active" href="/docs/cli-reference">2.0.0</a></li>\<li\><a class="dropdown__link" href="/docs/1.12.0/cli-reference">1.12.0</a></li></ul></div></div><div class="theme-layout-navbar-right navbar__items navbar__items--right"><a href="https://spacetimedb.com/install" target="_blank" rel="noopener noreferrer" class="navbar__item navbar__link">Install</a><a href="https://spacetimedb.com/pricing" target="_blank" rel="noopener noreferrer" class="navbar__item navbar__link">Pricing</a><a href="https://spacetimedb.com/blog" target="_blank" rel="noopener noreferrer" class="navbar__item navbar__link">Blog</a><a href="https://spacetimedb.com/community" target="_blank" rel="noopener noreferrer" class="navbar__item navbar__link">Community</a><a href="https://spacetimedb.com/space-race" target="_blank" rel="noopener noreferrer" class="navbar__item navbar__link">Referrals</a><a href="https://spacetimedb.com/login" target="_blank" rel="noopener noreferrer" class="navbar__item navbar__link navbar__button">Login</a></div></div><div role="presentation" class="navbar-sidebar__backdrop"></div></nav><div id="__docusaurus_skipToContent_fallback" class="theme-layout-main main-wrapper mainWrapper_zieo"><div class="docsWrapper_LRYd"><button aria-label="Scroll back to top" class="clean-btn theme-back-to-top-button backToTopButton_uRlU" type="button"></button><div class="docRoot_xHAI"><aside class="theme-doc-sidebar-container docSidebarContainer_izyX"><div class="sidebarViewport_fAk8"><div class="sidebar_mXBm"><nav aria-label="Docs sidebar" class="menu thin-scrollbar menu_xs3F"><ul class="theme-doc-sidebar-menu menu__list"><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item sidebar-ask-ai"><a class="menu__link" href="/docs/ask-ai/ask-ai"><span title="Ask AI Chat" class="linkLabel_UvDg">Ask AI Chat</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1"><div class="spacetime-menu-header">Intro</div></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/"><span title="Getting Started" class="linkLabel_UvDg">Getting Started</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/intro/what-is-spacetimedb"><span title="What is SpacetimeDB?" class="linkLabel_UvDg">What is SpacetimeDB?</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/intro/zen"><span title="The Zen of Spacetime" class="linkLabel_UvDg">The Zen of Spacetime</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/intro/language-support"><span title="Language Support" class="linkLabel_UvDg">Language Support</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/intro/key-architecture"><span title="Key Architecture" class="linkLabel_UvDg">Key Architecture</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/intro/faq"><span title="FAQ" class="linkLabel_UvDg">FAQ</span></a></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist" href="/docs/quickstarts/react"><span title="Quickstarts" class="categoryLinkLabel_HATI">Quickstarts</span></a><button aria-label="Expand sidebar category &#x27;Quickstarts&#x27;" aria-expanded="false" type="button" class="clean-btn menu__caret"></button></div></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist" href="/docs/tutorials/chat-app"><span title="Tutorials" class="categoryLinkLabel_HATI">Tutorials</span></a><button aria-label="Expand sidebar category &#x27;Tutorials&#x27;" aria-expanded="false" type="button" class="clean-btn menu__caret"></button></div></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item spacetime-menu-header-link"><a class="menu__link" href="/docs/core-concepts"><span title="Core Concepts" class="linkLabel_UvDg">Core Concepts</span></a></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist" href="/docs/databases"><span title="Databases" class="categoryLinkLabel_HATI">Databases</span></a><button aria-label="Expand sidebar category &#x27;Databases&#x27;" aria-expanded="false" type="button" class="clean-btn menu__caret"></button></div></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist" href="/docs/functions"><span title="Functions" class="categoryLinkLabel_HATI">Functions</span></a><button aria-label="Expand sidebar category &#x27;Functions&#x27;" aria-expanded="false" type="button" class="clean-btn menu__caret"></button></div></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist" href="/docs/tables"><span title="Tables" class="categoryLinkLabel_HATI">Tables</span></a><button aria-label="Expand sidebar category &#x27;Tables&#x27;" aria-expanded="false" type="button" class="clean-btn menu__caret"></button></div></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist" href="/docs/core-concepts/authentication"><span title="Authentication" class="categoryLinkLabel_HATI">Authentication</span></a><button aria-label="Expand sidebar category &#x27;Authentication&#x27;" aria-expanded="false" type="button" class="clean-btn menu__caret"></button></div></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist" href="/docs/clients"><span title="Clients" class="categoryLinkLabel_HATI">Clients</span></a><button aria-label="Expand sidebar category &#x27;Clients&#x27;" aria-expanded="false" type="button" class="clean-btn menu__caret"></button></div></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item spacetime-menu-header-link"><a class="menu__link" href="/docs/resources"><span title="Developer Resources" class="linkLabel_UvDg">Developer Resources</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/troubleshooting"><span title="Basic troubleshooting" class="linkLabel_UvDg">Basic troubleshooting</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/upgrade"><span title="Migrating from 1.0 to 2.0" class="linkLabel_UvDg">Migrating from 1.0 to 2.0</span></a></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist menu__link--sublist-caret" role="button" aria-expanded="false" href="/docs/troubleshooting"><span title="How To" class="categoryLinkLabel_HATI">How To</span></a></div></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist menu__link--sublist-caret menu__link--active" role="button" aria-expanded="true" href="/docs/cli-reference"><span title="References" class="categoryLinkLabel_HATI">References</span></a></div><ul class="menu__list"><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-2 menu__list-item"><div class="menu__list-item-collapsible menu__list-item-collapsible--active"><a class="categoryLink_iWzw menu__link menu__link--sublist menu__link--active" aria-current="page" tabindex="0" href="/docs/cli-reference"><span title="CLI Reference" class="categoryLinkLabel_HATI">CLI Reference</span></a><button aria-label="Collapse sidebar category &#x27;CLI Reference&#x27;" aria-expanded="true" type="button" class="clean-btn menu__caret"></button></div><ul class="menu__list"><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-3 menu__list-item"><a class="menu__link" tabindex="0" href="/docs/cli-reference/standalone-config"><span title="Standalone Configuration" class="linkLabel_UvDg">Standalone Configuration</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-3 menu__list-item"><a class="menu__link" tabindex="0" href="/docs/cli-reference/spacetime-json"><span title="spacetime.json" class="linkLabel_UvDg">spacetime.json</span></a></li></ul></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-2 menu__list-item"><a class="menu__link" tabindex="0" href="/docs/resources/mcp"><span title="MCP Reference" class="linkLabel_UvDg">MCP Reference</span></a></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-2 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist menu__link--sublist-caret" role="button" aria-expanded="false" tabindex="0" href="/docs/http/authorization"><span title="HTTP API" class="categoryLinkLabel_HATI">HTTP API</span></a></div></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-2 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist menu__link--sublist-caret" role="button" aria-expanded="false" tabindex="0" href="/docs/webassembly-abi"><span title="Internals" class="categoryLinkLabel_HATI">Internals</span></a></div></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-2 menu__list-item"><a class="menu__link" tabindex="0" href="/docs/reference/sql"><span title="SQL Reference" class="linkLabel_UvDg">SQL Reference</span></a></li></ul></li></ul></nav></div></div></aside><main class="docMainContainer_dsjb"><div class="container padding-top--md padding-bottom--lg"><div class="row"><div class="col docItemCol_TFty"><div class="docItemContainer_LO0A">\<article\><nav class="theme-doc-breadcrumbs breadcrumbsContainer_EWVV" aria-label="Breadcrumbs"><ul class="breadcrumbs"><li class="breadcrumbs__item"><a aria-label="Home page" class="breadcrumbs__link" href="/docs/"><svg viewBox="0 0 24 24" class="breadcrumbHomeIcon_i5pl"><path d="M10 19v-5h4v5c0 .55.45 1 1 1h3c.55 0 1-.45 1-1v-7h1.7c.46 0 .68-.57.33-.87L12.67 3.6c-.38-.34-.96-.34-1.34 0l-8.36 7.53c-.34.3-.13.87.33.87H5v7c0 .55.45 1 1 1h3c.55 0 1-.45 1-1z" fill="currentColor"></path></svg></a></li><li class="breadcrumbs__item"><span class="breadcrumbs__link">References</span></li><li class="breadcrumbs__item breadcrumbs__item--active"><span class="breadcrumbs__link">CLI Reference</span></li></ul></nav><span class="theme-doc-version-badge badge badge--secondary">Version: 2.0.0</span><div class="tocCollapsible_ldQs theme-doc-toc-mobile tocMobile_xBih"><button type="button" class="clean-btn tocCollapsibleButton_Dk1U">On this page</button></div><div class="theme-doc-markdown markdown">\<header\>\<h1\>Command-Line Help for <code>spacetime</code></h1></header>
<p>This document contains the help content for the <code>spacetime</code> command-line program.</p>
<p><strong>Command Overview:</strong></p>
\<ul\>
<li class=""><a href="#spacetime" class=""><code>spacetime</code>↴</a></li>
<li class=""><a href="#spacetime-publish" class=""><code>spacetime publish</code>↴</a></li>
<li class=""><a href="#spacetime-env" class=""><code>spacetime env</code>↴</a></li>
<li class=""><a href="#spacetime-env-get" class=""><code>spacetime env get</code>↴</a></li>
<li class=""><a href="#spacetime-env-list" class=""><code>spacetime env list</code>↴</a></li>
<li class=""><a href="#spacetime-delete" class=""><code>spacetime delete</code>↴</a></li>
<li class=""><a href="#spacetime-logs" class=""><code>spacetime logs</code>↴</a></li>
<li class=""><a href="#spacetime-call" class=""><code>spacetime call</code>↴</a></li>
<li class=""><a href="#spacetime-describe" class=""><code>spacetime describe</code>↴</a></li>
<li class=""><a href="#spacetime-dev" class=""><code>spacetime dev</code>↴</a></li>
<li class=""><a href="#spacetime-sql" class=""><code>spacetime sql</code>↴</a></li>
<li class=""><a href="#spacetime-mcp" class=""><code>spacetime mcp</code>↴</a></li>
<li class=""><a href="#spacetime-rename" class=""><code>spacetime rename</code>↴</a></li>
<li class=""><a href="#spacetime-generate" class=""><code>spacetime generate</code>↴</a></li>
<li class=""><a href="#spacetime-list" class=""><code>spacetime list</code>↴</a></li>
<li class=""><a href="#spacetime-login" class=""><code>spacetime login</code>↴</a></li>
<li class=""><a href="#spacetime-login-show" class=""><code>spacetime login show</code>↴</a></li>
<li class=""><a href="#spacetime-logout" class=""><code>spacetime logout</code>↴</a></li>
<li class=""><a href="#spacetime-init" class=""><code>spacetime init</code>↴</a></li>
<li class=""><a href="#spacetime-build" class=""><code>spacetime build</code>↴</a></li>
<li class=""><a href="#spacetime-server" class=""><code>spacetime server</code>↴</a></li>
<li class=""><a href="#spacetime-server-list" class=""><code>spacetime server list</code>↴</a></li>
<li class=""><a href="#spacetime-server-set-default" class=""><code>spacetime server set-default</code>↴</a></li>
<li class=""><a href="#spacetime-server-add" class=""><code>spacetime server add</code>↴</a></li>
<li class=""><a href="#spacetime-server-remove" class=""><code>spacetime server remove</code>↴</a></li>
<li class=""><a href="#spacetime-server-fingerprint" class=""><code>spacetime server fingerprint</code>↴</a></li>
<li class=""><a href="#spacetime-server-ping" class=""><code>spacetime server ping</code>↴</a></li>
<li class=""><a href="#spacetime-server-edit" class=""><code>spacetime server edit</code>↴</a></li>
<li class=""><a href="#spacetime-server-clear" class=""><code>spacetime server clear</code>↴</a></li>
<li class=""><a href="#spacetime-subscribe" class=""><code>spacetime subscribe</code>↴</a></li>
<li class=""><a href="#spacetime-start" class=""><code>spacetime start</code>↴</a></li>
<li class=""><a href="#spacetime-lock" class=""><code>spacetime lock</code>↴</a></li>
<li class=""><a href="#spacetime-unlock" class=""><code>spacetime unlock</code>↴</a></li>
<li class=""><a href="#spacetime-version" class=""><code>spacetime version</code>↴</a></li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime"><code>spacetime</code><a href="#spacetime" class="hash-link" aria-label="Direct link to spacetime" title="Direct link to spacetime" translate="no">​</a></h2>
<p><strong>Usage:</strong> <code>spacetime [OPTIONS] \<COMMAND\></code></p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="subcommands"><strong>Subcommands:</strong><a href="#subcommands" class="hash-link" aria-label="Direct link to subcommands" title="Direct link to subcommands" translate="no">​</a></h6>
\<ul\>
<li class=""><code>publish</code> — Create and update a SpacetimeDB database</li>
<li class=""><code>env</code> — Inspect published database environment variables</li>
<li class=""><code>delete</code> — Deletes a SpacetimeDB database</li>
<li class=""><code>logs</code> — Prints logs from a SpacetimeDB database</li>
<li class=""><code>call</code> — Invokes a function (reducer or procedure) in a database. WARNING: This command is UNSTABLE and subject to breaking changes.</li>
<li class=""><code>describe</code> — Describe the structure of a database or entities within it. WARNING: This command is UNSTABLE and subject to breaking changes.</li>
<li class=""><code>dev</code> — Start development mode with auto-regenerate client module bindings, auto-rebuild, and auto-publish on file changes.</li>
<li class=""><code>sql</code> — Runs a SQL query on the database. WARNING: This command is UNSTABLE and subject to breaking changes.</li>
<li class=""><code>mcp</code> — Serve SpacetimeDB to MCP-aware agents and editors over stdio. WARNING: This command is UNSTABLE and subject to breaking changes.</li>
<li class=""><code>rename</code> — Rename a database</li>
<li class=""><code>generate</code> — Generate client files for a spacetime module.</li>
<li class=""><code>list</code> — Lists the databases attached to an identity. WARNING: This command is UNSTABLE and subject to breaking changes.</li>
<li class=""><code>login</code> — Manage your login to the SpacetimeDB CLI</li>
<li class=""><code>logout</code> —</li>
<li class=""><code>init</code> — Initializes a new spacetime project.</li>
<li class=""><code>build</code> — Builds a spacetime module.</li>
<li class=""><code>server</code> — Manage the connection to the SpacetimeDB server. WARNING: This command is UNSTABLE and subject to breaking changes.</li>
<li class=""><code>subscribe</code> — Subscribe to SQL queries on the database. WARNING: This command is UNSTABLE and subject to breaking changes.</li>
<li class=""><code>start</code> — Start a local SpacetimeDB instance</li>
<li class=""><code>lock</code> — Lock a database to prevent accidental deletion</li>
<li class=""><code>unlock</code> — Unlock a database to allow deletion</li>
<li class=""><code>version</code> — Manage installed spacetime versions</li>
</ul>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options"><strong>Options:</strong><a href="#options" class="hash-link" aria-label="Direct link to options" title="Direct link to options" translate="no">​</a></h6>
\<ul\>
<li class=""><code>--root-dir \<ROOT_DIR\></code> — The root directory for SpacetimeDB files. If you installed with --root-dir, pass the same value on each command, before the subcommand (for example, spacetime --root-dir /stdb start).</li>
<li class=""><code>--config-path \<CONFIG_PATH\></code> — The path to the cli.toml config file</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-publish"><code>spacetime publish</code><a href="#spacetime-publish" class="hash-link" aria-label="Direct link to spacetime-publish" title="Direct link to spacetime-publish" translate="no">​</a></h2>
<p>Create and update a SpacetimeDB database</p>
<p><strong>Usage:</strong> <code>spacetime publish [OPTIONS] [name|identity]</code></p>
<p>Publishing preserves unspecified environment values. Put an env map in spacetime.json in order to specify variables in config. Explicit keys which are not declared in the module are allowed. Declared shell variables override config values (including empty strings). The CLI displays supplied keys and sources. --env-only updates an existing database&#x27;s environment without publishing the module. --unset-env explicitly removes a previously published environment variable, unless the variable is declared required by the currently published module. --replace-env replaces all stored values with the supplied set, including deleting unspecified undeclared keys, and cannot be combined with --unset-env. The host validates the resulting environment atomically against the declared module. --env selects which config file to use.</p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="arguments"><strong>Arguments:</strong><a href="#arguments" class="hash-link" aria-label="Direct link to arguments" title="Direct link to arguments" translate="no">​</a></h6>
\<ul\>
<li class="">
<p><code><NAME|IDENTITY></code> — A valid domain or identity for this database.</p>
<p>Database names must match the regex <code>/^[a-z0-9]+(-[a-z0-9]+)*$/</code>,
i.e. only lowercase ASCII letters and numbers, separated by dashes.</p>
</li>
</ul>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-1"><strong>Options:</strong><a href="#options-1" class="hash-link" aria-label="Direct link to options-1" title="Direct link to options-1" translate="no">​</a></h6>
\<ul\>
<li class="">
<p><code>-c</code>, <code>--delete-data <CLEAR-DATABASE></code> — When publishing to an existing database identity, first DESTROY all data associated with the module. With &#x27;on-conflict&#x27;: only when breaking schema changes occur.</p>
<p>Possible values: <code>always</code>, <code>on-conflict</code>, <code>never</code></p>
</li>
<li class="">
<p><code>--build-options <BUILD_OPTIONS></code> — Options to pass to the build command, for example --build-options=&#x27;--lint-dir=&#x27;</p>
<p>Default value: ``</p>
</li>
<li class="">
<p><code>-p</code>, <code>--module-path \<MODULE_PATH\></code> — The system path (absolute or relative) to the module project. Defaults to spacetimedb/ subdirectory, then current directory.</p>
</li>
<li class="">
<p><code>-b</code>, <code>--bin-path \<WASM_FILE\></code> — The system path (absolute or relative) to the compiled wasm binary we should publish, instead of building the project.</p>
</li>
<li class="">
<p><code>-j</code>, <code>--js-path \<JS_FILE\></code> — UNSTABLE: The system path (absolute or relative) to the javascript file we should publish, instead of building the project.</p>
</li>
<li class="">
<p><code>--break-clients</code> — Allow breaking changes when publishing to an existing database identity. Equivalent to --yes=break-clients: skips the "this will BREAK existing clients" prompt, but will NOT force publish if it would cause deletion of any data in the database. See --yes and --delete-data for details.</p>
</li>
<li class="">
<p><code>--anonymous</code> — Perform this action with an anonymous identity</p>
</li>
<li class="">
<p><code>--parent \<PARENT\></code> — A valid domain or identity of an existing database that should be the parent of this database.</p>
<p>If a parent is given, the new database inherits the team permissions from the parent.
A parent can only be set when a database is created, not when it is updated.</p>
</li>
<li class="">
<p><code>--organization \<ORGANIZATION\></code> — The name or identity of an existing organization this database should be created under.</p>
<p>If an organization is given, the organization member&#x27;s permissions apply to the new database.
An organization can only be set when a database is created, not when it is updated.</p>
</li>
<li class="">
<p><code>-s</code>, <code>--server \<SERVER\></code> — The nickname, domain name or URL of the server to host the database.</p>
</li>
<li class="">
<p><code>-y</code>, <code>--yes \<YES\></code> — Skip confirmation prompts. With no value, defaults to &#x27;all&#x27; (equivalent to --yes=all). To skip specific prompts, pass one or more of: all, remote, migrate, break-clients, skip-login, delete-data. Multiple values can be comma-separated (--yes=migrate,break-clients) or given via repeated flags (--yes=migrate --yes=break-clients). The value must be attached with &#x27;=&#x27; (so <code>--yes my-db</code> treats <code>my-db</code> as the database name).</p>
<p>Possible values:</p>
\<ul\>
<li class=""><code>all</code>:
Equivalent to passing every other option. This is the default when <code>--yes</code> is given without a value</li>
<li class=""><code>remote</code>:
Skip the "publish to a non-local server?" confirmation</li>
<li class=""><code>migrate</code>:
Skip migration confirmations (e.g. major version upgrade)</li>
<li class=""><code>break-clients</code>:
Skip the "this will BREAK existing clients" confirmation</li>
<li class=""><code>skip-login</code>:
Don&#x27;t prompt the user to log in; act non-interactively for auth</li>
<li class=""><code>delete-data</code>:
Skip the destructive "this will DESTROY ... ALL corresponding data" confirmation</li>
</ul>
</li>
<li class="">
<p><code>--no-config</code> — Ignore spacetime.json configuration</p>
</li>
<li class="">
<p><code>--env \<ENV\></code> — Environment name for config file layering (e.g., dev, staging)</p>
</li>
<li class="">
<p><code>--native-aot</code> — Use NativeAOT-LLVM compilation for C# modules (experimental; supported on Windows, and on Linux with .NET 10)</p>
</li>
<li class="">
<p><code>--dotnet-version \<VERSION\></code> — Target .NET SDK major version for C# projects (e.g. 8 or 10). Auto-detected when omitted.</p>
</li>
<li class="">
<p><code>--env-only</code> — Update environment values without building or uploading a module.</p>
</li>
<li class="">
<p><code>--unset-env \<KEY\></code> — Delete an environment value. Repeat for multiple keys.</p>
</li>
<li class="">
<p><code>--replace-env</code> — Replace all environment values, deleting every unspecified key.</p>
</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-env"><code>spacetime env</code><a href="#spacetime-env" class="hash-link" aria-label="Direct link to spacetime-env" title="Direct link to spacetime-env" translate="no">​</a></h2>
<p>Inspect published database environment variables</p>
<p><strong>Usage:</strong> <code>spacetime env \<COMMAND\></code></p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="subcommands-1"><strong>Subcommands:</strong><a href="#subcommands-1" class="hash-link" aria-label="Direct link to subcommands-1" title="Direct link to subcommands-1" translate="no">​</a></h6>
\<ul\>
<li class=""><code>get</code> — Read one published environment value</li>
<li class=""><code>list</code> — List published environment keys and values</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-env-get"><code>spacetime env get</code><a href="#spacetime-env-get" class="hash-link" aria-label="Direct link to spacetime-env-get" title="Direct link to spacetime-env-get" translate="no">​</a></h2>
<p>Read one published environment value</p>
<p><strong>Usage:</strong> <code>spacetime env get [OPTIONS] \<database\> \<key\></code></p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="arguments-1"><strong>Arguments:</strong><a href="#arguments-1" class="hash-link" aria-label="Direct link to arguments-1" title="Direct link to arguments-1" translate="no">​</a></h6>
\<ul\>
<li class=""><code>\<KEY\></code> — The stored environment key to read</li>
<li class=""><code>\<DATABASE\></code> — The database name, identity, or configured target</li>
</ul>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-2"><strong>Options:</strong><a href="#options-2" class="hash-link" aria-label="Direct link to options-2" title="Direct link to options-2" translate="no">​</a></h6>
\<ul\>
<li class="">
<p><code>-s</code>, <code>--server \<SERVER\></code> — The nickname, host name or URL of the server</p>
</li>
<li class="">
<p><code>--anonymous</code> — Perform this action with an anonymous identity</p>
</li>
<li class="">
<p><code>-y</code>, <code>--yes</code> — Run non-interactively wherever possible. This will answer "yes" to almost all prompts, but will sometimes answer "no" to preserve non-interactivity (e.g. when prompting whether to log in with spacetimedb.com).</p>
</li>
<li class="">
<p><code>--confirmed \<CONFIRMED\></code> — Instruct the server to deliver only updates of confirmed transactions</p>
<p>Possible values: <code>true</code>, <code>false</code></p>
</li>
<li class="">
<p><code>--no-config</code> — Ignore project configuration when resolving the database target</p>
</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-env-list"><code>spacetime env list</code><a href="#spacetime-env-list" class="hash-link" aria-label="Direct link to spacetime-env-list" title="Direct link to spacetime-env-list" translate="no">​</a></h2>
<p>List published environment keys and values</p>
<p><strong>Usage:</strong> <code>spacetime env list [OPTIONS] \<database\></code></p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="arguments-2"><strong>Arguments:</strong><a href="#arguments-2" class="hash-link" aria-label="Direct link to arguments-2" title="Direct link to arguments-2" translate="no">​</a></h6>
\<ul\>
<li class=""><code>\<DATABASE\></code> — The database name, identity, or configured target</li>
</ul>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-3"><strong>Options:</strong><a href="#options-3" class="hash-link" aria-label="Direct link to options-3" title="Direct link to options-3" translate="no">​</a></h6>
\<ul\>
<li class="">
<p><code>-s</code>, <code>--server \<SERVER\></code> — The nickname, host name or URL of the server</p>
</li>
<li class="">
<p><code>--anonymous</code> — Perform this action with an anonymous identity</p>
</li>
<li class="">
<p><code>-y</code>, <code>--yes</code> — Run non-interactively wherever possible. This will answer "yes" to almost all prompts, but will sometimes answer "no" to preserve non-interactivity (e.g. when prompting whether to log in with spacetimedb.com).</p>
</li>
<li class="">
<p><code>--confirmed \<CONFIRMED\></code> — Instruct the server to deliver only updates of confirmed transactions</p>
<p>Possible values: <code>true</code>, <code>false</code></p>
</li>
<li class="">
<p><code>--no-config</code> — Ignore project configuration when resolving the database target</p>
</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-delete"><code>spacetime delete</code><a href="#spacetime-delete" class="hash-link" aria-label="Direct link to spacetime-delete" title="Direct link to spacetime-delete" translate="no">​</a></h2>
<p>Deletes a SpacetimeDB database</p>
<p><strong>Usage:</strong> <code>spacetime delete [OPTIONS] [database]</code></p>
<p>Run <code>spacetime help delete</code> for more detailed information.</p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="arguments-3"><strong>Arguments:</strong><a href="#arguments-3" class="hash-link" aria-label="Direct link to arguments-3" title="Direct link to arguments-3" translate="no">​</a></h6>
\<ul\>
<li class=""><code>\<DATABASE\></code> — The name or identity of the database to delete</li>
</ul>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-4"><strong>Options:</strong><a href="#options-4" class="hash-link" aria-label="Direct link to options-4" title="Direct link to options-4" translate="no">​</a></h6>
\<ul\>
<li class=""><code>-s</code>, <code>--server \<SERVER\></code> — The nickname, host name or URL of the server hosting the database</li>
<li class=""><code>-y</code>, <code>--yes</code> — Run non-interactively wherever possible. This will answer "yes" to almost all prompts, but will sometimes answer "no" to preserve non-interactivity (e.g. when prompting whether to log in with spacetimedb.com).</li>
<li class=""><code>--no-config</code> — Ignore spacetime.json configuration</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-logs"><code>spacetime logs</code><a href="#spacetime-logs" class="hash-link" aria-label="Direct link to spacetime-logs" title="Direct link to spacetime-logs" translate="no">​</a></h2>
<p>Prints logs from a SpacetimeDB database</p>
<p><strong>Usage:</strong> <code>spacetime logs [OPTIONS] [database]</code></p>
<p>Run <code>spacetime help logs</code> for more detailed information.</p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="arguments-4"><strong>Arguments:</strong><a href="#arguments-4" class="hash-link" aria-label="Direct link to arguments-4" title="Direct link to arguments-4" translate="no">​</a></h6>
\<ul\>
<li class=""><code>\<DATABASE\></code> — The name or identity of the database to print logs from</li>
</ul>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-5"><strong>Options:</strong><a href="#options-5" class="hash-link" aria-label="Direct link to options-5" title="Direct link to options-5" translate="no">​</a></h6>
\<ul\>
<li class="">
<p><code>-s</code>, <code>--server \<SERVER\></code> — The nickname, host name or URL of the server hosting the database</p>
</li>
<li class="">
<p><code>-n</code>, <code>--num-lines \<NUM_LINES\></code> — The number of lines to print from the start of the log of this database. If no num lines is provided, all lines will be returned.</p>
</li>
<li class="">
<p><code>-f</code>, <code>--follow</code> — A flag that causes logs to not stop when end of the log file is reached, but rather to wait for additional data to be appended to the input.</p>
</li>
<li class="">
<p><code>--format \<FORMAT\></code> — Output format for the logs</p>
<p>Default value: <code>text</code></p>
<p>Possible values: <code>text</code>, <code>json</code></p>
</li>
<li class="">
<p><code>-l</code>, <code>--level \<LEVEL\></code> — Filter logs by severity level. Only messages at the specified level or higher will be shown. Levels from least to most severe: trace, debug, info, warn, error, panic.</p>
<p>Possible values: <code>trace</code>, <code>debug</code>, <code>info</code>, <code>warn</code>, <code>error</code>, <code>panic</code></p>
</li>
<li class="">
<p><code>--level-exact</code> — When combined with --level, show only logs at exactly the specified level instead of that level and above.</p>
</li>
<li class="">
<p><code>-y</code>, <code>--yes</code> — Run non-interactively wherever possible. This will answer "yes" to almost all prompts, but will sometimes answer "no" to preserve non-interactivity (e.g. when prompting whether to log in with spacetimedb.com).</p>
</li>
<li class="">
<p><code>--no-config</code> — Ignore spacetime.json configuration</p>
</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-call"><code>spacetime call</code><a href="#spacetime-call" class="hash-link" aria-label="Direct link to spacetime-call" title="Direct link to spacetime-call" translate="no">​</a></h2>
<p>Invokes a function (reducer or procedure) in a database. WARNING: This command is UNSTABLE and subject to breaking changes.</p>
<p><strong>Usage:</strong> <code>spacetime call [OPTIONS] [call_parts]...</code></p>
<p>Run <code>spacetime help call</code> for more detailed information.</p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="arguments-5"><strong>Arguments:</strong><a href="#arguments-5" class="hash-link" aria-label="Direct link to arguments-5" title="Direct link to arguments-5" translate="no">​</a></h6>
\<ul\>
<li class=""><code>\<CALL_PARTS\></code> — Call arguments: [DATABASE] \<FUNCTION_NAME\> [ARGUMENTS...]</li>
</ul>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-6"><strong>Options:</strong><a href="#options-6" class="hash-link" aria-label="Direct link to options-6" title="Direct link to options-6" translate="no">​</a></h6>
\<ul\>
<li class=""><code>-s</code>, <code>--server \<SERVER\></code> — The nickname, host name or URL of the server hosting the database</li>
<li class=""><code>--anonymous</code> — Perform this action with an anonymous identity</li>
<li class=""><code>-y</code>, <code>--yes</code> — Run non-interactively wherever possible. This will answer "yes" to almost all prompts, but will sometimes answer "no" to preserve non-interactivity (e.g. when prompting whether to log in with spacetimedb.com).</li>
<li class=""><code>--no-config</code> — Ignore spacetime.json configuration</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-describe"><code>spacetime describe</code><a href="#spacetime-describe" class="hash-link" aria-label="Direct link to spacetime-describe" title="Direct link to spacetime-describe" translate="no">​</a></h2>
<p>Describe the structure of a database or entities within it. WARNING: This command is UNSTABLE and subject to breaking changes.</p>
<p><strong>Usage:</strong> <code>spacetime describe [OPTIONS] [describe_parts]...</code></p>
<p>Run <code>spacetime help describe</code> for more detailed information.</p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="arguments-6"><strong>Arguments:</strong><a href="#arguments-6" class="hash-link" aria-label="Direct link to arguments-6" title="Direct link to arguments-6" translate="no">​</a></h6>
\<ul\>
<li class=""><code>\<DESCRIBE_PARTS\></code> — Describe arguments: [DATABASE] [ENTITY_TYPE [ENTITY_NAME]]. ENTITY_TYPE is one of tables, views, reducers, procedures, routes, env or types. A route is named by its path, and an environment variable by its key.</li>
</ul>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-7"><strong>Options:</strong><a href="#options-7" class="hash-link" aria-label="Direct link to options-7" title="Direct link to options-7" translate="no">​</a></h6>
\<ul\>
<li class="">
<p><code>--format \<FORMAT\></code> — Output format for the schema</p>
<p>Default value: <code>text</code></p>
<p>Possible values: <code>text</code>, <code>json</code></p>
</li>
<li class="">
<p><code>--json</code> — Output the schema in JSON format. Shorthand for <code>--format json</code>.</p>
</li>
<li class="">
<p><code>--anonymous</code> — Perform this action with an anonymous identity</p>
</li>
<li class="">
<p><code>-s</code>, <code>--server \<SERVER\></code> — The nickname, host name or URL of the server hosting the database</p>
</li>
<li class="">
<p><code>-y</code>, <code>--yes</code> — Run non-interactively wherever possible. This will answer "yes" to almost all prompts, but will sometimes answer "no" to preserve non-interactivity (e.g. when prompting whether to log in with spacetimedb.com).</p>
</li>
<li class="">
<p><code>--no-config</code> — Ignore spacetime.json configuration</p>
</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-dev"><code>spacetime dev</code><a href="#spacetime-dev" class="hash-link" aria-label="Direct link to spacetime-dev" title="Direct link to spacetime-dev" translate="no">​</a></h2>
<p>Start development mode with auto-regenerate client module bindings, auto-rebuild, and auto-publish on file changes.</p>
<p><strong>Usage:</strong> <code>spacetime dev [OPTIONS] [database]</code></p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="arguments-7"><strong>Arguments:</strong><a href="#arguments-7" class="hash-link" aria-label="Direct link to arguments-7" title="Direct link to arguments-7" translate="no">​</a></h6>
\<ul\>
<li class=""><code>\<DATABASE\></code> — The database name/identity to publish to (optional, will prompt if not provided)</li>
</ul>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-8"><strong>Options:</strong><a href="#options-8" class="hash-link" aria-label="Direct link to options-8" title="Direct link to options-8" translate="no">​</a></h6>
\<ul\>
<li class="">
<p><code>--project-path <PROJECT-PATH></code> — The path to the project directory</p>
<p>Default value: <code>.</code></p>
</li>
<li class="">
<p><code>--module-bindings-path <MODULE-BINDINGS-PATH></code> — The path to the module bindings directory relative to the project directory, defaults to <code><project-path>/src/module_bindings</code></p>
<p>Default value: <code>src/module_bindings</code></p>
</li>
<li class="">
<p><code>--module-path <MODULE-PATH></code> — Path to the SpacetimeDB server module, relative to current directory. Defaults to <code><project-path>/spacetimedb</code>.</p>
</li>
<li class="">
<p><code>--client-lang <CLIENT-LANG></code> — The programming language for the generated client module bindings (e.g., typescript, csharp, rust, unrealcpp). If not specified, it will be detected from the project.</p>
<p>Possible values: <code>csharp</code>, <code>typescript</code>, <code>rust</code>, <code>unrealcpp</code></p>
</li>
<li class="">
<p><code>-s</code>, <code>--server \<SERVER\></code> — The nickname, host name or URL of the server to publish to</p>
</li>
<li class="">
<p><code>-y</code>, <code>--yes</code> — Run non-interactively wherever possible. This will answer "yes" to almost all prompts, but will sometimes answer "no" to preserve non-interactivity (e.g. when prompting whether to log in with spacetimedb.com).</p>
</li>
<li class="">
<p><code>-c</code>, <code>--delete-data <CLEAR-DATABASE></code> — When publishing to an existing database identity, first DESTROY all data associated with the module. With &#x27;on-conflict&#x27;: only when breaking schema changes occur.</p>
<p>Possible values: <code>always</code>, <code>on-conflict</code>, <code>never</code></p>
</li>
<li class="">
<p><code>-t</code>, <code>--template \<TEMPLATE\></code> — Template ID or GitHub repository (owner/repo or URL) for project initialization</p>
</li>
<li class="">
<p><code>--dotnet-version \<VERSION\></code> — Target .NET SDK major version for C# projects (e.g. 8 or 10). Auto-detected when omitted.</p>
</li>
<li class="">
<p><code>--native-aot</code> — Build C# projects with NativeAOT-LLVM. Ignored with .NET 10 because NativeAOT-LLVM is always used.</p>
</li>
<li class="">
<p><code>--run \<COMMAND\></code> — Command to run the client development server (overrides spacetime.json config)</p>
</li>
<li class="">
<p><code>--server-only</code> — Only run the server (module) without starting the client</p>
</li>
<li class="">
<p><code>--no-config</code> — Ignore spacetime.json configuration</p>
</li>
<li class="">
<p><code>--env \<ENV\></code> — Environment name for config file layering (e.g., dev, staging). Defaults to &#x27;dev&#x27;.</p>
</li>
<li class="">
<p><code>--skip-publish</code> — Skip the publish step</p>
</li>
<li class="">
<p><code>--skip-generate</code> — Skip the generate step</p>
</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-sql"><code>spacetime sql</code><a href="#spacetime-sql" class="hash-link" aria-label="Direct link to spacetime-sql" title="Direct link to spacetime-sql" translate="no">​</a></h2>
<p>Runs a SQL query on the database. WARNING: This command is UNSTABLE and subject to breaking changes.</p>
<p><strong>Usage:</strong> <code>spacetime sql [OPTIONS] [sql_parts]...</code></p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="arguments-8"><strong>Arguments:</strong><a href="#arguments-8" class="hash-link" aria-label="Direct link to arguments-8" title="Direct link to arguments-8" translate="no">​</a></h6>
\<ul\>
<li class=""><code>\<SQL_PARTS\></code> — SQL arguments: [DATABASE] \<QUERY\></li>
</ul>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-9"><strong>Options:</strong><a href="#options-9" class="hash-link" aria-label="Direct link to options-9" title="Direct link to options-9" translate="no">​</a></h6>
\<ul\>
<li class="">
<p><code>--interactive</code> — Instead of using a query, run an interactive command prompt for <code>SQL</code> expressions</p>
</li>
<li class="">
<p><code>--confirmed \<CONFIRMED\></code> — Instruct the server to deliver only updates of confirmed transactions</p>
<p>Possible values: <code>true</code>, <code>false</code></p>
</li>
<li class="">
<p><code>--anonymous</code> — Perform this action with an anonymous identity</p>
</li>
<li class="">
<p><code>-s</code>, <code>--server \<SERVER\></code> — The nickname, host name or URL of the server hosting the database</p>
</li>
<li class="">
<p><code>--format \<FORMAT\></code> — Output format for the SQL results</p>
<p>Default value: <code>text</code></p>
<p>Possible values: <code>text</code>, <code>json</code></p>
</li>
<li class="">
<p><code>-y</code>, <code>--yes</code> — Run non-interactively wherever possible. This will answer "yes" to almost all prompts, but will sometimes answer "no" to preserve non-interactivity (e.g. when prompting whether to log in with spacetimedb.com).</p>
</li>
<li class="">
<p><code>--no-config</code> — Ignore spacetime.json configuration</p>
</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-mcp"><code>spacetime mcp</code><a href="#spacetime-mcp" class="hash-link" aria-label="Direct link to spacetime-mcp" title="Direct link to spacetime-mcp" translate="no">​</a></h2>
<p>Serve SpacetimeDB to MCP-aware agents and editors over stdio. WARNING: This command is UNSTABLE and subject to breaking changes.</p>
<p><strong>Usage:</strong> <code>spacetime mcp [OPTIONS] [database]</code></p>
<p>Run <code>spacetime help mcp</code> for more detailed information.</p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="arguments-9"><strong>Arguments:</strong><a href="#arguments-9" class="hash-link" aria-label="Direct link to arguments-9" title="Direct link to arguments-9" translate="no">​</a></h6>
\<ul\>
<li class=""><code>\<DATABASE\></code> — The name or identity of a single database to serve. Falls back to the SPACETIMEDB_DB_NAME environment variable. Omit it to serve the whole server, where each tool takes a database argument instead</li>
</ul>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-10"><strong>Options:</strong><a href="#options-10" class="hash-link" aria-label="Direct link to options-10" title="Direct link to options-10" translate="no">​</a></h6>
\<ul\>
<li class=""><code>-s</code>, <code>--server \<SERVER\></code> — The nickname, host name or URL of the server hosting the database</li>
<li class=""><code>--anonymous</code> — Perform this action with an anonymous identity</li>
<li class=""><code>--no-config</code> — Ignore spacetime.json configuration</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-rename"><code>spacetime rename</code><a href="#spacetime-rename" class="hash-link" aria-label="Direct link to spacetime-rename" title="Direct link to spacetime-rename" translate="no">​</a></h2>
<p>Rename a database</p>
<p><strong>Usage:</strong> <code>spacetime rename [OPTIONS] --to <new-name> <database-identity></code></p>
<p>Run <code>spacetime rename --help</code> for more detailed information.</p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="arguments-10"><strong>Arguments:</strong><a href="#arguments-10" class="hash-link" aria-label="Direct link to arguments-10" title="Direct link to arguments-10" translate="no">​</a></h6>
\<ul\>
<li class=""><code><DATABASE-IDENTITY></code> — The database identity to rename</li>
</ul>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-11"><strong>Options:</strong><a href="#options-11" class="hash-link" aria-label="Direct link to options-11" title="Direct link to options-11" translate="no">​</a></h6>
\<ul\>
<li class=""><code>--to <NEW-NAME></code> — The new name you would like to assign</li>
<li class=""><code>-s</code>, <code>--server \<SERVER\></code> — The nickname, host name or URL of the server on which to set the name</li>
<li class=""><code>-y</code>, <code>--yes</code> — Run non-interactively wherever possible. This will answer "yes" to almost all prompts, but will sometimes answer "no" to preserve non-interactivity (e.g. when prompting whether to log in with spacetimedb.com).</li>
<li class=""><code>--no-config</code> — Ignore spacetime.json configuration</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-generate"><code>spacetime generate</code><a href="#spacetime-generate" class="hash-link" aria-label="Direct link to spacetime-generate" title="Direct link to spacetime-generate" translate="no">​</a></h2>
<p>Generate client files for a spacetime module.</p>
<p><strong>Usage:</strong> <code>spacetime generate [DATABASE] --lang \<LANG\> [--module-path \<DIR\> | --bin-path \<PATH\> | --js-path \<PATH\>] [--out-dir \<DIR\> | --uproject-dir \<DIR\>] [--unreal-module-name \<MODULE_NAME\>] [OPTIONS]</code></p>
<p>Run <code>spacetime help generate</code> for more detailed information.</p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="arguments-11"><strong>Arguments:</strong><a href="#arguments-11" class="hash-link" aria-label="Direct link to arguments-11" title="Direct link to arguments-11" translate="no">​</a></h6>
\<ul\>
<li class=""><code>\<DATABASE\></code> — Database name or glob pattern to filter which databases to generate for</li>
</ul>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-12"><strong>Options:</strong><a href="#options-12" class="hash-link" aria-label="Direct link to options-12" title="Direct link to options-12" translate="no">​</a></h6>
\<ul\>
<li class="">
<p><code>-b</code>, <code>--bin-path \<WASM_FILE\></code> — The system path (absolute or relative) to the compiled wasm binary we should inspect</p>
</li>
<li class="">
<p><code>-j</code>, <code>--js-path \<JS_FILE\></code> — The system path (absolute or relative) to the bundled javascript file we should inspect</p>
</li>
<li class="">
<p><code>-p</code>, <code>--module-path \<MODULE_PATH\></code> — The system path (absolute or relative) to the module project. Defaults to spacetimedb/ subdirectory, then current directory.</p>
</li>
<li class="">
<p><code>-o</code>, <code>--out-dir \<OUT_DIR\></code> — The system path (absolute or relative) to the generate output directory</p>
</li>
<li class="">
<p><code>--uproject-dir \<UPROJECT_DIR\></code> — Path to the Unreal project directory, replaces --out-dir for Unreal generation (only used with --lang unrealcpp)</p>
</li>
<li class="">
<p><code>--namespace \<NAMESPACE\></code> — The namespace that should be used</p>
<p>Default value: <code>SpacetimeDB.Types</code></p>
</li>
<li class="">
<p><code>--unreal-module-name \<UNREAL_MODULE_NAME\></code> — The module name that should be used for DLL export macros (required for lang unrealcpp)</p>
</li>
<li class="">
<p><code>--module-prefix \<MODULE_PREFIX\></code> — The module prefix to use for generated types (only used with --lang unrealcpp)</p>
</li>
<li class="">
<p><code>-l</code>, <code>--lang \<LANG\></code> — The language to generate</p>
<p>Possible values: <code>csharp</code>, <code>typescript</code>, <code>rust</code>, <code>unrealcpp</code></p>
</li>
<li class="">
<p><code>--build-options <BUILD_OPTIONS></code> — Options to pass to the build command, for example --build-options=&#x27;--lint-dir=&#x27;</p>
<p>Default value: ``</p>
</li>
<li class="">
<p><code>--dotnet-version \<VERSION\></code> — Target .NET SDK major version for C# projects (e.g. 8 or 10). Auto-detected when omitted.</p>
</li>
<li class="">
<p><code>--include-private</code> — Include private tables and functions in generated code (types are always included).</p>
<p>Default value: <code>false</code></p>
</li>
<li class="">
<p><code>-y</code>, <code>--yes</code> — Run non-interactively wherever possible. This will answer "yes" to almost all prompts, but will sometimes answer "no" to preserve non-interactivity (e.g. when prompting whether to log in with spacetimedb.com).</p>
</li>
<li class="">
<p><code>--no-config</code> — Ignore spacetime.json configuration</p>
</li>
<li class="">
<p><code>--env \<ENV\></code> — Environment name for config file layering (e.g., dev, staging)</p>
</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-list"><code>spacetime list</code><a href="#spacetime-list" class="hash-link" aria-label="Direct link to spacetime-list" title="Direct link to spacetime-list" translate="no">​</a></h2>
<p>Lists the databases attached to an identity. WARNING: This command is UNSTABLE and subject to breaking changes.</p>
<p><strong>Usage:</strong> <code>spacetime list [OPTIONS]</code></p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-13"><strong>Options:</strong><a href="#options-13" class="hash-link" aria-label="Direct link to options-13" title="Direct link to options-13" translate="no">​</a></h6>
\<ul\>
<li class=""><code>-s</code>, <code>--server \<SERVER\></code> — The nickname, host name or URL of the server from which to list databases</li>
<li class=""><code>-y</code>, <code>--yes</code> — Run non-interactively wherever possible. This will answer "yes" to almost all prompts, but will sometimes answer "no" to preserve non-interactivity (e.g. when prompting whether to log in with spacetimedb.com).</li>
<li class=""><code>--no-config</code> — Ignore spacetime.json configuration</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-login"><code>spacetime login</code><a href="#spacetime-login" class="hash-link" aria-label="Direct link to spacetime-login" title="Direct link to spacetime-login" translate="no">​</a></h2>
<p>Manage your login to the SpacetimeDB CLI</p>
<p><strong>Usage:</strong> <code>spacetime login [OPTIONS]        login \<COMMAND\></code></p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="subcommands-2"><strong>Subcommands:</strong><a href="#subcommands-2" class="hash-link" aria-label="Direct link to subcommands-2" title="Direct link to subcommands-2" translate="no">​</a></h6>
\<ul\>
<li class=""><code>show</code> — Show the current login info</li>
</ul>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-14"><strong>Options:</strong><a href="#options-14" class="hash-link" aria-label="Direct link to options-14" title="Direct link to options-14" translate="no">​</a></h6>
\<ul\>
<li class="">
<p><code>--auth-host <AUTH-HOST></code> — Fetch login token from a different host</p>
<p>Default value: <code>https://spacetimedb.com</code></p>
</li>
<li class="">
<p><code>--token <SPACETIMEDB-TOKEN></code> — Bypass the login flow and use a login token directly</p>
</li>
<li class="">
<p><code>--no-browser</code> — Do not open a browser window</p>
</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-login-show"><code>spacetime login show</code><a href="#spacetime-login-show" class="hash-link" aria-label="Direct link to spacetime-login-show" title="Direct link to spacetime-login-show" translate="no">​</a></h2>
<p>Show the current login info</p>
<p><strong>Usage:</strong> <code>spacetime login show [OPTIONS]</code></p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-15"><strong>Options:</strong><a href="#options-15" class="hash-link" aria-label="Direct link to options-15" title="Direct link to options-15" translate="no">​</a></h6>
\<ul\>
<li class=""><code>--token</code> — Also show the auth token</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-logout"><code>spacetime logout</code><a href="#spacetime-logout" class="hash-link" aria-label="Direct link to spacetime-logout" title="Direct link to spacetime-logout" translate="no">​</a></h2>
<p><strong>Usage:</strong> <code>spacetime logout [OPTIONS]</code></p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-16"><strong>Options:</strong><a href="#options-16" class="hash-link" aria-label="Direct link to options-16" title="Direct link to options-16" translate="no">​</a></h6>
\<ul\>
<li class="">
<p><code>--auth-host <AUTH-HOST></code> — Log out from a custom auth server</p>
<p>Default value: <code>https://spacetimedb.com</code></p>
</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-init"><code>spacetime init</code><a href="#spacetime-init" class="hash-link" aria-label="Direct link to spacetime-init" title="Direct link to spacetime-init" translate="no">​</a></h2>
<p>Initializes a new spacetime project.</p>
<p><strong>Usage:</strong> <code>spacetime init [OPTIONS] [PROJECT_NAME]</code></p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="arguments-12"><strong>Arguments:</strong><a href="#arguments-12" class="hash-link" aria-label="Direct link to arguments-12" title="Direct link to arguments-12" translate="no">​</a></h6>
\<ul\>
<li class=""><code>\<PROJECT_NAME\></code> — Project name</li>
</ul>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-17"><strong>Options:</strong><a href="#options-17" class="hash-link" aria-label="Direct link to options-17" title="Direct link to options-17" translate="no">​</a></h6>
\<ul\>
<li class=""><code>--project-path \<PATH\></code> — Directory where the project will be created (defaults to <code>./\<PROJECT_NAME\></code>)</li>
<li class=""><code>--server-only</code> — Initialize server only from the template (no client)</li>
<li class=""><code>--lang \<LANG\></code> — Server language: rust, csharp, typescript, cpp (it can only be used when --template is not specified)</li>
<li class=""><code>-t</code>, <code>--template \<TEMPLATE\></code> — Template ID or GitHub repository (owner/repo or URL)</li>
<li class=""><code>--local</code> — Use local deployment instead of Maincloud</li>
<li class=""><code>--non-interactive</code> — Run in non-interactive mode</li>
<li class=""><code>--native-aot</code> — Configure C# project for NativeAOT-LLVM compilation (experimental; supported on Windows, and on Linux with .NET 10)</li>
<li class=""><code>--dotnet-version \<VERSION\></code> — Target .NET SDK major version for C# projects (e.g. 8 or 10). Defaults to 10 except on macOS or when only .NET 8 is installed.</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-build"><code>spacetime build</code><a href="#spacetime-build" class="hash-link" aria-label="Direct link to spacetime-build" title="Direct link to spacetime-build" translate="no">​</a></h2>
<p>Builds a spacetime module.</p>
<p><strong>Usage:</strong> <code>spacetime build [OPTIONS]</code></p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-18"><strong>Options:</strong><a href="#options-18" class="hash-link" aria-label="Direct link to options-18" title="Direct link to options-18" translate="no">​</a></h6>
\<ul\>
<li class="">
<p><code>-p</code>, <code>--module-path \<MODULE_PATH\></code> — The system path (absolute or relative) to the module project. Defaults to spacetimedb/ subdirectory, then current directory.</p>
</li>
<li class="">
<p><code>--lint-dir \<LINT_DIR\></code> — The directory to lint for nonfunctional print statements. If set to the empty string, skips linting.</p>
<p>Default value: <code>src</code></p>
</li>
<li class="">
<p><code>-d</code>, <code>--debug</code> — Builds the module using debug instead of release (intended to speed up local iteration, not recommended for CI)</p>
</li>
<li class="">
<p><code>--dotnet-version \<VERSION\></code> — Target .NET SDK major version for C# projects (e.g. 8 or 10). Auto-detected when omitted.</p>
</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-server"><code>spacetime server</code><a href="#spacetime-server" class="hash-link" aria-label="Direct link to spacetime-server" title="Direct link to spacetime-server" translate="no">​</a></h2>
<p>Manage the connection to the SpacetimeDB server. WARNING: This command is UNSTABLE and subject to breaking changes.</p>
<p><strong>Usage:</strong> <code>spacetime server        server \<COMMAND\></code></p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="subcommands-3"><strong>Subcommands:</strong><a href="#subcommands-3" class="hash-link" aria-label="Direct link to subcommands-3" title="Direct link to subcommands-3" translate="no">​</a></h6>
\<ul\>
<li class=""><code>list</code> — List stored server configurations</li>
<li class=""><code>set-default</code> — Set the default server for future operations</li>
<li class=""><code>add</code> — Add a new server configuration</li>
<li class=""><code>remove</code> — Remove a saved server configuration</li>
<li class=""><code>fingerprint</code> — Show or update a saved server&#x27;s fingerprint</li>
<li class=""><code>ping</code> — Checks to see if a SpacetimeDB host is online</li>
<li class=""><code>edit</code> — Update a saved server&#x27;s nickname, host name or protocol</li>
<li class=""><code>clear</code> — Deletes all data from all local databases</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-server-list"><code>spacetime server list</code><a href="#spacetime-server-list" class="hash-link" aria-label="Direct link to spacetime-server-list" title="Direct link to spacetime-server-list" translate="no">​</a></h2>
<p>List stored server configurations</p>
<p><strong>Usage:</strong> <code>spacetime server list</code></p>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-server-set-default"><code>spacetime server set-default</code><a href="#spacetime-server-set-default" class="hash-link" aria-label="Direct link to spacetime-server-set-default" title="Direct link to spacetime-server-set-default" translate="no">​</a></h2>
<p>Set the default server for future operations</p>
<p><strong>Usage:</strong> <code>spacetime server set-default \<server\></code></p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="arguments-13"><strong>Arguments:</strong><a href="#arguments-13" class="hash-link" aria-label="Direct link to arguments-13" title="Direct link to arguments-13" translate="no">​</a></h6>
\<ul\>
<li class=""><code>\<SERVER\></code> — The nickname, host name or URL of the new default server</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-server-add"><code>spacetime server add</code><a href="#spacetime-server-add" class="hash-link" aria-label="Direct link to spacetime-server-add" title="Direct link to spacetime-server-add" translate="no">​</a></h2>
<p>Add a new server configuration</p>
<p><strong>Usage:</strong> <code>spacetime server add [OPTIONS] --url \<url\> \<name\></code></p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="arguments-14"><strong>Arguments:</strong><a href="#arguments-14" class="hash-link" aria-label="Direct link to arguments-14" title="Direct link to arguments-14" translate="no">​</a></h6>
\<ul\>
<li class=""><code>\<NAME\></code> — Nickname for this server</li>
</ul>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-19"><strong>Options:</strong><a href="#options-19" class="hash-link" aria-label="Direct link to options-19" title="Direct link to options-19" translate="no">​</a></h6>
\<ul\>
<li class=""><code>--url \<URL\></code> — The URL of the server to add</li>
<li class=""><code>-d</code>, <code>--default</code> — Make the new server the default server for future operations</li>
<li class=""><code>--no-fingerprint</code> — Skip fingerprinting the server</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-server-remove"><code>spacetime server remove</code><a href="#spacetime-server-remove" class="hash-link" aria-label="Direct link to spacetime-server-remove" title="Direct link to spacetime-server-remove" translate="no">​</a></h2>
<p>Remove a saved server configuration</p>
<p><strong>Usage:</strong> <code>spacetime server remove [OPTIONS] \<server\></code></p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="arguments-15"><strong>Arguments:</strong><a href="#arguments-15" class="hash-link" aria-label="Direct link to arguments-15" title="Direct link to arguments-15" translate="no">​</a></h6>
\<ul\>
<li class=""><code>\<SERVER\></code> — The nickname, host name or URL of the server to remove</li>
</ul>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-20"><strong>Options:</strong><a href="#options-20" class="hash-link" aria-label="Direct link to options-20" title="Direct link to options-20" translate="no">​</a></h6>
\<ul\>
<li class=""><code>-y</code>, <code>--yes</code> — Run non-interactively wherever possible. This will answer "yes" to almost all prompts, but will sometimes answer "no" to preserve non-interactivity (e.g. when prompting whether to log in with spacetimedb.com).</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-server-fingerprint"><code>spacetime server fingerprint</code><a href="#spacetime-server-fingerprint" class="hash-link" aria-label="Direct link to spacetime-server-fingerprint" title="Direct link to spacetime-server-fingerprint" translate="no">​</a></h2>
<p>Show or update a saved server&#x27;s fingerprint</p>
<p><strong>Usage:</strong> <code>spacetime server fingerprint [OPTIONS] \<server\></code></p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="arguments-16"><strong>Arguments:</strong><a href="#arguments-16" class="hash-link" aria-label="Direct link to arguments-16" title="Direct link to arguments-16" translate="no">​</a></h6>
\<ul\>
<li class=""><code>\<SERVER\></code> — The nickname, host name or URL of the server</li>
</ul>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-21"><strong>Options:</strong><a href="#options-21" class="hash-link" aria-label="Direct link to options-21" title="Direct link to options-21" translate="no">​</a></h6>
\<ul\>
<li class=""><code>-y</code>, <code>--yes</code> — Run non-interactively wherever possible. This will answer "yes" to almost all prompts, but will sometimes answer "no" to preserve non-interactivity (e.g. when prompting whether to log in with spacetimedb.com).</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-server-ping"><code>spacetime server ping</code><a href="#spacetime-server-ping" class="hash-link" aria-label="Direct link to spacetime-server-ping" title="Direct link to spacetime-server-ping" translate="no">​</a></h2>
<p>Checks to see if a SpacetimeDB host is online</p>
<p><strong>Usage:</strong> <code>spacetime server ping \<server\></code></p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="arguments-17"><strong>Arguments:</strong><a href="#arguments-17" class="hash-link" aria-label="Direct link to arguments-17" title="Direct link to arguments-17" translate="no">​</a></h6>
\<ul\>
<li class=""><code>\<SERVER\></code> — The nickname, host name or URL of the server to ping</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-server-edit"><code>spacetime server edit</code><a href="#spacetime-server-edit" class="hash-link" aria-label="Direct link to spacetime-server-edit" title="Direct link to spacetime-server-edit" translate="no">​</a></h2>
<p>Update a saved server&#x27;s nickname, host name or protocol</p>
<p><strong>Usage:</strong> <code>spacetime server edit [OPTIONS] \<server\></code></p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="arguments-18"><strong>Arguments:</strong><a href="#arguments-18" class="hash-link" aria-label="Direct link to arguments-18" title="Direct link to arguments-18" translate="no">​</a></h6>
\<ul\>
<li class=""><code>\<SERVER\></code> — The nickname, host name or URL of the server</li>
</ul>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-22"><strong>Options:</strong><a href="#options-22" class="hash-link" aria-label="Direct link to options-22" title="Direct link to options-22" translate="no">​</a></h6>
\<ul\>
<li class=""><code>--new-name \<NICKNAME\></code> — A new nickname to assign the server configuration</li>
<li class=""><code>--url \<URL\></code> — A new URL to assign the server configuration</li>
<li class=""><code>--no-fingerprint</code> — Skip fingerprinting the server</li>
<li class=""><code>-y</code>, <code>--yes</code> — Run non-interactively wherever possible. This will answer "yes" to almost all prompts, but will sometimes answer "no" to preserve non-interactivity (e.g. when prompting whether to log in with spacetimedb.com).</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-server-clear"><code>spacetime server clear</code><a href="#spacetime-server-clear" class="hash-link" aria-label="Direct link to spacetime-server-clear" title="Direct link to spacetime-server-clear" translate="no">​</a></h2>
<p>Deletes all data from all local databases</p>
<p><strong>Usage:</strong> <code>spacetime server clear [OPTIONS]</code></p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-23"><strong>Options:</strong><a href="#options-23" class="hash-link" aria-label="Direct link to options-23" title="Direct link to options-23" translate="no">​</a></h6>
\<ul\>
<li class=""><code>--data-dir \<DATA_DIR\></code> — The path to the server data directory to clear [default: that of the selected spacetime instance]</li>
<li class=""><code>-y</code>, <code>--yes</code> — Run non-interactively wherever possible. This will answer "yes" to almost all prompts, but will sometimes answer "no" to preserve non-interactivity (e.g. when prompting whether to log in with spacetimedb.com).</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-subscribe"><code>spacetime subscribe</code><a href="#spacetime-subscribe" class="hash-link" aria-label="Direct link to spacetime-subscribe" title="Direct link to spacetime-subscribe" translate="no">​</a></h2>
<p>Subscribe to SQL queries on the database. WARNING: This command is UNSTABLE and subject to breaking changes.</p>
<p><strong>Usage:</strong> <code>spacetime subscribe [OPTIONS] [subscribe_parts]...</code></p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="arguments-19"><strong>Arguments:</strong><a href="#arguments-19" class="hash-link" aria-label="Direct link to arguments-19" title="Direct link to arguments-19" translate="no">​</a></h6>
\<ul\>
<li class=""><code>\<SUBSCRIBE_PARTS\></code> — Subscribe arguments: [DATABASE] \<QUERY\> [QUERY...]</li>
</ul>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-24"><strong>Options:</strong><a href="#options-24" class="hash-link" aria-label="Direct link to options-24" title="Direct link to options-24" translate="no">​</a></h6>
\<ul\>
<li class="">
<p><code>-n</code>, <code>--num-updates <NUM-UPDATES></code> — The number of subscription updates to receive before exiting</p>
</li>
<li class="">
<p><code>-t</code>, <code>--timeout \<TIMEOUT\></code> — The timeout, in seconds, after which to disconnect and stop receiving subscription messages. If <code>-n</code> is specified, it will stop after whichever one comes first. Timing out before receiving <code>-n</code> updates is an error.</p>
</li>
<li class="">
<p><code>--print-initial-update</code> — Print the initial update for the queries.</p>
</li>
<li class="">
<p><code>--confirmed \<CONFIRMED\></code> — Instruct the server to deliver only updates of confirmed transactions</p>
<p>Possible values: <code>true</code>, <code>false</code></p>
</li>
<li class="">
<p><code>--anonymous</code> — Perform this action with an anonymous identity</p>
</li>
<li class="">
<p><code>-y</code>, <code>--yes</code> — Run non-interactively wherever possible. This will answer "yes" to almost all prompts, but will sometimes answer "no" to preserve non-interactivity (e.g. when prompting whether to log in with spacetimedb.com).</p>
</li>
<li class="">
<p><code>--no-config</code> — Ignore spacetime.json configuration</p>
</li>
<li class="">
<p><code>-s</code>, <code>--server \<SERVER\></code> — The nickname, host name or URL of the server hosting the database</p>
</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-start"><code>spacetime start</code><a href="#spacetime-start" class="hash-link" aria-label="Direct link to spacetime-start" title="Direct link to spacetime-start" translate="no">​</a></h2>
<p>Start a local SpacetimeDB instance</p>
<p>Set a persistent default listen address in cli.toml with:
listen_addr = "0.0.0.0:4000"</p>
<p>When present, <code>listen_addr</code> is used unless <code>--listen-addr</code> is passed explicitly.</p>
<p>Run <code>spacetime start --help</code> to see all options.</p>
<p><strong>Usage:</strong> <code>spacetime start [OPTIONS] [args]...</code></p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="arguments-20"><strong>Arguments:</strong><a href="#arguments-20" class="hash-link" aria-label="Direct link to arguments-20" title="Direct link to arguments-20" translate="no">​</a></h6>
\<ul\>
<li class=""><code>\<ARGS\></code> — The args to pass to <code>spacetimedb-{edition} start</code></li>
</ul>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-25"><strong>Options:</strong><a href="#options-25" class="hash-link" aria-label="Direct link to options-25" title="Direct link to options-25" translate="no">​</a></h6>
\<ul\>
<li class="">
<p><code>--edition \<EDITION\></code> — The edition of SpacetimeDB to start up</p>
<p>Default value: <code>standalone</code></p>
<p>Possible values: <code>standalone</code>, <code>cloud</code></p>
</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-lock"><code>spacetime lock</code><a href="#spacetime-lock" class="hash-link" aria-label="Direct link to spacetime-lock" title="Direct link to spacetime-lock" translate="no">​</a></h2>
<p>Lock a database to prevent it from being deleted.</p>
<p>A locked database cannot be deleted until it is unlocked with <code>spacetime unlock</code>.
This is a safety mechanism to protect production databases from accidental deletion.</p>
<p><strong>Usage:</strong> <code>spacetime lock [OPTIONS] [database]</code></p>
<p>Run <code>spacetime help lock</code> for more detailed information.</p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="arguments-21"><strong>Arguments:</strong><a href="#arguments-21" class="hash-link" aria-label="Direct link to arguments-21" title="Direct link to arguments-21" translate="no">​</a></h6>
\<ul\>
<li class=""><code>\<DATABASE\></code> — The name or identity of the database to lock</li>
</ul>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-26"><strong>Options:</strong><a href="#options-26" class="hash-link" aria-label="Direct link to options-26" title="Direct link to options-26" translate="no">​</a></h6>
\<ul\>
<li class=""><code>-s</code>, <code>--server \<SERVER\></code> — The nickname, host name or URL of the server hosting the database</li>
<li class=""><code>--no-config</code> — Ignore spacetime.json configuration</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-unlock"><code>spacetime unlock</code><a href="#spacetime-unlock" class="hash-link" aria-label="Direct link to spacetime-unlock" title="Direct link to spacetime-unlock" translate="no">​</a></h2>
<p>Unlock a database that was previously locked with <code>spacetime lock</code>.</p>
<p>After unlocking, the database can be deleted normally with <code>spacetime delete</code>.</p>
<p><strong>Usage:</strong> <code>spacetime unlock [OPTIONS] [database]</code></p>
<p>Run <code>spacetime help unlock</code> for more detailed information.</p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="arguments-22"><strong>Arguments:</strong><a href="#arguments-22" class="hash-link" aria-label="Direct link to arguments-22" title="Direct link to arguments-22" translate="no">​</a></h6>
\<ul\>
<li class=""><code>\<DATABASE\></code> — The name or identity of the database to unlock</li>
</ul>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="options-27"><strong>Options:</strong><a href="#options-27" class="hash-link" aria-label="Direct link to options-27" title="Direct link to options-27" translate="no">​</a></h6>
\<ul\>
<li class=""><code>-s</code>, <code>--server \<SERVER\></code> — The nickname, host name or URL of the server hosting the database</li>
<li class=""><code>--no-config</code> — Ignore spacetime.json configuration</li>
</ul>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="spacetime-version"><code>spacetime version</code><a href="#spacetime-version" class="hash-link" aria-label="Direct link to spacetime-version" title="Direct link to spacetime-version" translate="no">​</a></h2>
<p>Manage installed spacetime versions</p>
<p>Run <code>spacetime version --help</code> to see all options.</p>
<p><strong>Usage:</strong> <code>spacetime version [ARGS]...</code></p>
<h6 class="anchor anchorTargetStickyNavbar_Y_NL" id="arguments-23"><strong>Arguments:</strong><a href="#arguments-23" class="hash-link" aria-label="Direct link to arguments-23" title="Direct link to arguments-23" translate="no">​</a></h6>
\<ul\>
<li class=""><code>\<ARGS\></code> — The args to pass to spacetimedb-update</li>
</ul>
\<hr\>
\<small\><i><p>This document was generated automatically by
<a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.</p></i></small></div><footer class="theme-doc-footer docusaurus-mt-lg"><div class="row margin-top--sm theme-doc-footer-edit-meta-row"><div class="col noPrint_OLXC"><a href="https://github.com/clockworklabs/SpacetimeDB/edit/master/docs/docs/00300-resources/00200-reference/00100-cli-reference/00100-cli-reference.md" target="_blank" rel="noopener noreferrer" class="theme-edit-this-page"><svg fill="currentColor" height="20" width="20" viewBox="0 0 40 40" class="iconEdit_kIlO" aria-hidden="true">\<g\><path d="m34.5 11.7l-3 3.1-6.3-6.3 3.1-3q0.5-0.5 1.2-0.5t1.1 0.5l3.9 3.9q0.5 0.4 0.5 1.1t-0.5 1.2z m-29.5 17.1l18.4-18.5 6.3 6.3-18.4 18.4h-6.3v-6.2z"></path></g></svg>Edit this page</a></div><div class="col lastUpdated_XxEB"></div></div></footer></article><nav class="docusaurus-mt-lg pagination-nav" aria-label="Docs pages"><a class="pagination-nav__link pagination-nav__link--prev" href="/docs/migrating-from-convex"><div class="pagination-nav__sublabel">Previous</div><div class="pagination-nav__label">Migrating from Convex</div></a><a class="pagination-nav__link pagination-nav__link--next" href="/docs/cli-reference/standalone-config"><div class="pagination-nav__sublabel">Next</div><div class="pagination-nav__label">Standalone Configuration</div></a></nav></div></div><div class="col col--3"><div class="tableOfContents_jOhJ thin-scrollbar theme-doc-toc-desktop"><ul class="table-of-contents table-of-contents__left-border">\<li\><a href="#spacetime" class="table-of-contents__link toc-highlight"><code>spacetime</code></a></li>\<li\><a href="#spacetime-publish" class="table-of-contents__link toc-highlight"><code>spacetime publish</code></a></li>\<li\><a href="#spacetime-env" class="table-of-contents__link toc-highlight"><code>spacetime env</code></a></li>\<li\><a href="#spacetime-env-get" class="table-of-contents__link toc-highlight"><code>spacetime env get</code></a></li>\<li\><a href="#spacetime-env-list" class="table-of-contents__link toc-highlight"><code>spacetime env list</code></a></li>\<li\><a href="#spacetime-delete" class="table-of-contents__link toc-highlight"><code>spacetime delete</code></a></li>\<li\><a href="#spacetime-logs" class="table-of-contents__link toc-highlight"><code>spacetime logs</code></a></li>\<li\><a href="#spacetime-call" class="table-of-contents__link toc-highlight"><code>spacetime call</code></a></li>\<li\><a href="#spacetime-describe" class="table-of-contents__link toc-highlight"><code>spacetime describe</code></a></li>\<li\><a href="#spacetime-dev" class="table-of-contents__link toc-highlight"><code>spacetime dev</code></a></li>\<li\><a href="#spacetime-sql" class="table-of-contents__link toc-highlight"><code>spacetime sql</code></a></li>\<li\><a href="#spacetime-mcp" class="table-of-contents__link toc-highlight"><code>spacetime mcp</code></a></li>\<li\><a href="#spacetime-rename" class="table-of-contents__link toc-highlight"><code>spacetime rename</code></a></li>\<li\><a href="#spacetime-generate" class="table-of-contents__link toc-highlight"><code>spacetime generate</code></a></li>\<li\><a href="#spacetime-list" class="table-of-contents__link toc-highlight"><code>spacetime list</code></a></li>\<li\><a href="#spacetime-login" class="table-of-contents__link toc-highlight"><code>spacetime login</code></a></li>\<li\><a href="#spacetime-login-show" class="table-of-contents__link toc-highlight"><code>spacetime login show</code></a></li>\<li\><a href="#spacetime-logout" class="table-of-contents__link toc-highlight"><code>spacetime logout</code></a></li>\<li\><a href="#spacetime-init" class="table-of-contents__link toc-highlight"><code>spacetime init</code></a></li>\<li\><a href="#spacetime-build" class="table-of-contents__link toc-highlight"><code>spacetime build</code></a></li>\<li\><a href="#spacetime-server" class="table-of-contents__link toc-highlight"><code>spacetime server</code></a></li>\<li\><a href="#spacetime-server-list" class="table-of-contents__link toc-highlight"><code>spacetime server list</code></a></li>\<li\><a href="#spacetime-server-set-default" class="table-of-contents__link toc-highlight"><code>spacetime server set-default</code></a></li>\<li\><a href="#spacetime-server-add" class="table-of-contents__link toc-highlight"><code>spacetime server add</code></a></li>\<li\><a href="#spacetime-server-remove" class="table-of-contents__link toc-highlight"><code>spacetime server remove</code></a></li>\<li\><a href="#spacetime-server-fingerprint" class="table-of-contents__link toc-highlight"><code>spacetime server fingerprint</code></a></li>\<li\><a href="#spacetime-server-ping" class="table-of-contents__link toc-highlight"><code>spacetime server ping</code></a></li>\<li\><a href="#spacetime-server-edit" class="table-of-contents__link toc-highlight"><code>spacetime server edit</code></a></li>\<li\><a href="#spacetime-server-clear" class="table-of-contents__link toc-highlight"><code>spacetime server clear</code></a></li>\<li\><a href="#spacetime-subscribe" class="table-of-contents__link toc-highlight"><code>spacetime subscribe</code></a></li>\<li\><a href="#spacetime-start" class="table-of-contents__link toc-highlight"><code>spacetime start</code></a></li>\<li\><a href="#spacetime-lock" class="table-of-contents__link toc-highlight"><code>spacetime lock</code></a></li>\<li\><a href="#spacetime-unlock" class="table-of-contents__link toc-highlight"><code>spacetime unlock</code></a></li>\<li\><a href="#spacetime-version" class="table-of-contents__link toc-highlight"><code>spacetime version</code></a></li></ul></div></div></div></div></main></div></div></div><div id="inkeep-shadowradix-:Rp:" style="display:contents"></div><span></span></div>
</body>
</html>