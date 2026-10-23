<!doctype html>
<html lang="en" dir="ltr" class="docs-wrapper plugin-docs plugin-id-default docs-version-current docs-doc-page docs-doc-id-resources/internals/module-abi-reference" data-has-hydrated="false">
\<head\>
<meta charset="UTF-8">
<meta name="generator" content="Docusaurus v3.9.2">
<title data-rh="true">Module ABI Reference | SpacetimeDB docs</title><meta data-rh="true" name="viewport" content="width=device-width,initial-scale=1"><meta data-rh="true" name="twitter:card" content="summary_large_image"><meta data-rh="true" property="og:url" content="https://spacetimedb.com/docs/webassembly-abi"><meta data-rh="true" property="og:locale" content="en"><meta data-rh="true" name="docusaurus_locale" content="en"><meta data-rh="true" name="docsearch:language" content="en"><meta data-rh="true" name="docusaurus_version" content="current"><meta data-rh="true" name="docusaurus_tag" content="docs-default-current"><meta data-rh="true" name="docsearch:version" content="current"><meta data-rh="true" name="docsearch:docusaurus_tag" content="docs-default-current"><meta data-rh="true" property="og:title" content="Module ABI Reference | SpacetimeDB docs"><meta data-rh="true" name="description" content="Low-level specification of how WebAssembly modules talk to the SpacetimeDB host. Most users should use the module SDKs instead."><meta data-rh="true" property="og:description" content="Low-level specification of how WebAssembly modules talk to the SpacetimeDB host. Most users should use the module SDKs instead."><link data-rh="true" rel="icon" href="https://spacetimedb.com/favicon-32x32.png"><link data-rh="true" rel="canonical" href="https://spacetimedb.com/docs/webassembly-abi"><link data-rh="true" rel="alternate" href="https://spacetimedb.com/docs/webassembly-abi" hreflang="en"><link data-rh="true" rel="alternate" href="https://spacetimedb.com/docs/webassembly-abi" hreflang="x-default"><script data-rh="true" type="application/ld+json">{"@context":"https://schema.org","@type":"BreadcrumbList","itemListElement":[{"@type":"ListItem","position":1,"name":"Module ABI Reference","item":"https://spacetimedb.com/docs/webassembly-abi"}]}</script><link rel="alternate" type="text/markdown" title="SpacetimeDB docs for agents" href="/docs/llms.txt">
<link rel="alternate" type="text/markdown" title="Full SpacetimeDB docs for agents" href="/docs/llms-full.txt">
<link rel="sitemap" type="application/xml" href="/docs/sitemap.xml">
<link rel="preload" as="font" type="font/woff2" href="/docs/fonts/inter-latin-wght-normal.woff2" crossorigin="anonymous">
<link rel="preload" as="font" type="font/woff2" href="/docs/fonts/source-code-pro-latin-wght-normal.woff2" crossorigin="anonymous"><link rel="stylesheet" href="/docs/assets/css/styles.4f000462.css">
<script src="/docs/assets/js/runtime~main.b38fc630.js" defer="defer"></script>
<script src="/docs/assets/js/main.2768f899.js" defer="defer"></script>
</head>
<body class="navigation-with-keyboard">
<svg style="display: none;">\<defs\>
<symbol id="theme-svg-external-link" viewBox="0 0 24 24"><path fill="currentColor" d="M21 13v10h-21v-19h12v2h-10v15h17v-8h2zm3-12h-10.988l4.035 4-6.977 7.07 2.828 2.828 6.977-7.07 4.125 4.172v-11z"/></symbol>
</defs></svg>
\<script\>document.documentElement.setAttribute("data-theme","light"),document.documentElement.setAttribute("data-theme-choice","light"),function(){try{const c=new URLSearchParams(window.location.search).entries();for(var[t,e]of c)if(t.startsWith("docusaurus-data-")){var a=t.replace("docusaurus-data-","data-");document.documentElement.setAttribute(a,e)}}catch(t){}}()</script><div id="__docusaurus"><div role="region" aria-label="Skip to main content"><a class="skipToContent_M6kL" href="#__docusaurus_skipToContent_fallback">Skip to main content</a></div><nav aria-label="Main" class="theme-layout-navbar navbar navbar--fixed-top"><div class="navbar__inner"><div class="theme-layout-navbar-left navbar__items"><button aria-label="Toggle navigation bar" aria-expanded="false" class="navbar__toggle clean-btn" type="button"><svg width="30" height="30" viewBox="0 0 30 30" aria-hidden="true"><path stroke="currentColor" stroke-linecap="round" stroke-miterlimit="10" stroke-width="2" d="M4 7h22M4 15h22M4 23h22"></path></svg></button><a href="https://spacetimedb.com" target="_self" rel="noopener noreferrer" class="navbar__brand"><div class="navbar__logo"><img src="https://spacetimedb.com/images/brand.svg" alt="Spacetime Logo" class="themedComponent_J5mh themedComponent--light_Du4S" height="32" width="133"><img src="https://spacetimedb.com/images/brand.svg" alt="Spacetime Logo" class="themedComponent_J5mh themedComponent--dark_W6YC" height="32" width="133"></div></a><div class="navbarSearchContainer_NRqO"><div id="inkeep-shadowradix-:Rnb5a5:" style="display:contents"></div><span></span></div><div class="navbar__item dropdown dropdown--hoverable"><a aria-current="page" class="navbar__link active" aria-haspopup="true" aria-expanded="false" role="button" href="/docs/webassembly-abi">2.0.0</a><ul class="dropdown__menu">\<li\><a aria-current="page" class="dropdown__link dropdown__link--active" href="/docs/webassembly-abi">2.0.0</a></li>\<li\><a class="dropdown__link" href="/docs/1.12.0/">1.12.0</a></li></ul></div></div><div class="theme-layout-navbar-right navbar__items navbar__items--right"><a href="https://spacetimedb.com/install" target="_blank" rel="noopener noreferrer" class="navbar__item navbar__link">Install</a><a href="https://spacetimedb.com/pricing" target="_blank" rel="noopener noreferrer" class="navbar__item navbar__link">Pricing</a><a href="https://spacetimedb.com/blog" target="_blank" rel="noopener noreferrer" class="navbar__item navbar__link">Blog</a><a href="https://spacetimedb.com/community" target="_blank" rel="noopener noreferrer" class="navbar__item navbar__link">Community</a><a href="https://spacetimedb.com/space-race" target="_blank" rel="noopener noreferrer" class="navbar__item navbar__link">Referrals</a><a href="https://spacetimedb.com/login" target="_blank" rel="noopener noreferrer" class="navbar__item navbar__link navbar__button">Login</a></div></div><div role="presentation" class="navbar-sidebar__backdrop"></div></nav><div id="__docusaurus_skipToContent_fallback" class="theme-layout-main main-wrapper mainWrapper_zieo"><div class="docsWrapper_LRYd"><button aria-label="Scroll back to top" class="clean-btn theme-back-to-top-button backToTopButton_uRlU" type="button"></button><div class="docRoot_xHAI"><aside class="theme-doc-sidebar-container docSidebarContainer_izyX"><div class="sidebarViewport_fAk8"><div class="sidebar_mXBm"><nav aria-label="Docs sidebar" class="menu thin-scrollbar menu_xs3F"><ul class="theme-doc-sidebar-menu menu__list"><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item sidebar-ask-ai"><a class="menu__link" href="/docs/ask-ai/ask-ai"><span title="Ask AI Chat" class="linkLabel_UvDg">Ask AI Chat</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1"><div class="spacetime-menu-header">Intro</div></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/"><span title="Getting Started" class="linkLabel_UvDg">Getting Started</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/intro/what-is-spacetimedb"><span title="What is SpacetimeDB?" class="linkLabel_UvDg">What is SpacetimeDB?</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/intro/zen"><span title="The Zen of Spacetime" class="linkLabel_UvDg">The Zen of Spacetime</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/intro/language-support"><span title="Language Support" class="linkLabel_UvDg">Language Support</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/intro/key-architecture"><span title="Key Architecture" class="linkLabel_UvDg">Key Architecture</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/intro/faq"><span title="FAQ" class="linkLabel_UvDg">FAQ</span></a></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist" href="/docs/quickstarts/react"><span title="Quickstarts" class="categoryLinkLabel_HATI">Quickstarts</span></a><button aria-label="Expand sidebar category &#x27;Quickstarts&#x27;" aria-expanded="false" type="button" class="clean-btn menu__caret"></button></div></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist" href="/docs/tutorials/chat-app"><span title="Tutorials" class="categoryLinkLabel_HATI">Tutorials</span></a><button aria-label="Expand sidebar category &#x27;Tutorials&#x27;" aria-expanded="false" type="button" class="clean-btn menu__caret"></button></div></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item spacetime-menu-header-link"><a class="menu__link" href="/docs/core-concepts"><span title="Core Concepts" class="linkLabel_UvDg">Core Concepts</span></a></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist" href="/docs/databases"><span title="Databases" class="categoryLinkLabel_HATI">Databases</span></a><button aria-label="Expand sidebar category &#x27;Databases&#x27;" aria-expanded="false" type="button" class="clean-btn menu__caret"></button></div></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist" href="/docs/functions"><span title="Functions" class="categoryLinkLabel_HATI">Functions</span></a><button aria-label="Expand sidebar category &#x27;Functions&#x27;" aria-expanded="false" type="button" class="clean-btn menu__caret"></button></div></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist" href="/docs/tables"><span title="Tables" class="categoryLinkLabel_HATI">Tables</span></a><button aria-label="Expand sidebar category &#x27;Tables&#x27;" aria-expanded="false" type="button" class="clean-btn menu__caret"></button></div></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist" href="/docs/core-concepts/authentication"><span title="Authentication" class="categoryLinkLabel_HATI">Authentication</span></a><button aria-label="Expand sidebar category &#x27;Authentication&#x27;" aria-expanded="false" type="button" class="clean-btn menu__caret"></button></div></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist" href="/docs/clients"><span title="Clients" class="categoryLinkLabel_HATI">Clients</span></a><button aria-label="Expand sidebar category &#x27;Clients&#x27;" aria-expanded="false" type="button" class="clean-btn menu__caret"></button></div></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item spacetime-menu-header-link"><a class="menu__link" href="/docs/how-to"><span title="How To Guides" class="linkLabel_UvDg">How To Guides</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/troubleshooting"><span title="Troubleshooting" class="linkLabel_UvDg">Troubleshooting</span></a></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist menu__link--sublist-caret" role="button" aria-expanded="false" href="/docs/how-to/deploy/maincloud"><span title="Deploy" class="categoryLinkLabel_HATI">Deploy</span></a></div></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/how-to/pg-wire"><span title="PostgreSQL Wire Protocol (PGWire)" class="linkLabel_UvDg">PostgreSQL Wire Protocol (PGWire)</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/how-to/logging"><span title="Logging" class="linkLabel_UvDg">Logging</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/how-to/rls"><span title="Row Level Security" class="linkLabel_UvDg">Row Level Security</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/how-to/reject-client-connections"><span title="Reject Client Connections" class="linkLabel_UvDg">Reject Client Connections</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/upgrade"><span title="Migrating from 1.0 to 2.0" class="linkLabel_UvDg">Migrating from 1.0 to 2.0</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/how-to/self-hosted-key-rotation"><span title="Azure Self-Hosted VMs + Key Rotation & Key Vault" class="linkLabel_UvDg">Azure Self-Hosted VMs + Key Rotation & Key Vault</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/migrating-from-convex"><span title="Migrating from Convex" class="linkLabel_UvDg">Migrating from Convex</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/how-to/unit-testing"><span title="Unit Testing" class="linkLabel_UvDg">Unit Testing</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item spacetime-menu-header-link"><a class="menu__link" href="/docs/resources"><span title="Developer Resources" class="linkLabel_UvDg">Developer Resources</span></a></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist menu__link--sublist-caret" role="button" aria-expanded="false" href="/docs/cli-reference"><span title="CLI Reference" class="categoryLinkLabel_HATI">CLI Reference</span></a></div></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/resources/mcp"><span title="MCP Reference" class="linkLabel_UvDg">MCP Reference</span></a></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist menu__link--sublist-caret" role="button" aria-expanded="false" href="/docs/http/authorization"><span title="HTTP API" class="categoryLinkLabel_HATI">HTTP API</span></a></div></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist menu__link--sublist-caret menu__link--active" role="button" aria-expanded="true" href="/docs/webassembly-abi"><span title="Internals" class="categoryLinkLabel_HATI">Internals</span></a></div><ul class="menu__list"><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-2 menu__list-item"><a class="menu__link menu__link--active" aria-current="page" tabindex="0" href="/docs/webassembly-abi"><span title="Module ABI Reference" class="linkLabel_UvDg">Module ABI Reference</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-2 menu__list-item"><a class="menu__link" tabindex="0" href="/docs/sats-json"><span title="SATS-JSON Data Format" class="linkLabel_UvDg">SATS-JSON Data Format</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-2 menu__list-item"><a class="menu__link" tabindex="0" href="/docs/bsatn"><span title="BSATN Data Format" class="linkLabel_UvDg">BSATN Data Format</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-2 menu__list-item"><a class="menu__link" tabindex="0" href="/docs/reference/internals/commitlog"><span title="Commitlog" class="linkLabel_UvDg">Commitlog</span></a></li></ul></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/reference/sql"><span title="SQL Reference" class="linkLabel_UvDg">SQL Reference</span></a></li></ul></nav></div></div></aside><main class="docMainContainer_dsjb"><div class="container padding-top--md padding-bottom--lg"><div class="row"><div class="col docItemCol_TFty"><div class="docItemContainer_LO0A">\<article\><nav class="theme-doc-breadcrumbs breadcrumbsContainer_EWVV" aria-label="Breadcrumbs"><ul class="breadcrumbs"><li class="breadcrumbs__item"><a aria-label="Home page" class="breadcrumbs__link" href="/docs/"><svg viewBox="0 0 24 24" class="breadcrumbHomeIcon_i5pl"><path d="M10 19v-5h4v5c0 .55.45 1 1 1h3c.55 0 1-.45 1-1v-7h1.7c.46 0 .68-.57.33-.87L12.67 3.6c-.38-.34-.96-.34-1.34 0l-8.36 7.53c-.34.3-.13.87.33.87H5v7c0 .55.45 1 1 1h3c.55 0 1-.45 1-1z" fill="currentColor"></path></svg></a></li><li class="breadcrumbs__item"><span class="breadcrumbs__link">Internals</span></li><li class="breadcrumbs__item breadcrumbs__item--active"><span class="breadcrumbs__link">Module ABI Reference</span></li></ul></nav><span class="theme-doc-version-badge badge badge--secondary">Version: 2.0.0</span><div class="tocCollapsible_ldQs theme-doc-toc-mobile tocMobile_xBih"><button type="button" class="clean-btn tocCollapsibleButton_Dk1U">On this page</button></div><div class="theme-doc-markdown markdown">\<header\>\<h1\>Module ABI Reference</h1></header>
<p>This document specifies the <em>low level details</em> of module-host interactions (<em>"Module ABI"</em>). <em><strong>Most users</strong></em> looking to interact with the host will want to use derived and higher level functionality like <a href="https://github.com/clockworklabs/SpacetimeDB/blob/master/crates/bindings/src/lib.rs" target="_blank" rel="noopener noreferrer" class=""><code>bindings</code></a>, <code>#[spacetimedb(table)]</code>, and <code>#[derive(SpacetimeType)]</code> rather than this low level ABI. For more on those, read the <a class="" href="/docs/quickstarts/rust">Rust module quick start</a> guide and the <a href="https://docs.rs/spacetimedb/latest/spacetimedb/" target="_blank" rel="noopener noreferrer" class="">Rust module reference</a>.</p>
<p>The Module ABI is defined in <a href="https://github.com/clockworklabs/SpacetimeDB/blob/master/crates/bindings-sys/src/lib.rs#L44-L215" target="_blank" rel="noopener noreferrer" class=""><code>bindings_sys::raw</code></a> and is used by modules to interact with their host and perform various operations like:</p>
\<ul\>
<li class="">logging,</li>
<li class="">transporting data,</li>
<li class="">scheduling reducers,</li>
<li class="">altering tables,</li>
<li class="">inserting and deleting rows,</li>
<li class="">querying tables.</li>
</ul>
<p>In the next few sections, we&#x27;ll define the functions that make up the ABI and what these functions do.</p>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="general-notes">General notes<a href="#general-notes" class="hash-link" aria-label="Direct link to General notes" title="Direct link to General notes" translate="no">​</a></h2>
<p>The functions in this ABI all use the <a href="https://github.com/WebAssembly/tool-conventions/blob/main/BasicCABI.md" target="_blank" rel="noopener noreferrer" class=""><code>C</code> ABI on the <code>wasm32</code> platform</a>. They are specified in a Rust <code>extern "C" { .. }</code> block. For those more familiar with the <code>C</code> notation, an <a href="#appendix-bindingsh" class="">appendix</a> is provided with equivalent definitions as would occur in a <code>.h</code> file.</p>
<p>Many functions in the ABI take in- or out-pointers, e.g. <code>*const u8</code> and <code>*mut u8</code>. The WASM host itself does not have undefined behavior. However, what WASM does not consider a memory access violation could be one according to some other language&#x27;s abstract machine. For example, running the following on a WASM host would violate Rust&#x27;s rules around writing across allocations:</p>
<pre tabindex="0" class="codeBlockStandalone_LlrK thin-scrollbar codeBlockContainer_hFbQ theme-code-block" style="--prism-color:#bfc7d5;--prism-background-color:#292d3e"><code class="codeBlockLines_JqGU"><span class="line"><span style="color:#FF79C6">fn</span><span style="color:#50FA7B"> main</span><span style="color:#F8F8F2">() {</span></span>
<span class="line"><span style="color:#FF79C6">    let</span><span style="color:#FF79C6"> mut</span><span style="color:#F8F8F2"> bytes </span><span style="color:#FF79C6">=</span><span style="color:#F8F8F2"> [</span><span style="color:#BD93F9">0</span><span style="color:#8BE9FD;font-style:italic">u8</span><span style="color:#F8F8F2">; </span><span style="color:#BD93F9">12</span><span style="color:#F8F8F2">];</span></span>
<span class="line"><span style="color:#FF79C6">    let</span><span style="color:#F8F8F2"> other_bytes </span><span style="color:#FF79C6">=</span><span style="color:#F8F8F2"> [</span><span style="color:#BD93F9">0</span><span style="color:#8BE9FD;font-style:italic">u8</span><span style="color:#F8F8F2">; </span><span style="color:#BD93F9">4</span><span style="color:#F8F8F2">];</span></span>
<span class="line"><span style="color:#FF79C6">    unsafe</span><span style="color:#F8F8F2"> { </span><span style="color:#50FA7B">ffi_func_with_out_ptr_and_len</span><span style="color:#F8F8F2">(</span><span style="color:#FF79C6">&mut</span><span style="color:#F8F8F2"> bytes </span><span style="color:#FF79C6">as</span><span style="color:#FF79C6"> *mut</span><span style="color:#8BE9FD;font-style:italic"> u8</span><span style="color:#F8F8F2">, </span><span style="color:#BD93F9">16</span><span style="color:#F8F8F2">); }</span></span>
<span class="line"><span style="color:#50FA7B">    assert_eq!</span><span style="color:#F8F8F2">(other_bytes, [</span><span style="color:#BD93F9">0</span><span style="color:#8BE9FD;font-style:italic">u8</span><span style="color:#F8F8F2">; </span><span style="color:#BD93F9">4</span><span style="color:#F8F8F2">]);</span></span>
<span class="line"><span style="color:#F8F8F2">}</span></span></code></pre>
<p>When we note in this reference that traps occur or errors are returned on memory access violations, we only mean those that WASM can directly detected, and not cases like the one above.</p>
<p>Should memory access violations occur, such as a buffer overrun, undefined behavior will never result, as it does not exist in WASM. However, in many cases, an error code will result.</p>
<p>Some functions will treat UTF-8 strings <em>lossily</em>. That is, if the slice identified by a <code>(ptr, len)</code> contains non-UTF-8 bytes, these bytes will be replaced with <code></code> in the read string.</p>
<p>Most functions return a <code>u16</code> value. This is how these functions indicate an error where a <code>0</code> value means that there were no errors. Such functions will instead return any data they need to through out pointers.</p>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="logging">Logging<a href="#logging" class="hash-link" aria-label="Direct link to Logging" title="Direct link to Logging" translate="no">​</a></h2>
<pre tabindex="0" class="codeBlockStandalone_LlrK thin-scrollbar codeBlockContainer_hFbQ theme-code-block" style="--prism-color:#bfc7d5;--prism-background-color:#292d3e"><code class="codeBlockLines_JqGU"><span class="line"><span style="color:#6272A4">/// The error log level.</span></span>
<span class="line"><span style="color:#FF79C6">const</span><span style="color:#BD93F9"> LOG_LEVEL_ERROR</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> u8</span><span style="color:#FF79C6"> =</span><span style="color:#BD93F9"> 0</span><span style="color:#F8F8F2">;</span></span>
<span class="line"><span style="color:#6272A4">/// The warn log level.</span></span>
<span class="line"><span style="color:#FF79C6">const</span><span style="color:#BD93F9"> LOG_LEVEL_WARN</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> u8</span><span style="color:#FF79C6"> =</span><span style="color:#BD93F9"> 1</span><span style="color:#F8F8F2">;</span></span>
<span class="line"><span style="color:#6272A4">/// The info log level.</span></span>
<span class="line"><span style="color:#FF79C6">const</span><span style="color:#BD93F9"> LOG_LEVEL_INFO</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> u8</span><span style="color:#FF79C6"> =</span><span style="color:#BD93F9"> 2</span><span style="color:#F8F8F2">;</span></span>
<span class="line"><span style="color:#6272A4">/// The debug log level.</span></span>
<span class="line"><span style="color:#FF79C6">const</span><span style="color:#BD93F9"> LOG_LEVEL_DEBUG</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> u8</span><span style="color:#FF79C6"> =</span><span style="color:#BD93F9"> 3</span><span style="color:#F8F8F2">;</span></span>
<span class="line"><span style="color:#6272A4">/// The trace log level.</span></span>
<span class="line"><span style="color:#FF79C6">const</span><span style="color:#BD93F9"> LOG_LEVEL_TRACE</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> u8</span><span style="color:#FF79C6"> =</span><span style="color:#BD93F9"> 4</span><span style="color:#F8F8F2">;</span></span>
<span class="line"><span style="color:#6272A4">/// The panic log level.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// A panic level is emitted just before</span></span>
<span class="line"><span style="color:#6272A4">/// a fatal error causes the WASM module to trap.</span></span>
<span class="line"><span style="color:#FF79C6">const</span><span style="color:#BD93F9"> LOG_LEVEL_PANIC</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> u8</span><span style="color:#FF79C6"> =</span><span style="color:#BD93F9"> 101</span><span style="color:#F8F8F2">;</span></span>
<span class="line"></span>
<span class="line"><span style="color:#6272A4">/// Log at `level` a `text` message occurring in `filename:line_number`</span></span>
<span class="line"><span style="color:#6272A4">/// with `target` being the module path at the `log!` invocation site.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// These various pointers are interpreted lossily as UTF-8 strings.</span></span>
<span class="line"><span style="color:#6272A4">/// The data pointed to are copied. Ownership does not transfer.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// See https://docs.rs/log/latest/log/struct.Record.html#method.target</span></span>
<span class="line"><span style="color:#6272A4">/// for more info on `target`.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// Calls to the function cannot fail</span></span>
<span class="line"><span style="color:#6272A4">/// irrespective of memory access violations.</span></span>
<span class="line"><span style="color:#6272A4">/// If they occur, no message is logged.</span></span>
<span class="line"><span style="color:#FF79C6">fn</span><span style="color:#50FA7B"> _console_log</span><span style="color:#F8F8F2">(</span></span>
<span class="line"><span style="color:#6272A4">    // The level we&#x27;re logging at.</span></span>
<span class="line"><span style="color:#6272A4">    // One of the `LOG_*` constants above.</span></span>
<span class="line"><span style="color:#F8F8F2">    level</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> u8</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // The module path, if any, associated with the message</span></span>
<span class="line"><span style="color:#6272A4">    // or to "blame" for the reason we&#x27;re logging.</span></span>
<span class="line"><span style="color:#6272A4">    //</span></span>
<span class="line"><span style="color:#6272A4">    // This is a pointer to a buffer holding an UTF-8 encoded string.</span></span>
<span class="line"><span style="color:#6272A4">    // When the pointer is `NULL`, `target` is ignored.</span></span>
<span class="line"><span style="color:#F8F8F2">    target</span><span style="color:#FF79C6">:</span><span style="color:#FF79C6"> *const</span><span style="color:#8BE9FD;font-style:italic"> u8</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // The length of the buffer pointed to by `text`.</span></span>
<span class="line"><span style="color:#6272A4">    // Unused when `target` is `NULL`.</span></span>
<span class="line"><span style="color:#F8F8F2">    target_len</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> usize</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // The file name, if any, associated with the message</span></span>
<span class="line"><span style="color:#6272A4">    // or to "blame" for the reason we&#x27;re logging.</span></span>
<span class="line"><span style="color:#6272A4">    //</span></span>
<span class="line"><span style="color:#6272A4">    // This is a pointer to a buffer holding an UTF-8 encoded string.</span></span>
<span class="line"><span style="color:#6272A4">    // When the pointer is `NULL`, `filename` is ignored.</span></span>
<span class="line"><span style="color:#F8F8F2">    filename</span><span style="color:#FF79C6">:</span><span style="color:#FF79C6"> *const</span><span style="color:#8BE9FD;font-style:italic"> u8</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // The length of the buffer pointed to by `text`.</span></span>
<span class="line"><span style="color:#6272A4">    // Unused when `filename` is `NULL`.</span></span>
<span class="line"><span style="color:#F8F8F2">    filename_len</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> usize</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // The line number associated with the message</span></span>
<span class="line"><span style="color:#6272A4">    // or to "blame" for the reason we&#x27;re logging.</span></span>
<span class="line"><span style="color:#F8F8F2">    line_number</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> u32</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // A pointer to a buffer holding an UTF-8 encoded message to log.</span></span>
<span class="line"><span style="color:#F8F8F2">    text</span><span style="color:#FF79C6">:</span><span style="color:#FF79C6"> *const</span><span style="color:#8BE9FD;font-style:italic"> u8</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // The length of the buffer pointed to by `text`.</span></span>
<span class="line"><span style="color:#F8F8F2">    text_len</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> usize</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#F8F8F2">);</span></span></code></pre>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="buffer-handling">Buffer handling<a href="#buffer-handling" class="hash-link" aria-label="Direct link to Buffer handling" title="Direct link to Buffer handling" translate="no">​</a></h2>
<pre tabindex="0" class="codeBlockStandalone_LlrK thin-scrollbar codeBlockContainer_hFbQ theme-code-block" style="--prism-color:#bfc7d5;--prism-background-color:#292d3e"><code class="codeBlockLines_JqGU"><span class="line"><span style="color:#6272A4">/// Returns the length of buffer `bufh` without</span></span>
<span class="line"><span style="color:#6272A4">/// transferring ownership of the data into the function.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// The `bufh` must have previously been allocating using `_buffer_alloc`.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// Traps if the buffer does not exist.</span></span>
<span class="line"><span style="color:#FF79C6">fn</span><span style="color:#50FA7B"> _buffer_len</span><span style="color:#F8F8F2">(</span></span>
<span class="line"><span style="color:#6272A4">    // The buffer previously allocated using `_buffer_alloc`.</span></span>
<span class="line"><span style="color:#6272A4">    // Ownership of the buffer is not taken.</span></span>
<span class="line"><span style="color:#F8F8F2">    bufh</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> ManuallyDrop</span><span style="color:#F8F8F2"><</span><span style="color:#8BE9FD;font-style:italic">Buffer</span><span style="color:#F8F8F2">></span></span>
<span class="line"><span style="color:#F8F8F2">) </span><span style="color:#FF79C6">-></span><span style="color:#8BE9FD;font-style:italic"> usize</span><span style="color:#F8F8F2">;</span></span>
<span class="line"></span>
<span class="line"><span style="color:#6272A4">/// Consumes the buffer `bufh`,</span></span>
<span class="line"><span style="color:#6272A4">/// moving its contents to the WASM byte slice `(ptr, len)`.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// Returns an error if the buffer does not exist</span></span>
<span class="line"><span style="color:#6272A4">/// or on any memory access violations associated with `(ptr, len)`.</span></span>
<span class="line"><span style="color:#FF79C6">fn</span><span style="color:#50FA7B"> _buffer_consume</span><span style="color:#F8F8F2">(</span></span>
<span class="line"><span style="color:#6272A4">    // The buffer to consume and move into `(ptr, len)`.</span></span>
<span class="line"><span style="color:#6272A4">    // Ownership of the buffer and its contents are taken.</span></span>
<span class="line"><span style="color:#6272A4">    // That is, `bufh` won&#x27;t be usable after this call.</span></span>
<span class="line"><span style="color:#F8F8F2">    bufh</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> Buffer</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // A WASM out pointer to write the contents of `bufh` to.</span></span>
<span class="line"><span style="color:#F8F8F2">    ptr</span><span style="color:#FF79C6">:</span><span style="color:#FF79C6"> *mut</span><span style="color:#8BE9FD;font-style:italic"> u8</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // The size of the buffer pointed to by `ptr`.</span></span>
<span class="line"><span style="color:#6272A4">    // This size must match that of `bufh` or a trap will occur.</span></span>
<span class="line"><span style="color:#F8F8F2">    len</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> usize</span></span>
<span class="line"><span style="color:#F8F8F2">);</span></span>
<span class="line"></span>
<span class="line"><span style="color:#6272A4">/// Creates a buffer of size `data_len` in the host environment.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// The contents of the byte slice lasting `data_len` bytes</span></span>
<span class="line"><span style="color:#6272A4">/// at the `data` WASM pointer are read</span></span>
<span class="line"><span style="color:#6272A4">/// and written into the newly initialized buffer.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// Traps on any memory access violations.</span></span>
<span class="line"><span style="color:#FF79C6">fn</span><span style="color:#50FA7B"> _buffer_alloc</span><span style="color:#F8F8F2">(data</span><span style="color:#FF79C6">:</span><span style="color:#FF79C6"> *const</span><span style="color:#8BE9FD;font-style:italic"> u8</span><span style="color:#F8F8F2">, data_len</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> usize</span><span style="color:#F8F8F2">) </span><span style="color:#FF79C6">-></span><span style="color:#8BE9FD;font-style:italic"> Buffer</span><span style="color:#F8F8F2">;</span></span></code></pre>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="reducer-scheduling">Reducer scheduling<a href="#reducer-scheduling" class="hash-link" aria-label="Direct link to Reducer scheduling" title="Direct link to Reducer scheduling" translate="no">​</a></h2>
<pre tabindex="0" class="codeBlockStandalone_LlrK thin-scrollbar codeBlockContainer_hFbQ theme-code-block" style="--prism-color:#bfc7d5;--prism-background-color:#292d3e"><code class="codeBlockLines_JqGU"><span class="line"><span style="color:#6272A4">/// Schedules a reducer to be called asynchronously at `time`.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// The reducer is named as the valid UTF-8 slice `(name, name_len)`,</span></span>
<span class="line"><span style="color:#6272A4">/// and is passed the slice `(args, args_len)` as its argument.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// A generated schedule id is assigned to the reducer.</span></span>
<span class="line"><span style="color:#6272A4">/// This id is written to the pointer `out`.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// Errors on any memory access violations,</span></span>
<span class="line"><span style="color:#6272A4">/// if `(name, name_len)` does not point to valid UTF-8,</span></span>
<span class="line"><span style="color:#6272A4">/// or if the `time` delay exceeds `64^6 - 1` milliseconds from now.</span></span>
<span class="line"><span style="color:#FF79C6">fn</span><span style="color:#50FA7B"> _schedule_reducer</span><span style="color:#F8F8F2">(</span></span>
<span class="line"><span style="color:#6272A4">    // A pointer to a buffer</span></span>
<span class="line"><span style="color:#6272A4">    // with a valid UTF-8 string of `name_len` many bytes.</span></span>
<span class="line"><span style="color:#F8F8F2">    name</span><span style="color:#FF79C6">:</span><span style="color:#FF79C6"> *const</span><span style="color:#8BE9FD;font-style:italic"> u8</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // The number of bytes in the `name` buffer.</span></span>
<span class="line"><span style="color:#F8F8F2">    name_len</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> usize</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // A pointer to a byte buffer of `args_len` many bytes.</span></span>
<span class="line"><span style="color:#F8F8F2">    args</span><span style="color:#FF79C6">:</span><span style="color:#FF79C6"> *const</span><span style="color:#8BE9FD;font-style:italic"> u8</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // The number of bytes in the `args` buffer.</span></span>
<span class="line"><span style="color:#F8F8F2">    args_len</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> usize</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // When to call the reducer.</span></span>
<span class="line"><span style="color:#F8F8F2">    time</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> u64</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // The schedule ID is written to this out pointer on a successful call.</span></span>
<span class="line"><span style="color:#F8F8F2">    out</span><span style="color:#FF79C6">:</span><span style="color:#FF79C6"> *mut</span><span style="color:#8BE9FD;font-style:italic"> u64</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#F8F8F2">);</span></span>
<span class="line"></span>
<span class="line"><span style="color:#6272A4">/// Unschedules a reducer</span></span>
<span class="line"><span style="color:#6272A4">/// using the same `id` generated as when it was scheduled.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// This assumes that the reducer hasn&#x27;t already been executed.</span></span>
<span class="line"><span style="color:#FF79C6">fn</span><span style="color:#50FA7B"> _cancel_reducer</span><span style="color:#F8F8F2">(id</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> u64</span><span style="color:#F8F8F2">);</span></span></code></pre>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="altering-tables">Altering tables<a href="#altering-tables" class="hash-link" aria-label="Direct link to Altering tables" title="Direct link to Altering tables" translate="no">​</a></h2>
<pre tabindex="0" class="codeBlockStandalone_LlrK thin-scrollbar codeBlockContainer_hFbQ theme-code-block" style="--prism-color:#bfc7d5;--prism-background-color:#292d3e"><code class="codeBlockLines_JqGU"><span class="line"><span style="color:#6272A4">/// Creates an index with the name `index_name` and type `index_type`,</span></span>
<span class="line"><span style="color:#6272A4">/// on a product of the given columns in `col_ids`</span></span>
<span class="line"><span style="color:#6272A4">/// in the table identified by `table_id`.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// Here `index_name` points to a UTF-8 slice in WASM memory</span></span>
<span class="line"><span style="color:#6272A4">/// and `col_ids` points to a byte slice in WASM memory</span></span>
<span class="line"><span style="color:#6272A4">/// with each element being a column.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// Currently only single-column-indices are supported</span></span>
<span class="line"><span style="color:#6272A4">/// and they may only be of the btree index type.</span></span>
<span class="line"><span style="color:#6272A4">/// In the former case, the function will panic,</span></span>
<span class="line"><span style="color:#6272A4">/// and in latter, an error is returned.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// Returns an error on any memory access violations,</span></span>
<span class="line"><span style="color:#6272A4">/// if `(index_name, index_name_len)` is not valid UTF-8,</span></span>
<span class="line"><span style="color:#6272A4">/// or when a table with the provided `table_id` doesn&#x27;t exist.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// Traps if `index_type /= 0` or if `col_len /= 1`.</span></span>
<span class="line"><span style="color:#FF79C6">fn</span><span style="color:#50FA7B"> _create_index</span><span style="color:#F8F8F2">(</span></span>
<span class="line"><span style="color:#6272A4">    // A pointer to a buffer holding an UTF-8 encoded index name.</span></span>
<span class="line"><span style="color:#F8F8F2">    index_name</span><span style="color:#FF79C6">:</span><span style="color:#FF79C6"> *const</span><span style="color:#8BE9FD;font-style:italic"> u8</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // The length of the buffer pointed to by `index_name`.</span></span>
<span class="line"><span style="color:#F8F8F2">    index_name_len</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> usize</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // The ID of the table to create the index for.</span></span>
<span class="line"><span style="color:#F8F8F2">    table_id</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> u32</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // The type of the index.</span></span>
<span class="line"><span style="color:#6272A4">    // Must be `0` currently, that is, a btree-index.</span></span>
<span class="line"><span style="color:#F8F8F2">    index_type</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> u8</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // A pointer to a buffer holding a byte slice</span></span>
<span class="line"><span style="color:#6272A4">    // where each element is the position</span></span>
<span class="line"><span style="color:#6272A4">    // of a column to include in the index.</span></span>
<span class="line"><span style="color:#F8F8F2">    col_ids</span><span style="color:#FF79C6">:</span><span style="color:#FF79C6"> *const</span><span style="color:#8BE9FD;font-style:italic"> u8</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // The length of the byte slice in `col_ids`. Must be `1`.</span></span>
<span class="line"><span style="color:#F8F8F2">    col_len</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> usize</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#F8F8F2">) </span><span style="color:#FF79C6">-></span><span style="color:#8BE9FD;font-style:italic"> u16</span><span style="color:#F8F8F2">;</span></span></code></pre>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="inserting-and-deleting-rows">Inserting and deleting rows<a href="#inserting-and-deleting-rows" class="hash-link" aria-label="Direct link to Inserting and deleting rows" title="Direct link to Inserting and deleting rows" translate="no">​</a></h2>
<pre tabindex="0" class="codeBlockStandalone_LlrK thin-scrollbar codeBlockContainer_hFbQ theme-code-block" style="--prism-color:#bfc7d5;--prism-background-color:#292d3e"><code class="codeBlockLines_JqGU"><span class="line"><span style="color:#6272A4">/// Inserts a row into the table identified by `table_id`,</span></span>
<span class="line"><span style="color:#6272A4">/// where the row is read from the byte slice `row_ptr` in WASM memory,</span></span>
<span class="line"><span style="color:#6272A4">/// lasting `row_len` bytes.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// Errors if there were unique constraint violations,</span></span>
<span class="line"><span style="color:#6272A4">/// if there were any memory access violations in associated with `row`,</span></span>
<span class="line"><span style="color:#6272A4">/// if the `table_id` doesn&#x27;t identify a table,</span></span>
<span class="line"><span style="color:#6272A4">/// or if `(row, row_len)` doesn&#x27;t decode from BSATN to a `ProductValue`</span></span>
<span class="line"><span style="color:#6272A4">/// according to the `ProductType` that the table&#x27;s schema specifies.</span></span>
<span class="line"><span style="color:#FF79C6">fn</span><span style="color:#50FA7B"> _insert</span><span style="color:#F8F8F2">(</span></span>
<span class="line"><span style="color:#6272A4">    // The table to insert the row into.</span></span>
<span class="line"><span style="color:#6272A4">    // The interpretation of `(row, row_len)` depends on this ID</span></span>
<span class="line"><span style="color:#6272A4">    // as it&#x27;s table schema determines how to decode the raw bytes.</span></span>
<span class="line"><span style="color:#F8F8F2">    table_id</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> u32</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // An in/out pointer to a byte buffer</span></span>
<span class="line"><span style="color:#6272A4">    // holding the BSATN-encoded `ProductValue` row data to insert.</span></span>
<span class="line"><span style="color:#6272A4">    //</span></span>
<span class="line"><span style="color:#6272A4">    // The pointer is written to with the inserted row re-encoded.</span></span>
<span class="line"><span style="color:#6272A4">    // This is due to auto-incrementing columns.</span></span>
<span class="line"><span style="color:#F8F8F2">    row</span><span style="color:#FF79C6">:</span><span style="color:#FF79C6"> *mut</span><span style="color:#8BE9FD;font-style:italic"> u8</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // The length of the buffer pointed to by `row`.</span></span>
<span class="line"><span style="color:#F8F8F2">    row_len</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> usize</span></span>
<span class="line"><span style="color:#F8F8F2">) </span><span style="color:#FF79C6">-></span><span style="color:#8BE9FD;font-style:italic"> u16</span><span style="color:#F8F8F2">;</span></span>
<span class="line"></span>
<span class="line"><span style="color:#6272A4">/// Deletes all rows in the table identified by `table_id`</span></span>
<span class="line"><span style="color:#6272A4">/// where the column identified by `col_id` matches the byte string,</span></span>
<span class="line"><span style="color:#6272A4">/// in WASM memory, pointed to by `value`.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// Matching is defined by decoding of `value` to an `AlgebraicValue`</span></span>
<span class="line"><span style="color:#6272A4">/// according to the column&#x27;s schema and then `Ord for AlgebraicValue`.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// The number of rows deleted is written to the WASM pointer `out`.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// Errors if there were memory access violations</span></span>
<span class="line"><span style="color:#6272A4">/// associated with `value` or `out`,</span></span>
<span class="line"><span style="color:#6272A4">/// if no columns were deleted,</span></span>
<span class="line"><span style="color:#6272A4">/// or if the column wasn&#x27;t found.</span></span>
<span class="line"><span style="color:#FF79C6">fn</span><span style="color:#50FA7B"> _delete_by_col_eq</span><span style="color:#F8F8F2">(</span></span>
<span class="line"><span style="color:#6272A4">    // The table to delete rows from.</span></span>
<span class="line"><span style="color:#F8F8F2">    table_id</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> u32</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // The position of the column to match `(value, value_len)` against.</span></span>
<span class="line"><span style="color:#F8F8F2">    col_id</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> u32</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // A pointer to a byte buffer holding a BSATN-encoded `AlgebraicValue`</span></span>
<span class="line"><span style="color:#6272A4">    // of the `AlgebraicType` that the table&#x27;s schema specifies</span></span>
<span class="line"><span style="color:#6272A4">    // for the column identified by `col_id`.</span></span>
<span class="line"><span style="color:#F8F8F2">    value</span><span style="color:#FF79C6">:</span><span style="color:#FF79C6"> *const</span><span style="color:#8BE9FD;font-style:italic"> u8</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // The length of the buffer pointed to by `value`.</span></span>
<span class="line"><span style="color:#F8F8F2">    value_len</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> usize</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // An out pointer that the number of rows deleted is written to.</span></span>
<span class="line"><span style="color:#F8F8F2">    out</span><span style="color:#FF79C6">:</span><span style="color:#FF79C6"> *mut</span><span style="color:#8BE9FD;font-style:italic"> u32</span></span>
<span class="line"><span style="color:#F8F8F2">) </span><span style="color:#FF79C6">-></span><span style="color:#8BE9FD;font-style:italic"> u16</span><span style="color:#F8F8F2">;</span></span></code></pre>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="querying-tables">Querying tables<a href="#querying-tables" class="hash-link" aria-label="Direct link to Querying tables" title="Direct link to Querying tables" translate="no">​</a></h2>
<pre tabindex="0" class="codeBlockStandalone_LlrK thin-scrollbar codeBlockContainer_hFbQ theme-code-block" style="--prism-color:#bfc7d5;--prism-background-color:#292d3e"><code class="codeBlockLines_JqGU"><span class="line"><span style="color:#6272A4">/// Queries the `table_id` associated with the given (table) `name`</span></span>
<span class="line"><span style="color:#6272A4">/// where `name` points to a UTF-8 slice</span></span>
<span class="line"><span style="color:#6272A4">/// in WASM memory of `name_len` bytes.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// The table id is written into the `out` pointer.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// Errors on memory access violations associated with `name`</span></span>
<span class="line"><span style="color:#6272A4">/// or if the table does not exist.</span></span>
<span class="line"><span style="color:#FF79C6">fn</span><span style="color:#50FA7B"> _get_table_id</span><span style="color:#F8F8F2">(</span></span>
<span class="line"><span style="color:#6272A4">    // A pointer to a buffer holding the name of the table</span></span>
<span class="line"><span style="color:#6272A4">    // as a valid UTF-8 encoded string.</span></span>
<span class="line"><span style="color:#F8F8F2">    name</span><span style="color:#FF79C6">:</span><span style="color:#FF79C6"> *const</span><span style="color:#8BE9FD;font-style:italic"> u8</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // The length of the buffer pointed to by `name`.</span></span>
<span class="line"><span style="color:#F8F8F2">    name_len</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> usize</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // An out pointer to write the table ID to.</span></span>
<span class="line"><span style="color:#F8F8F2">    out</span><span style="color:#FF79C6">:</span><span style="color:#FF79C6"> *mut</span><span style="color:#8BE9FD;font-style:italic"> u32</span></span>
<span class="line"><span style="color:#F8F8F2">) </span><span style="color:#FF79C6">-></span><span style="color:#8BE9FD;font-style:italic"> u16</span><span style="color:#F8F8F2">;</span></span>
<span class="line"></span>
<span class="line"><span style="color:#6272A4">/// Finds all rows in the table identified by `table_id`,</span></span>
<span class="line"><span style="color:#6272A4">/// where the row has a column, identified by `col_id`,</span></span>
<span class="line"><span style="color:#6272A4">/// with data matching the byte string,</span></span>
<span class="line"><span style="color:#6272A4">/// in WASM memory, pointed to at by `val`.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// Matching is defined by decoding of `value`</span></span>
<span class="line"><span style="color:#6272A4">/// to an `AlgebraicValue` according to the column&#x27;s schema</span></span>
<span class="line"><span style="color:#6272A4">/// and then `Ord for AlgebraicValue`.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// The rows found are BSATN encoded and then concatenated.</span></span>
<span class="line"><span style="color:#6272A4">/// The resulting byte string from the concatenation</span></span>
<span class="line"><span style="color:#6272A4">/// is written to a fresh buffer</span></span>
<span class="line"><span style="color:#6272A4">/// with the buffer&#x27;s identifier written to the WASM pointer `out`.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// Errors if no table with `table_id` exists,</span></span>
<span class="line"><span style="color:#6272A4">/// if `col_id` does not identify a column of the table,</span></span>
<span class="line"><span style="color:#6272A4">/// if `(value, value_len)` cannot be decoded to an `AlgebraicValue`</span></span>
<span class="line"><span style="color:#6272A4">/// typed at the `AlgebraicType` of the column,</span></span>
<span class="line"><span style="color:#6272A4">/// or if memory access violations occurred associated with `value` or `out`.</span></span>
<span class="line"><span style="color:#FF79C6">fn</span><span style="color:#50FA7B"> _iter_by_col_eq</span><span style="color:#F8F8F2">(</span></span>
<span class="line"><span style="color:#6272A4">    // Identifies the table to find rows in.</span></span>
<span class="line"><span style="color:#F8F8F2">    table_id</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> u32</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // The position of the column in the table</span></span>
<span class="line"><span style="color:#6272A4">    // to match `(value, value_len)` against.</span></span>
<span class="line"><span style="color:#F8F8F2">    col_id</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> u32</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // A pointer to a byte buffer holding a BSATN encoded</span></span>
<span class="line"><span style="color:#6272A4">    // value typed at the `AlgebraicType` of the column.</span></span>
<span class="line"><span style="color:#F8F8F2">    value</span><span style="color:#FF79C6">:</span><span style="color:#FF79C6"> *const</span><span style="color:#8BE9FD;font-style:italic"> u8</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // The length of the buffer pointed to by `value`.</span></span>
<span class="line"><span style="color:#F8F8F2">    value_len</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> usize</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // An out pointer to which the new buffer&#x27;s id is written to.</span></span>
<span class="line"><span style="color:#F8F8F2">    out</span><span style="color:#FF79C6">:</span><span style="color:#FF79C6"> *mut</span><span style="color:#8BE9FD;font-style:italic"> Buffer</span></span>
<span class="line"><span style="color:#F8F8F2">) </span><span style="color:#FF79C6">-></span><span style="color:#8BE9FD;font-style:italic"> u16</span><span style="color:#F8F8F2">;</span></span>
<span class="line"></span>
<span class="line"><span style="color:#6272A4">/// Starts iteration on each row, as bytes,</span></span>
<span class="line"><span style="color:#6272A4">/// of a table identified by `table_id`.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// The iterator is registered in the host environment</span></span>
<span class="line"><span style="color:#6272A4">/// under an assigned index which is written to the `out` pointer provided.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// Errors if the table doesn&#x27;t exist</span></span>
<span class="line"><span style="color:#6272A4">/// or if memory access violations occurred in association with `out`.</span></span>
<span class="line"><span style="color:#FF79C6">fn</span><span style="color:#50FA7B"> _iter_start</span><span style="color:#F8F8F2">(</span></span>
<span class="line"><span style="color:#6272A4">    // The ID of the table to start row iteration on.</span></span>
<span class="line"><span style="color:#F8F8F2">    table_id</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> u32</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // An out pointer to which an identifier</span></span>
<span class="line"><span style="color:#6272A4">    // to the newly created buffer is written.</span></span>
<span class="line"><span style="color:#F8F8F2">    out</span><span style="color:#FF79C6">:</span><span style="color:#FF79C6"> *mut</span><span style="color:#8BE9FD;font-style:italic"> BufferIter</span></span>
<span class="line"><span style="color:#F8F8F2">) </span><span style="color:#FF79C6">-></span><span style="color:#8BE9FD;font-style:italic"> u16</span><span style="color:#F8F8F2">;</span></span>
<span class="line"></span>
<span class="line"><span style="color:#6272A4">/// Like [`_iter_start`], starts iteration on each row,</span></span>
<span class="line"><span style="color:#6272A4">/// as bytes, of a table identified by `table_id`.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// The rows are filtered through `filter`, which is read from WASM memory</span></span>
<span class="line"><span style="color:#6272A4">/// and is encoded in the embedded language defined by `spacetimedb_lib::filter::Expr`.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// The iterator is registered in the host environment</span></span>
<span class="line"><span style="color:#6272A4">/// under an assigned index which is written to the `out` pointer provided.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// Errors if `table_id` doesn&#x27;t identify a table,</span></span>
<span class="line"><span style="color:#6272A4">/// if `(filter, filter_len)` doesn&#x27;t decode to a filter expression,</span></span>
<span class="line"><span style="color:#6272A4">/// or if there were memory access violations</span></span>
<span class="line"><span style="color:#6272A4">/// in association with `filter` or `out`.</span></span>
<span class="line"><span style="color:#FF79C6">fn</span><span style="color:#50FA7B"> _iter_start_filtered</span><span style="color:#F8F8F2">(</span></span>
<span class="line"><span style="color:#6272A4">    // The ID of the table to start row iteration on.</span></span>
<span class="line"><span style="color:#F8F8F2">    table_id</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> u32</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // A pointer to a buffer holding an encoded filter expression.</span></span>
<span class="line"><span style="color:#F8F8F2">    filter</span><span style="color:#FF79C6">:</span><span style="color:#FF79C6"> *const</span><span style="color:#8BE9FD;font-style:italic"> u8</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // The length of the buffer pointed to by `filter`.</span></span>
<span class="line"><span style="color:#F8F8F2">    filter_len</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> usize</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#6272A4">    // An out pointer to which an identifier</span></span>
<span class="line"><span style="color:#6272A4">    // to the newly created buffer is written.</span></span>
<span class="line"><span style="color:#F8F8F2">    out</span><span style="color:#FF79C6">:</span><span style="color:#FF79C6"> *mut</span><span style="color:#8BE9FD;font-style:italic"> BufferIter</span></span>
<span class="line"><span style="color:#F8F8F2">) </span><span style="color:#FF79C6">-></span><span style="color:#8BE9FD;font-style:italic"> u16</span><span style="color:#F8F8F2">;</span></span>
<span class="line"></span>
<span class="line"><span style="color:#6272A4">/// Advances the registered iterator with the index given by `iter_key`.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// On success, the next element (the row as bytes) is written to a buffer.</span></span>
<span class="line"><span style="color:#6272A4">/// The buffer&#x27;s index is returned and written to the `out` pointer.</span></span>
<span class="line"><span style="color:#6272A4">/// If there are no elements left, an invalid buffer index is written to `out`.</span></span>
<span class="line"><span style="color:#6272A4">/// On failure however, the error is returned.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// Errors if `iter` does not identify a registered `BufferIter`,</span></span>
<span class="line"><span style="color:#6272A4">/// or if there were memory access violations in association with `out`.</span></span>
<span class="line"><span style="color:#FF79C6">fn</span><span style="color:#50FA7B"> _iter_next</span><span style="color:#F8F8F2">(</span></span>
<span class="line"><span style="color:#6272A4">    // An identifier for the iterator buffer to advance.</span></span>
<span class="line"><span style="color:#6272A4">    // Ownership of the buffer nor the identifier is moved into the function.</span></span>
<span class="line"><span style="color:#F8F8F2">    iter</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> ManuallyDrop</span><span style="color:#F8F8F2"><</span><span style="color:#8BE9FD;font-style:italic">BufferIter</span><span style="color:#F8F8F2">>,</span></span>
<span class="line"><span style="color:#6272A4">    // An out pointer to write the newly created buffer&#x27;s identifier to.</span></span>
<span class="line"><span style="color:#F8F8F2">    out</span><span style="color:#FF79C6">:</span><span style="color:#FF79C6"> *mut</span><span style="color:#8BE9FD;font-style:italic"> Buffer</span></span>
<span class="line"><span style="color:#F8F8F2">) </span><span style="color:#FF79C6">-></span><span style="color:#8BE9FD;font-style:italic"> u16</span><span style="color:#F8F8F2">;</span></span>
<span class="line"></span>
<span class="line"><span style="color:#6272A4">/// Drops the entire registered iterator with the index given by `iter_key`.</span></span>
<span class="line"><span style="color:#6272A4">/// The iterator is effectively de-registered.</span></span>
<span class="line"><span style="color:#6272A4">///</span></span>
<span class="line"><span style="color:#6272A4">/// Returns an error if the iterator does not exist.</span></span>
<span class="line"><span style="color:#FF79C6">fn</span><span style="color:#50FA7B"> _iter_drop</span><span style="color:#F8F8F2">(</span></span>
<span class="line"><span style="color:#6272A4">    // An identifier for the iterator buffer to unregister / drop.</span></span>
<span class="line"><span style="color:#F8F8F2">    iter</span><span style="color:#FF79C6">:</span><span style="color:#8BE9FD;font-style:italic"> ManuallyDrop</span><span style="color:#F8F8F2"><</span><span style="color:#8BE9FD;font-style:italic">BufferIter</span><span style="color:#F8F8F2">></span></span>
<span class="line"><span style="color:#F8F8F2">) </span><span style="color:#FF79C6">-></span><span style="color:#8BE9FD;font-style:italic"> u16</span><span style="color:#F8F8F2">;</span></span></code></pre>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="appendix-bindingsh">Appendix, <code>bindings.h</code><a href="#appendix-bindingsh" class="hash-link" aria-label="Direct link to appendix-bindingsh" title="Direct link to appendix-bindingsh" translate="no">​</a></h2>
<pre tabindex="0" class="codeBlockStandalone_LlrK thin-scrollbar codeBlockContainer_hFbQ theme-code-block" style="--prism-color:#bfc7d5;--prism-background-color:#292d3e"><code class="codeBlockLines_JqGU"><span class="line"><span style="color:#FF79C6">#include</span><span style="color:#E9F284"> <</span><span style="color:#F1FA8C">stdarg.h</span><span style="color:#E9F284">></span></span>
<span class="line"><span style="color:#FF79C6">#include</span><span style="color:#E9F284"> <</span><span style="color:#F1FA8C">stdbool.h</span><span style="color:#E9F284">></span></span>
<span class="line"><span style="color:#FF79C6">#include</span><span style="color:#E9F284"> <</span><span style="color:#F1FA8C">stddef.h</span><span style="color:#E9F284">></span></span>
<span class="line"><span style="color:#FF79C6">#include</span><span style="color:#E9F284"> <</span><span style="color:#F1FA8C">stdint.h</span><span style="color:#E9F284">></span></span>
<span class="line"><span style="color:#FF79C6">#include</span><span style="color:#E9F284"> <</span><span style="color:#F1FA8C">stdlib.h</span><span style="color:#E9F284">></span></span>
<span class="line"></span>
<span class="line"><span style="color:#FF79C6">typedef</span><span style="color:#FF79C6"> uint32_t</span><span style="color:#F8F8F2"> Buffer;</span></span>
<span class="line"><span style="color:#FF79C6">typedef</span><span style="color:#FF79C6"> uint32_t</span><span style="color:#F8F8F2"> BufferIter;</span></span>
<span class="line"></span>
<span class="line"><span style="color:#FF79C6">void</span><span style="color:#50FA7B"> _console_log</span><span style="color:#F8F8F2">(</span></span>
<span class="line"><span style="color:#FF79C6">    uint8_t</span><span style="color:#FFB86C;font-style:italic"> level</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    const</span><span style="color:#FF79C6"> uint8_t</span><span style="color:#FF79C6"> *</span><span style="color:#FFB86C;font-style:italic">target</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    size_t</span><span style="color:#FFB86C;font-style:italic"> target_len</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    const</span><span style="color:#FF79C6"> uint8_t</span><span style="color:#FF79C6"> *</span><span style="color:#FFB86C;font-style:italic">filename</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    size_t</span><span style="color:#FFB86C;font-style:italic"> filename_len</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    uint32_t</span><span style="color:#FFB86C;font-style:italic"> line_number</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    const</span><span style="color:#FF79C6"> uint8_t</span><span style="color:#FF79C6"> *</span><span style="color:#FFB86C;font-style:italic">text</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    size_t</span><span style="color:#F8F8F2"> text_len</span></span>
<span class="line"><span style="color:#F8F8F2">);</span></span>
<span class="line"></span>
<span class="line"></span>
<span class="line"><span style="color:#F8F8F2">Buffer </span><span style="color:#50FA7B">_buffer_alloc</span><span style="color:#F8F8F2">(</span></span>
<span class="line"><span style="color:#FF79C6">    const</span><span style="color:#FF79C6"> uint8_t</span><span style="color:#FF79C6"> *</span><span style="color:#FFB86C;font-style:italic">data</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    size_t</span><span style="color:#F8F8F2"> data_len</span></span>
<span class="line"><span style="color:#F8F8F2">);</span></span>
<span class="line"><span style="color:#FF79C6">void</span><span style="color:#50FA7B"> _buffer_consume</span><span style="color:#F8F8F2">(</span></span>
<span class="line"><span style="color:#F8F8F2">    Buffer </span><span style="color:#FFB86C;font-style:italic">bufh</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    uint8_t</span><span style="color:#FF79C6"> *</span><span style="color:#FFB86C;font-style:italic">into</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    size_t</span><span style="color:#F8F8F2"> len</span></span>
<span class="line"><span style="color:#F8F8F2">);</span></span>
<span class="line"><span style="color:#FF79C6">size_t</span><span style="color:#50FA7B"> _buffer_len</span><span style="color:#F8F8F2">(Buffer </span><span style="color:#FFB86C;font-style:italic">bufh</span><span style="color:#F8F8F2">);</span></span>
<span class="line"></span>
<span class="line"></span>
<span class="line"><span style="color:#FF79C6">void</span><span style="color:#50FA7B"> _schedule_reducer</span><span style="color:#F8F8F2">(</span></span>
<span class="line"><span style="color:#FF79C6">    const</span><span style="color:#FF79C6"> uint8_t</span><span style="color:#FF79C6"> *</span><span style="color:#FFB86C;font-style:italic">name</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    size_t</span><span style="color:#FFB86C;font-style:italic"> name_len</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    const</span><span style="color:#FF79C6"> uint8_t</span><span style="color:#FF79C6"> *</span><span style="color:#FFB86C;font-style:italic">args</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    size_t</span><span style="color:#FFB86C;font-style:italic"> args_len</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    uint64_t</span><span style="color:#FFB86C;font-style:italic"> time</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    uint64_t</span><span style="color:#FF79C6"> *</span><span style="color:#F8F8F2">out</span></span>
<span class="line"><span style="color:#F8F8F2">);</span></span>
<span class="line"><span style="color:#FF79C6">void</span><span style="color:#50FA7B"> _cancel_reducer</span><span style="color:#F8F8F2">(</span><span style="color:#FF79C6">uint64_t</span><span style="color:#FFB86C;font-style:italic"> id</span><span style="color:#F8F8F2">);</span></span>
<span class="line"></span>
<span class="line"></span>
<span class="line"><span style="color:#FF79C6">uint16_t</span><span style="color:#50FA7B"> _create_index</span><span style="color:#F8F8F2">(</span></span>
<span class="line"><span style="color:#FF79C6">    const</span><span style="color:#FF79C6"> uint8_t</span><span style="color:#FF79C6"> *</span><span style="color:#FFB86C;font-style:italic">index_name</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    size_t</span><span style="color:#FFB86C;font-style:italic"> index_name_len</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    uint32_t</span><span style="color:#FFB86C;font-style:italic"> table_id</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    uint8_t</span><span style="color:#FFB86C;font-style:italic"> index_type</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    const</span><span style="color:#FF79C6"> uint8_t</span><span style="color:#FF79C6"> *</span><span style="color:#FFB86C;font-style:italic">col_ids</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    size_t</span><span style="color:#F8F8F2"> col_len</span></span>
<span class="line"><span style="color:#F8F8F2">);</span></span>
<span class="line"></span>
<span class="line"></span>
<span class="line"><span style="color:#FF79C6">uint16_t</span><span style="color:#50FA7B"> _insert</span><span style="color:#F8F8F2">(</span></span>
<span class="line"><span style="color:#FF79C6">    uint32_t</span><span style="color:#FFB86C;font-style:italic"> table_id</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    uint8_t</span><span style="color:#FF79C6"> *</span><span style="color:#FFB86C;font-style:italic">row</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    size_t</span><span style="color:#F8F8F2"> row_len</span></span>
<span class="line"><span style="color:#F8F8F2">);</span></span>
<span class="line"><span style="color:#FF79C6">uint16_t</span><span style="color:#50FA7B"> _delete_by_col_eq</span><span style="color:#F8F8F2">(</span></span>
<span class="line"><span style="color:#FF79C6">    uint32_t</span><span style="color:#FFB86C;font-style:italic"> table_id</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    uint32_t</span><span style="color:#FFB86C;font-style:italic"> col_id</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    const</span><span style="color:#FF79C6"> uint8_t</span><span style="color:#FF79C6"> *</span><span style="color:#FFB86C;font-style:italic">value</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    size_t</span><span style="color:#FFB86C;font-style:italic"> value_len</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    uint32_t</span><span style="color:#FF79C6"> *</span><span style="color:#F8F8F2">out</span></span>
<span class="line"><span style="color:#F8F8F2">);</span></span>
<span class="line"></span>
<span class="line"></span>
<span class="line"><span style="color:#FF79C6">uint16_t</span><span style="color:#50FA7B"> _get_table_id</span><span style="color:#F8F8F2">(</span></span>
<span class="line"><span style="color:#FF79C6">    const</span><span style="color:#FF79C6"> uint8_t</span><span style="color:#FF79C6"> *</span><span style="color:#FFB86C;font-style:italic">name</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    size_t</span><span style="color:#FFB86C;font-style:italic"> name_len</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    uint32_t</span><span style="color:#FF79C6"> *</span><span style="color:#F8F8F2">out</span></span>
<span class="line"><span style="color:#F8F8F2">);</span></span>
<span class="line"><span style="color:#FF79C6">uint16_t</span><span style="color:#50FA7B"> _iter_by_col_eq</span><span style="color:#F8F8F2">(</span></span>
<span class="line"><span style="color:#FF79C6">    uint32_t</span><span style="color:#FFB86C;font-style:italic"> table_id</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    uint32_t</span><span style="color:#FFB86C;font-style:italic"> col_id</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    const</span><span style="color:#FF79C6"> uint8_t</span><span style="color:#FF79C6"> *</span><span style="color:#FFB86C;font-style:italic">value</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    size_t</span><span style="color:#FFB86C;font-style:italic"> value_len</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#F8F8F2">    Buffer </span><span style="color:#FF79C6">*</span><span style="color:#F8F8F2">out</span></span>
<span class="line"><span style="color:#F8F8F2">);</span></span>
<span class="line"><span style="color:#FF79C6">uint16_t</span><span style="color:#50FA7B"> _iter_drop</span><span style="color:#F8F8F2">(BufferIter </span><span style="color:#FFB86C;font-style:italic">iter</span><span style="color:#F8F8F2">);</span></span>
<span class="line"><span style="color:#FF79C6">uint16_t</span><span style="color:#50FA7B"> _iter_next</span><span style="color:#F8F8F2">(BufferIter </span><span style="color:#FFB86C;font-style:italic">iter</span><span style="color:#F8F8F2">, Buffer </span><span style="color:#FF79C6">*</span><span style="color:#FFB86C;font-style:italic">out</span><span style="color:#F8F8F2">);</span></span>
<span class="line"><span style="color:#FF79C6">uint16_t</span><span style="color:#50FA7B"> _iter_start</span><span style="color:#F8F8F2">(</span><span style="color:#FF79C6">uint32_t</span><span style="color:#FFB86C;font-style:italic"> table_id</span><span style="color:#F8F8F2">, BufferIter </span><span style="color:#FF79C6">*</span><span style="color:#FFB86C;font-style:italic">out</span><span style="color:#F8F8F2">);</span></span>
<span class="line"><span style="color:#FF79C6">uint16_t</span><span style="color:#50FA7B"> _iter_start_filtered</span><span style="color:#F8F8F2">(</span></span>
<span class="line"><span style="color:#FF79C6">    uint32_t</span><span style="color:#FFB86C;font-style:italic"> table_id</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    const</span><span style="color:#FF79C6"> uint8_t</span><span style="color:#FF79C6"> *</span><span style="color:#FFB86C;font-style:italic">filter</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#FF79C6">    size_t</span><span style="color:#FFB86C;font-style:italic"> filter_len</span><span style="color:#F8F8F2">,</span></span>
<span class="line"><span style="color:#F8F8F2">    BufferIter </span><span style="color:#FF79C6">*</span><span style="color:#F8F8F2">out</span></span>
<span class="line"><span style="color:#F8F8F2">);</span></span></code></pre></div><footer class="theme-doc-footer docusaurus-mt-lg"><div class="row margin-top--sm theme-doc-footer-edit-meta-row"><div class="col noPrint_OLXC"><a href="https://github.com/clockworklabs/SpacetimeDB/edit/master/docs/docs/00300-resources/00300-internals/00100-module-abi-reference.md" target="_blank" rel="noopener noreferrer" class="theme-edit-this-page"><svg fill="currentColor" height="20" width="20" viewBox="0 0 40 40" class="iconEdit_kIlO" aria-hidden="true">\<g\><path d="m34.5 11.7l-3 3.1-6.3-6.3 3.1-3q0.5-0.5 1.2-0.5t1.1 0.5l3.9 3.9q0.5 0.4 0.5 1.1t-0.5 1.2z m-29.5 17.1l18.4-18.5 6.3 6.3-18.4 18.4h-6.3v-6.2z"></path></g></svg>Edit this page</a></div><div class="col lastUpdated_XxEB"></div></div></footer></article><nav class="docusaurus-mt-lg pagination-nav" aria-label="Docs pages"><a class="pagination-nav__link pagination-nav__link--prev" href="/docs/http/database"><div class="pagination-nav__sublabel">Previous</div><div class="pagination-nav__label">/v1/database</div></a><a class="pagination-nav__link pagination-nav__link--next" href="/docs/sats-json"><div class="pagination-nav__sublabel">Next</div><div class="pagination-nav__label">SATS-JSON Data Format</div></a></nav></div></div><div class="col col--3"><div class="tableOfContents_jOhJ thin-scrollbar theme-doc-toc-desktop"><ul class="table-of-contents table-of-contents__left-border">\<li\><a href="#general-notes" class="table-of-contents__link toc-highlight">General notes</a></li>\<li\><a href="#logging" class="table-of-contents__link toc-highlight">Logging</a></li>\<li\><a href="#buffer-handling" class="table-of-contents__link toc-highlight">Buffer handling</a></li>\<li\><a href="#reducer-scheduling" class="table-of-contents__link toc-highlight">Reducer scheduling</a></li>\<li\><a href="#altering-tables" class="table-of-contents__link toc-highlight">Altering tables</a></li>\<li\><a href="#inserting-and-deleting-rows" class="table-of-contents__link toc-highlight">Inserting and deleting rows</a></li>\<li\><a href="#querying-tables" class="table-of-contents__link toc-highlight">Querying tables</a></li>\<li\><a href="#appendix-bindingsh" class="table-of-contents__link toc-highlight">Appendix, <code>bindings.h</code></a></li></ul></div></div></div></div></main></div></div></div><div id="inkeep-shadowradix-:Rp:" style="display:contents"></div><span></span></div>
</body>
</html>