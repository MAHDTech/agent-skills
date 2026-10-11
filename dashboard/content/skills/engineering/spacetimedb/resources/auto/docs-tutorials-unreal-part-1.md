+++
title = "docs-tutorials-unreal-part-1"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "spacetimedb"
+++

<!doctype html>
<html lang="en" dir="ltr" class="docs-wrapper plugin-docs plugin-id-default docs-version-current docs-doc-page docs-doc-id-intro/tutorials/unreal-tutorial/part-1" data-has-hydrated="false">
\<head\>
<meta charset="UTF-8">
<meta name="generator" content="Docusaurus v3.9.2">
<title data-rh="true">1 - Setup | SpacetimeDB docs</title><meta data-rh="true" name="viewport" content="width=device-width,initial-scale=1"><meta data-rh="true" name="twitter:card" content="summary_large_image"><meta data-rh="true" property="og:url" content="https://spacetimedb.com/docs/tutorials/unreal/part-1"><meta data-rh="true" property="og:locale" content="en"><meta data-rh="true" name="docusaurus_locale" content="en"><meta data-rh="true" name="docsearch:language" content="en"><meta data-rh="true" name="docusaurus_version" content="current"><meta data-rh="true" name="docusaurus_tag" content="docs-default-current"><meta data-rh="true" name="docsearch:version" content="current"><meta data-rh="true" name="docsearch:docusaurus_tag" content="docs-default-current"><meta data-rh="true" property="og:title" content="1 - Setup | SpacetimeDB docs"><meta data-rh="true" name="description" content="Need help with the tutorial? Join our Discord server!"><meta data-rh="true" property="og:description" content="Need help with the tutorial? Join our Discord server!"><link data-rh="true" rel="icon" href="https://spacetimedb.com/favicon-32x32.png"><link data-rh="true" rel="canonical" href="https://spacetimedb.com/docs/tutorials/unreal/part-1"><link data-rh="true" rel="alternate" href="https://spacetimedb.com/docs/tutorials/unreal/part-1" hreflang="en"><link data-rh="true" rel="alternate" href="https://spacetimedb.com/docs/tutorials/unreal/part-1" hreflang="x-default"><script data-rh="true" type="application/ld+json">{"@context":"https://schema.org","@type":"BreadcrumbList","itemListElement":[{"@type":"ListItem","position":1,"name":"Tutorials","item":"https://spacetimedb.com/docs/tutorials/chat-app"},{"@type":"ListItem","position":2,"name":"Unreal Tutorial","item":"https://spacetimedb.com/docs/tutorials/unreal"},{"@type":"ListItem","position":3,"name":"1 - Setup","item":"https://spacetimedb.com/docs/tutorials/unreal/part-1"}]}</script><link rel="alternate" type="text/markdown" title="SpacetimeDB docs for agents" href="/docs/llms.txt">
<link rel="alternate" type="text/markdown" title="Full SpacetimeDB docs for agents" href="/docs/llms-full.txt">
<link rel="sitemap" type="application/xml" href="/docs/sitemap.xml">
<link rel="preload" as="font" type="font/woff2" href="/docs/fonts/inter-latin-wght-normal.woff2" crossorigin="anonymous">
<link rel="preload" as="font" type="font/woff2" href="/docs/fonts/source-code-pro-latin-wght-normal.woff2" crossorigin="anonymous"><link rel="stylesheet" href="/docs/assets/css/styles.4f000462.css">
<script src="/docs/assets/js/runtime~main.9d2ae6d0.js" defer="defer"></script>
<script src="/docs/assets/js/main.1a39f187.js" defer="defer"></script>
</head>
<body class="navigation-with-keyboard">
<svg style="display: none;">\<defs\>
<symbol id="theme-svg-external-link" viewBox="0 0 24 24"><path fill="currentColor" d="M21 13v10h-21v-19h12v2h-10v15h17v-8h2zm3-12h-10.988l4.035 4-6.977 7.07 2.828 2.828 6.977-7.07 4.125 4.172v-11z"/></symbol>
</defs></svg>
\<script\>document.documentElement.setAttribute("data-theme","light"),document.documentElement.setAttribute("data-theme-choice","light"),function(){try{const c=new URLSearchParams(window.location.search).entries();for(var[t,e]of c)if(t.startsWith("docusaurus-data-")){var a=t.replace("docusaurus-data-","data-");document.documentElement.setAttribute(a,e)}}catch(t){}}()</script><div id="__docusaurus"><div role="region" aria-label="Skip to main content"><a class="skipToContent_M6kL" href="#__docusaurus_skipToContent_fallback">Skip to main content</a></div><nav aria-label="Main" class="theme-layout-navbar navbar navbar--fixed-top"><div class="navbar__inner"><div class="theme-layout-navbar-left navbar__items"><button aria-label="Toggle navigation bar" aria-expanded="false" class="navbar__toggle clean-btn" type="button"><svg width="30" height="30" viewBox="0 0 30 30" aria-hidden="true"><path stroke="currentColor" stroke-linecap="round" stroke-miterlimit="10" stroke-width="2" d="M4 7h22M4 15h22M4 23h22"></path></svg></button><a href="https://spacetimedb.com" target="_self" rel="noopener noreferrer" class="navbar__brand"><div class="navbar__logo"><img src="https://spacetimedb.com/images/brand.svg" alt="Spacetime Logo" class="themedComponent_J5mh themedComponent--light_Du4S" height="32" width="133"><img src="https://spacetimedb.com/images/brand.svg" alt="Spacetime Logo" class="themedComponent_J5mh themedComponent--dark_W6YC" height="32" width="133"></div></a><div class="navbarSearchContainer_NRqO"><div id="inkeep-shadowradix-:Rnb5a5:" style="display:contents"></div><span></span></div><div class="navbar__item dropdown dropdown--hoverable"><a aria-current="page" class="navbar__link active" aria-haspopup="true" aria-expanded="false" role="button" href="/docs/tutorials/unreal/part-1">2.0.0</a><ul class="dropdown__menu">\<li\><a aria-current="page" class="dropdown__link dropdown__link--active" href="/docs/tutorials/unreal/part-1">2.0.0</a></li>\<li\><a class="dropdown__link" href="/docs/1.12.0/tutorials/unreal/part-1">1.12.0</a></li></ul></div></div><div class="theme-layout-navbar-right navbar__items navbar__items--right"><a href="https://spacetimedb.com/install" target="_blank" rel="noopener noreferrer" class="navbar__item navbar__link">Install</a><a href="https://spacetimedb.com/pricing" target="_blank" rel="noopener noreferrer" class="navbar__item navbar__link">Pricing</a><a href="https://spacetimedb.com/blog" target="_blank" rel="noopener noreferrer" class="navbar__item navbar__link">Blog</a><a href="https://spacetimedb.com/community" target="_blank" rel="noopener noreferrer" class="navbar__item navbar__link">Community</a><a href="https://spacetimedb.com/space-race" target="_blank" rel="noopener noreferrer" class="navbar__item navbar__link">Referrals</a><a href="https://spacetimedb.com/login" target="_blank" rel="noopener noreferrer" class="navbar__item navbar__link navbar__button">Login</a></div></div><div role="presentation" class="navbar-sidebar__backdrop"></div></nav><div id="__docusaurus_skipToContent_fallback" class="theme-layout-main main-wrapper mainWrapper_zieo"><div class="docsWrapper_LRYd"><button aria-label="Scroll back to top" class="clean-btn theme-back-to-top-button backToTopButton_uRlU" type="button"></button><div class="docRoot_xHAI"><aside class="theme-doc-sidebar-container docSidebarContainer_izyX"><div class="sidebarViewport_fAk8"><div class="sidebar_mXBm"><nav aria-label="Docs sidebar" class="menu thin-scrollbar menu_xs3F"><ul class="theme-doc-sidebar-menu menu__list"><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item sidebar-ask-ai"><a class="menu__link" href="/docs/ask-ai/ask-ai"><span title="Ask AI Chat" class="linkLabel_UvDg">Ask AI Chat</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1"><div class="spacetime-menu-header">Intro</div></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/"><span title="Getting Started" class="linkLabel_UvDg">Getting Started</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/intro/what-is-spacetimedb"><span title="What is SpacetimeDB?" class="linkLabel_UvDg">What is SpacetimeDB?</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/intro/zen"><span title="The Zen of Spacetime" class="linkLabel_UvDg">The Zen of Spacetime</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/intro/language-support"><span title="Language Support" class="linkLabel_UvDg">Language Support</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/intro/key-architecture"><span title="Key Architecture" class="linkLabel_UvDg">Key Architecture</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/intro/faq"><span title="FAQ" class="linkLabel_UvDg">FAQ</span></a></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist" href="/docs/quickstarts/react"><span title="Quickstarts" class="categoryLinkLabel_HATI">Quickstarts</span></a><button aria-label="Expand sidebar category &#x27;Quickstarts&#x27;" aria-expanded="false" type="button" class="clean-btn menu__caret"></button></div></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist menu__link--active" href="/docs/tutorials/chat-app"><span title="Tutorials" class="categoryLinkLabel_HATI">Tutorials</span></a><button aria-label="Collapse sidebar category &#x27;Tutorials&#x27;" aria-expanded="true" type="button" class="clean-btn menu__caret"></button></div><ul class="menu__list"><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-2 menu__list-item"><a class="menu__link" tabindex="0" href="/docs/tutorials/chat-app"><span title="Chat App Tutorial" class="linkLabel_UvDg">Chat App Tutorial</span></a></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-2 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist" tabindex="0" href="/docs/tutorials/unity"><span title="Unity Tutorial" class="categoryLinkLabel_HATI">Unity Tutorial</span></a><button aria-label="Expand sidebar category &#x27;Unity Tutorial&#x27;" aria-expanded="false" type="button" class="clean-btn menu__caret"></button></div></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-2 menu__list-item"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist menu__link--active" tabindex="0" href="/docs/tutorials/unreal"><span title="Unreal Tutorial" class="categoryLinkLabel_HATI">Unreal Tutorial</span></a><button aria-label="Collapse sidebar category &#x27;Unreal Tutorial&#x27;" aria-expanded="true" type="button" class="clean-btn menu__caret"></button></div><ul class="menu__list"><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-3 menu__list-item"><a class="menu__link menu__link--active" aria-current="page" tabindex="0" href="/docs/tutorials/unreal/part-1"><span title="1 - Setup" class="linkLabel_UvDg">1 - Setup</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-3 menu__list-item"><a class="menu__link" tabindex="0" href="/docs/tutorials/unreal/part-2"><span title="2 - Connecting to SpacetimeDB" class="linkLabel_UvDg">2 - Connecting to SpacetimeDB</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-3 menu__list-item"><a class="menu__link" tabindex="0" href="/docs/tutorials/unreal/part-3"><span title="3 - Gameplay" class="linkLabel_UvDg">3 - Gameplay</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-3 menu__list-item"><a class="menu__link" tabindex="0" href="/docs/tutorials/unreal/part-4"><span title="4 - Moving and Colliding" class="linkLabel_UvDg">4 - Moving and Colliding</span></a></li></ul></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-2 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist" tabindex="0" href="/docs/tutorials/godot"><span title="Godot Tutorial" class="categoryLinkLabel_HATI">Godot Tutorial</span></a><button aria-label="Expand sidebar category &#x27;Godot Tutorial&#x27;" aria-expanded="false" type="button" class="clean-btn menu__caret"></button></div></li></ul></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item spacetime-menu-header-link"><a class="menu__link" href="/docs/core-concepts"><span title="Core Concepts" class="linkLabel_UvDg">Core Concepts</span></a></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist" href="/docs/databases"><span title="Databases" class="categoryLinkLabel_HATI">Databases</span></a><button aria-label="Expand sidebar category &#x27;Databases&#x27;" aria-expanded="false" type="button" class="clean-btn menu__caret"></button></div></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist" href="/docs/functions"><span title="Functions" class="categoryLinkLabel_HATI">Functions</span></a><button aria-label="Expand sidebar category &#x27;Functions&#x27;" aria-expanded="false" type="button" class="clean-btn menu__caret"></button></div></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist" href="/docs/tables"><span title="Tables" class="categoryLinkLabel_HATI">Tables</span></a><button aria-label="Expand sidebar category &#x27;Tables&#x27;" aria-expanded="false" type="button" class="clean-btn menu__caret"></button></div></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist" href="/docs/core-concepts/authentication"><span title="Authentication" class="categoryLinkLabel_HATI">Authentication</span></a><button aria-label="Expand sidebar category &#x27;Authentication&#x27;" aria-expanded="false" type="button" class="clean-btn menu__caret"></button></div></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist" href="/docs/clients"><span title="Clients" class="categoryLinkLabel_HATI">Clients</span></a><button aria-label="Expand sidebar category &#x27;Clients&#x27;" aria-expanded="false" type="button" class="clean-btn menu__caret"></button></div></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item spacetime-menu-header-link"><a class="menu__link" href="/docs/resources"><span title="Developer Resources" class="linkLabel_UvDg">Developer Resources</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/troubleshooting"><span title="Basic troubleshooting" class="linkLabel_UvDg">Basic troubleshooting</span></a></li><li class="theme-doc-sidebar-item-link theme-doc-sidebar-item-link-level-1 menu__list-item"><a class="menu__link" href="/docs/upgrade"><span title="Migrating from 1.0 to 2.0" class="linkLabel_UvDg">Migrating from 1.0 to 2.0</span></a></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist menu__link--sublist-caret" role="button" aria-expanded="false" href="/docs/troubleshooting"><span title="How To" class="categoryLinkLabel_HATI">How To</span></a></div></li><li class="theme-doc-sidebar-item-category theme-doc-sidebar-item-category-level-1 menu__list-item menu__list-item--collapsed"><div class="menu__list-item-collapsible"><a class="categoryLink_iWzw menu__link menu__link--sublist menu__link--sublist-caret" role="button" aria-expanded="false" href="/docs/cli-reference"><span title="References" class="categoryLinkLabel_HATI">References</span></a></div></li></ul></nav></div></div></aside><main class="docMainContainer_dsjb"><div class="container padding-top--md padding-bottom--lg"><div class="row"><div class="col docItemCol_TFty"><div class="docItemContainer_LO0A">\<article\><nav class="theme-doc-breadcrumbs breadcrumbsContainer_EWVV" aria-label="Breadcrumbs"><ul class="breadcrumbs"><li class="breadcrumbs__item"><a aria-label="Home page" class="breadcrumbs__link" href="/docs/"><svg viewBox="0 0 24 24" class="breadcrumbHomeIcon_i5pl"><path d="M10 19v-5h4v5c0 .55.45 1 1 1h3c.55 0 1-.45 1-1v-7h1.7c.46 0 .68-.57.33-.87L12.67 3.6c-.38-.34-.96-.34-1.34 0l-8.36 7.53c-.34.3-.13.87.33.87H5v7c0 .55.45 1 1 1h3c.55 0 1-.45 1-1z" fill="currentColor"></path></svg></a></li><li class="breadcrumbs__item"><a class="breadcrumbs__link" href="/docs/tutorials/chat-app"><span>Tutorials</span></a></li><li class="breadcrumbs__item"><a class="breadcrumbs__link" href="/docs/tutorials/unreal"><span>Unreal Tutorial</span></a></li><li class="breadcrumbs__item breadcrumbs__item--active"><span class="breadcrumbs__link">1 - Setup</span></li></ul></nav><span class="theme-doc-version-badge badge badge--secondary">Version: 2.0.0</span><div class="tocCollapsible_ldQs theme-doc-toc-mobile tocMobile_xBih"><button type="button" class="clean-btn tocCollapsibleButton_Dk1U">On this page</button></div><div class="theme-doc-markdown markdown">\<header\>\<h1\>1 - Setup</h1></header><p>Need help with the tutorial? <a href="https://discord.gg/spacetimedb" target="_blank" rel="noopener noreferrer" class="">Join our Discord server</a>!</p>
<blockquote>
<p>A completed version of the game we&#x27;ll create in this tutorial is available at:</p>
<p><a href="https://github.com/clockworklabs/SpacetimeDB/tree/master/demo/Blackholio" target="_blank" rel="noopener noreferrer" class="">https://github.com/clockworklabs/SpacetimeDB/tree/master/demo/Blackholio</a></p>
</blockquote>
<h2 class="anchor anchorTargetStickyNavbar_Y_NL" id="setting-up-the-tutorial-unreal-project">Setting up the Tutorial Unreal Project<a href="#setting-up-the-tutorial-unreal-project" class="hash-link" aria-label="Direct link to Setting up the Tutorial Unreal Project" title="Direct link to Setting up the Tutorial Unreal Project" translate="no">​</a></h2>
<p>In this section, we will guide you through the process of setting up a Unreal Project that will serve as the starting point for our tutorial. By the end of this section, you will have a basic Unreal project and be ready to implement the server functionality.</p>
<h3 class="anchor anchorTargetStickyNavbar_Y_NL" id="step-1-create-a-blank-unreal-project">Step 1: Create a Blank Unreal Project<a href="#step-1-create-a-blank-unreal-project" class="hash-link" aria-label="Direct link to Step 1: Create a Blank Unreal Project" title="Direct link to Step 1: Create a Blank Unreal Project" translate="no">​</a></h3>
<p>SpacetimeDB supports Unreal version <code>5.6</code>. See <a class="" href="/docs/tutorials/unreal/">the overview</a> for more information on specific supported versions.</p>
<p>Launch Unreal 5.6 and create a new project by selecting Games from the Unreal Project Browser.</p>
<div class="theme-admonition theme-admonition-warning admonition_Ph9R alert alert--warning"><div class="admonitionHeading_kG9t"><span class="admonitionIcon_VA_f"><svg viewBox="0 0 16 16"><path fill-rule="evenodd" d="M8.893 1.5c-.183-.31-.52-.5-.887-.5s-.703.19-.886.5L.138 13.499a.98.98 0 0 0 0 1.001c.193.31.53.501.886.501h13.964c.367 0 .704-.19.877-.5a1.03 1.03 0 0 0 .01-1.002L8.893 1.5zm.133 11.497H6.987v-2.003h2.039v2.003zm0-3.004H6.987V5.987h2.039v4.006z"></path></svg></span>warning</div><div class="admonitionContent_STxx"><p>Select the <strong>Blank</strong> template and in <strong>Project Defaults</strong> select <strong>C++</strong>.</p></div></div>
<p>For <strong>Project Name</strong> use <code>blackholio</code>.</p>
<p>Click <strong>Create</strong> to generate the blank project.</p>
<p><img decoding="async" loading="lazy" alt="Create Blank Project" src="/docs/assets/images/part-1-01-create-project-378b79c8a34e3304733a550fc939c6f3.png" width="1187" height="854" class="img_sDl7"></p>
<h3 class="anchor anchorTargetStickyNavbar_Y_NL" id="import-the-spacetimedb-unreal-sdk">Import the SpacetimeDB Unreal SDK<a href="#import-the-spacetimedb-unreal-sdk" class="hash-link" aria-label="Direct link to Import the SpacetimeDB Unreal SDK" title="Direct link to Import the SpacetimeDB Unreal SDK" translate="no">​</a></h3>
<p>While the SpacetimeDB Unreal client SDK is in preview releases, it can only be installed from GitHub:</p>
<blockquote>
<p><a href="https://github.com/clockworklabs/SpacetimeDB/tree/master/sdks/unreal/src" target="_blank" rel="noopener noreferrer" class="">https://github.com/clockworklabs/SpacetimeDB/tree/master/sdks/unreal/src</a></p>
</blockquote>
<p>Once the SDK is stabilized, we&#x27;ll find a more ergonomic way to distribute it.</p>
<div class="theme-admonition theme-admonition-note admonition_Ph9R alert alert--secondary"><div class="admonitionHeading_kG9t"><span class="admonitionIcon_VA_f"><svg viewBox="0 0 14 16"><path fill-rule="evenodd" d="M6.3 5.69a.942.942 0 0 1-.28-.7c0-.28.09-.52.28-.7.19-.18.42-.28.7-.28.28 0 .52.09.7.28.18.19.28.42.28.7 0 .28-.09.52-.28.7a1 1 0 0 1-.7.3c-.28 0-.52-.11-.7-.3zM8 7.99c-.02-.25-.11-.48-.31-.69-.2-.19-.42-.3-.69-.31H6c-.27.02-.48.13-.69.31-.2.2-.3.44-.31.69h1v3c.02.27.11.5.31.69.2.2.42.31.69.31h1c.27 0 .48-.11.69-.31.2-.19.3-.42.31-.69H8V7.98v.01zM7 2.3c-3.14 0-5.7 2.54-5.7 5.68 0 3.14 2.56 5.7 5.7 5.7s5.7-2.55 5.7-5.7c0-3.15-2.56-5.69-5.7-5.69v.01zM7 .98c3.86 0 7 3.14 7 7s-3.14 7-7 7-7-3.12-7-7 3.14-7 7-7z"></path></svg></span>note</div><div class="admonitionContent_STxx"><p>Before beginning make sure to close the Unreal project and IDE.</p></div></div>
<h4 class="anchor anchorTargetStickyNavbar_Y_NL" id="installation-steps">Installation steps<a href="#installation-steps" class="hash-link" aria-label="Direct link to Installation steps" title="Direct link to Installation steps" translate="no">​</a></h4>
\<ol\>
<li class="">
<p>Navigate to your Unreal project directory and create a <code>Plugins</code> folder if it doesn’t already exist:</p>
<pre tabindex="0" class="codeBlockStandalone_LlrK thin-scrollbar codeBlockContainer_hFbQ theme-code-block" style="--prism-color:#bfc7d5;--prism-background-color:#292d3e"><code class="codeBlockLines_JqGU"><span class="line"><span style="color:#8BE9FD">cd</span><span style="color:#F1FA8C"> blackholio</span></span>
<span class="line"><span style="color:#50FA7B">mkdir</span><span style="color:#F1FA8C"> Plugins</span></span></code></pre>
</li>
<li class="">
<p>Download or clone the SDK from GitHub and copy the SpacetimeDbSdk folder into your new Plugins directory.</p>
\<ul\>
<li class="">This should create <code>/blackholio/Plugins/SpacetimeDbSdk</code>.</li>
</ul>
</li>
<li class="">
<p>In the root of the Unreal project, right click the blackholio.uproject and select <strong>Generate Visual Studio project files</strong>. On Windows 11 you may need to expand <strong>Show more options</strong> to select the generate option.</p>
</li>
</ol>
<p><img decoding="async" loading="lazy" alt="Generate project files" src="/docs/assets/images/part-1-02-01-generate-project-61146b2e33b2039df6d88670b6f594bc.png" width="613" height="96" class="img_sDl7">
<img decoding="async" loading="lazy" alt="Generate project files" src="data:image/png;base64,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" width="482" height="92" class="img_sDl7"></p>
<h3 class="anchor anchorTargetStickyNavbar_Y_NL" id="create-the-gamemanager-actor">Create the GameManager Actor<a href="#create-the-gamemanager-actor" class="hash-link" aria-label="Direct link to Create the GameManager Actor" title="Direct link to Create the GameManager Actor" translate="no">​</a></h3>
<div class="theme-tabs-container tabs-container tabList_duiv"><ul role="tablist" aria-orientation="horizontal" class="tabs"><li role="tab" tabindex="0" aria-selected="true" class="tabs__item tabItem_a1H9 tabs__item--active">C++</li><li role="tab" tabindex="-1" aria-selected="false" class="tabs__item tabItem_a1H9">Blueprint</li></ul><div class="margin-top--md"><div role="tabpanel" class="tabItem_w6aI">\<ol\>
<li class="">Open the <code>blackholio</code> project in your IDE (Visual Studio or JetBrains Rider) and run the project to launch the Unreal Editor.<!-- -->
\<ul\>
<li class="">This will enable <strong>Live Coding</strong>, making the workflow a bit smoother.</li>
<li class="">Unreal will prompt you to build the <code>SpacetimeDbSdk</code> plugin. Do so.</li>
</ul>
</li>
<li class="">Open <strong>Tools -> New C++ Class</strong> in the top menu, select <strong>Actor</strong> as the parent and click <strong>Next</strong></li>
<li class="">Select <strong>Public</strong> Class Type</li>
<li class="">Name the class <code>GameManager</code>.</li>
</ol><p>The <code>GameManager</code> class will be where we will put the high level initialization and coordination logic for our game.</p><blockquote>
<p><strong>Note:</strong> In a production Unreal project, you would typically implement this logic in a Subsystem. For simplicity, this tutorial uses a singleton actor.</p>
</blockquote></div><div role="tabpanel" class="tabItem_w6aI" hidden="">\<ol\>
<li class="">Open the <code>blackholio</code> project to launch the Unreal Editor.</li>
<li class=""><strong>Create a GameManager Blueprint</strong>
\<ul\>
<li class="">In the <strong>Content Drawer</strong>, click <strong>Add</strong>, then select <strong>Blueprint -> Blueprint Class</strong>.</li>
<li class="">Click <strong>Actor</strong>.</li>
<li class="">Name the blueprint <code>BP_GameManager</code>.</li>
</ul>
</li>
</ol></div></div></div>
<h3 class="anchor anchorTargetStickyNavbar_Y_NL" id="set-up-the-level">Set Up the Level<a href="#set-up-the-level" class="hash-link" aria-label="Direct link to Set Up the Level" title="Direct link to Set Up the Level" translate="no">​</a></h3>
<p>Set up the empty level, add the new <code>GameManager</code> to the level, and add lighting.</p>
<div class="theme-tabs-container tabs-container tabList_duiv"><ul role="tablist" aria-orientation="horizontal" class="tabs"><li role="tab" tabindex="0" aria-selected="true" class="tabs__item tabItem_a1H9 tabs__item--active">C++</li><li role="tab" tabindex="-1" aria-selected="false" class="tabs__item tabItem_a1H9">Blueprint</li></ul><div class="margin-top--md"><div role="tabpanel" class="tabItem_w6aI">\<ol\>
<li class="">
<p><strong>Create a new level</strong></p>
\<ul\>
<li class="">Open <strong>File -> New Level</strong> in the top menu, select <strong>Empty Level</strong>, and click <strong>Create</strong>.</li>
<li class="">Save the level and name it <code>Blackholio</code>.</li>
</ul>
</li>
<li class="">
<p><strong>Create a GameManager Blueprint</strong></p>
\<ul\>
<li class="">In the <strong>Content Drawer</strong>, click <strong>Add</strong>, then select <strong>Blueprint -> Blueprint Class</strong>.</li>
<li class="">Expand <strong>All Classes</strong>, search for <strong>GameManager</strong>, highlight it, and click <strong>Select</strong>.</li>
<li class="">Name the blueprint <code>BP_GameManager</code>.</li>
</ul>
<p><img decoding="async" loading="lazy" alt="Pick Parent Class" src="/docs/assets/images/part-1-03-create-blueprint-98f1a712896285dde7aad0eed871b272.png" width="620" height="534" class="img_sDl7"></p>
</li>
<li class="">
<p><strong>Update Maps & Modes</strong></p>
\<ul\>
<li class="">Open <strong>Edit -> Project Settings</strong> in the top menu, then select <strong>Project -> Maps & Modes</strong> on the left.</li>
<li class="">Set <strong>Editor Startup Map</strong> to <code>Blackholio</code>.</li>
<li class="">Set <strong>Game Default Map</strong> to <code>Blackholio</code>.</li>
</ul>
</li>
<li class="">
<p><strong>Add to the Level</strong></p>
\<ul\>
<li class="">Drag the <code>BP_GameManager</code> blueprint from the <strong>Content Drawer</strong> into the scene view.</li>
</ul>
</li>
<li class="">
<p><strong>Add a Directional Light</strong></p>
\<ul\>
<li class="">Click <strong>Add</strong> in the top toolbar, then select <strong>Lights -> Directional Light</strong>.</li>
<li class="">Set <strong>Rotation</strong> to -105.0, -31.0, -14.0.</li>
</ul>
</li>
<li class="">
<p><strong>Add a Post Process Volume</strong></p>
\<ul\>
<li class="">Click <strong>Add</strong> in the top toolbar, then select <strong>Volumes -> Post Process Volume</strong>.</li>
<li class="">Enable and set <strong>Exposure -> Exposure Compensation</strong> to 0.0.</li>
<li class="">Enable and set <strong>Exposure -> Min EV100</strong> to 1.0.</li>
<li class="">Enable and set <strong>Exposure -> Max EV100</strong> to 1.0.</li>
<li class="">Enable <strong>Post Process Volume Settings -> Infinite Extend (Unbounded)</strong>.</li>
</ul>
</li>
</ol></div><div role="tabpanel" class="tabItem_w6aI" hidden="">\<ol\>
<li class="">
<p><strong>Create a new level</strong></p>
\<ul\>
<li class="">Open <strong>File -> New Level</strong> in the top menu, select <strong>Empty Level</strong>, and click <strong>Create</strong>.</li>
<li class="">Save the level and name it <code>Blackholio</code>.</li>
</ul>
</li>
<li class="">
<p><strong>Update Maps & Modes</strong></p>
\<ul\>
<li class="">Open <strong>Edit -> Project Settings</strong> in the top menu, then select <strong>Project -> Maps & Modes</strong> on the left.</li>
<li class="">Set <strong>Editor Startup Map</strong> to <code>Blackholio</code>.</li>
<li class="">Set <strong>Game Default Map</strong> to <code>Blackholio</code>.</li>
</ul>
</li>
<li class="">
<p><strong>Add to the Level</strong></p>
\<ul\>
<li class="">Drag the <code>BP_GameManager</code> blueprint from the <strong>Content Drawer</strong> into the scene view.</li>
</ul>
</li>
<li class="">
<p><strong>Add a Directional Light</strong></p>
\<ul\>
<li class="">Click <strong>Add</strong> in the top toolbar, then select <strong>Lights -> Directional Light</strong>.</li>
<li class="">Set <strong>Rotation</strong> to -105.0, -31.0, -14.0.</li>
</ul>
</li>
<li class="">
<p><strong>Add a Post Process Volume</strong></p>
\<ul\>
<li class="">Click <strong>Add</strong> in the top toolbar, then select <strong>Volumes -> Post Process Volume</strong>.</li>
<li class="">Enable and set <strong>Exposure -> Exposure Compensation</strong> to 0.0.</li>
<li class="">Enable and set <strong>Exposure -> Min EV100</strong> to 1.0.</li>
<li class="">Enable and set <strong>Exposure -> Max EV100</strong> to 1.0.</li>
<li class="">Enable <strong>Post Process Volume Settings -> Infinite Extend (Unbounded)</strong>.</li>
</ul>
</li>
</ol></div></div></div>
<h3 class="anchor anchorTargetStickyNavbar_Y_NL" id="add-a-simple-gamemode">Add a Simple GameMode<a href="#add-a-simple-gamemode" class="hash-link" aria-label="Direct link to Add a Simple GameMode" title="Direct link to Add a Simple GameMode" translate="no">​</a></h3>
<p>Create a simple GameMode to tweak the startup settings and connect it to the World Settings.</p>
<div class="theme-tabs-container tabs-container tabList_duiv"><ul role="tablist" aria-orientation="horizontal" class="tabs"><li role="tab" tabindex="0" aria-selected="true" class="tabs__item tabItem_a1H9 tabs__item--active">C++</li><li role="tab" tabindex="-1" aria-selected="false" class="tabs__item tabItem_a1H9">Blueprint</li></ul><div class="margin-top--md"><div role="tabpanel" class="tabItem_w6aI">\<ol\>
<li class="">
<p><strong>Create the C++ class</strong></p>
\<ul\>
<li class="">Open <strong>Tools -> New C++ Class</strong> in the top menu, select <strong>GameModeBase</strong> as the parent, and click <strong>Next</strong>.</li>
<li class="">Select <strong>Public</strong> as the class type.</li>
<li class="">Name the class <code>BlackholioGameMode</code>.</li>
</ul>
</li>
<li class="">
<p><strong>Create a GameMode Blueprint</strong></p>
\<ul\>
<li class="">In the <strong>Content Drawer</strong>, click <strong>Add</strong>, then select <strong>Blueprint -> Blueprint Class</strong>.</li>
<li class="">Expand <strong>All Classes</strong>, search for <code>BlackholioGameMode</code>, highlight it, and click <strong>Select</strong>.</li>
<li class="">Name the blueprint <code>BP_BlackholioGameMode</code>.</li>
</ul>
</li>
<li class="">
<p><strong>Update World Settings</strong></p>
\<ul\>
<li class="">Open <strong>Window -> World Settings</strong> in the top menu.</li>
<li class="">Change <strong>GameMode Override</strong> from <strong>None</strong> to <code>BP_BlackholioGameMode</code>.</li>
<li class="">Save the level.</li>
</ul>
</li>
</ol></div><div role="tabpanel" class="tabItem_w6aI" hidden="">\<ol\>
<li class="">
<p><strong>Create a GameMode Blueprint</strong></p>
\<ul\>
<li class="">In the <strong>Content Drawer</strong>, click <strong>Add</strong>, then select <strong>Blueprint -> Blueprint Class</strong>.</li>
<li class="">Expand <strong>All Classes</strong>, and click <code>Game Mode Base</code>.</li>
<li class="">Name the blueprint <code>BP_GameMode</code>.</li>
</ul>
</li>
<li class="">
<p><strong>Update World Settings</strong></p>
\<ul\>
<li class="">Open <strong>Window -> World Settings</strong> in the top menu.</li>
<li class="">Change <strong>GameMode Override</strong> from <strong>None</strong> to <code>BP_GameMode</code>.</li>
<li class="">Save the level.</li>
</ul>
</li>
</ol></div></div></div>
<p>At this point, the foundation of the Unreal project is set up. Pressing Play will show a blank screen, but the game should start without errors. Next, we’ll create the SpacetimeDB server module so we have something to connect to.</p>
<h3 class="anchor anchorTargetStickyNavbar_Y_NL" id="create-the-server-module">Create the Server Module<a href="#create-the-server-module" class="hash-link" aria-label="Direct link to Create the Server Module" title="Direct link to Create the Server Module" translate="no">​</a></h3>
<p>We&#x27;ve now got the very basics set up. In <a class="" href="/docs/tutorials/unreal/part-2">part 2</a> you&#x27;ll learn the basics of how to create a SpacetimeDB server module and how to connect to it from your client.</p></div><footer class="theme-doc-footer docusaurus-mt-lg"><div class="row margin-top--sm theme-doc-footer-edit-meta-row"><div class="col noPrint_OLXC"><a href="https://github.com/clockworklabs/SpacetimeDB/edit/master/docs/docs/00100-intro/00300-tutorials/00400-unreal-tutorial/00200-part-1.md" target="_blank" rel="noopener noreferrer" class="theme-edit-this-page"><svg fill="currentColor" height="20" width="20" viewBox="0 0 40 40" class="iconEdit_kIlO" aria-hidden="true">\<g\><path d="m34.5 11.7l-3 3.1-6.3-6.3 3.1-3q0.5-0.5 1.2-0.5t1.1 0.5l3.9 3.9q0.5 0.4 0.5 1.1t-0.5 1.2z m-29.5 17.1l18.4-18.5 6.3 6.3-18.4 18.4h-6.3v-6.2z"></path></g></svg>Edit this page</a></div><div class="col lastUpdated_XxEB"></div></div></footer></article><nav class="docusaurus-mt-lg pagination-nav" aria-label="Docs pages"><a class="pagination-nav__link pagination-nav__link--prev" href="/docs/tutorials/unreal"><div class="pagination-nav__sublabel">Previous</div><div class="pagination-nav__label">Unreal Tutorial</div></a><a class="pagination-nav__link pagination-nav__link--next" href="/docs/tutorials/unreal/part-2"><div class="pagination-nav__sublabel">Next</div><div class="pagination-nav__label">2 - Connecting to SpacetimeDB</div></a></nav></div></div><div class="col col--3"><div class="tableOfContents_jOhJ thin-scrollbar theme-doc-toc-desktop"><ul class="table-of-contents table-of-contents__left-border">\<li\><a href="#setting-up-the-tutorial-unreal-project" class="table-of-contents__link toc-highlight">Setting up the Tutorial Unreal Project</a>\<ul\>\<li\><a href="#step-1-create-a-blank-unreal-project" class="table-of-contents__link toc-highlight">Step 1: Create a Blank Unreal Project</a></li>\<li\><a href="#import-the-spacetimedb-unreal-sdk" class="table-of-contents__link toc-highlight">Import the SpacetimeDB Unreal SDK</a></li>\<li\><a href="#create-the-gamemanager-actor" class="table-of-contents__link toc-highlight">Create the GameManager Actor</a></li>\<li\><a href="#set-up-the-level" class="table-of-contents__link toc-highlight">Set Up the Level</a></li>\<li\><a href="#add-a-simple-gamemode" class="table-of-contents__link toc-highlight">Add a Simple GameMode</a></li>\<li\><a href="#create-the-server-module" class="table-of-contents__link toc-highlight">Create the Server Module</a></li></ul></li></ul></div></div></div></div></main></div></div></div><div id="inkeep-shadowradix-:Rp:" style="display:contents"></div><span></span></div>
</body>
</html>
