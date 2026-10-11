+++
title = "docs-cli-features"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "antigravity"
+++

<!DOCTYPE html><html lang="en" dir="ltr" data-theme="dark" data-has-toc data-has-sidebar class="astro-vdxcvpsm">\<head\>\<script\>
  if (window.location.hostname === 'www.antigravity.google') {
    window.location.replace(
      `https://antigravity.google${window.location.pathname}${window.location.search}${window.location.hash}`
    );
  }
</script><meta charset="utf-8"/><meta name="viewport" content="width=device-width, initial-scale=1"/>\<title\>Features - Antigravity CLI | Google Antigravity Docs</title><link rel="canonical" href="https://antigravity.google/docs/cli/features/"/><link rel="sitemap" href="/sitemap-index.xml"/><link rel="icon" type="image/x-icon" href="/favicon.ico" sizes="48x48"/><link rel="icon" type="image/png" href="/apple-touch-icon.png" sizes="180x180"/><link rel="icon" type="image/png" href="/assets/image/antigravity-logo.png" sizes="276x276"/><link rel="apple-touch-icon" href="/apple-touch-icon.png" sizes="180x180"/><link rel="apple-touch-icon-precomposed" href="/apple-touch-icon-precomposed.png" sizes="180x180"/><link rel="preconnect" href="https://fonts.googleapis.com"/><link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/><link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Google+Symbols:opsz,wght,FILL,GRAD,ROND@20..48,100..700,0..1,-50..200,0..100&display=block"/><link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Google+Sans+Flex:opsz,slnt,wdth,wght,ROND@8..144,-10..0,25..150,400..500,0..100&display=swap"/><link rel="shortcut icon" href="/favicon.svg" type="image/svg+xml"/><meta name="generator" content="Astro v7.2.9"/><meta name="generator" content="Starlight v0.41.5"/><meta property="og:title" content="Features - Antigravity CLI | Google Antigravity Docs"/><meta property="og:type" content="article"/><meta property="og:url" content="https://antigravity.google/docs/cli/features/"/><meta property="og:locale" content="en"/><meta property="og:site_name" content="Google Antigravity Docs"/><meta property="og:image" content="https://antigravity.google/assets/image/sitecards/sitecard-documentation.png"/><meta property="og:image:secure_url" content="https://antigravity.google/assets/image/sitecards/sitecard-documentation.png"/><meta name="twitter:image" content="https://antigravity.google/assets/image/sitecards/sitecard-documentation.png"/><meta name="twitter:card" content="summary_large_image"/><meta name="description" content="How plugins work Plugins are namespaced bundles that can contain skills, agents, rules, MCP servers, and hooks as a single deployable unit."/><meta property="og:description" content="How plugins work Plugins are namespaced bundles that can contain skills, agents, rules, MCP servers, and hooks as a single deployable unit."/><meta name="twitter:title" content="Features - Antigravity CLI | Google Antigravity Docs"/><meta name="twitter:description" content="How plugins work Plugins are namespaced bundles that can contain skills, agents, rules, MCP servers, and hooks as a single deployable unit."/><script type="application/ld+json">{"@context":"https://schema.org","@graph":[{"@type":"TechArticle","headline":"Features - Antigravity CLI","description":"How plugins work Plugins are namespaced bundles that can contain skills, agents, rules, MCP servers, and hooks as a single deployable unit.","url":"https://antigravity.google/docs/cli/features/","mainEntityOfPage":{"@type":"WebPage","@id":"https://antigravity.google/docs/cli/features/"},"author":{"@type":"Organization","name":"Google Antigravity","url":"https://antigravity.google/"},"publisher":{"@type":"Organization","name":"Google","url":"https://www.google.com/"}},{"@type":"BreadcrumbList","itemListElement":[{"@type":"ListItem","position":1,"name":"Docs","item":"https://antigravity.google/docs/home/"},{"@type":"ListItem","position":2,"name":"Antigravity CLI","item":"https://antigravity.google/docs/cli/overview/"},{"@type":"ListItem","position":3,"name":"Features","item":"https://antigravity.google/docs/cli/features/"}]}]}</script><link href="https://www.gstatic.com/glue/cookienotificationbar/cookienotificationbar.min.css" rel="stylesheet">\<script\>
  window.dataLayer = window.dataLayer || [];
  window.__agyGtmLoaded = window.__agyGtmLoaded || false;
  window.__agyLastPageViewPath = window.__agyLastPageViewPath || '';

  function __agyDetectViewport() {
    const width = Math.round(
      window.innerWidth ||
        (document.documentElement && document.documentElement.clientWidth) ||
        1440
    );
    const height = Math.round(
      window.innerHeight ||
        (document.documentElement && document.documentElement.clientHeight) ||
        900
    );
    const orientation = width >= height ? 'landscape' : 'portrait';
    const minDim = Math.min(width, height);
    const maxDim = Math.max(width, height);
    const aspectRatio = minDim > 0 ? maxDim / minDim : 1;
    const ua =
      typeof navigator !== 'undefined' ? navigator.userAgent || '' : '';

    const hasSegments =
      Boolean(
        window.visualViewport &&
          window.visualViewport.segments &&
          window.visualViewport.segments.length > 1
      ) ||
      Boolean(
        typeof navigator !== 'undefined' &&
          navigator.devicePosture &&
          navigator.devicePosture.type === 'folded'
      ) ||
      Boolean(
        window.matchMedia &&
          window.matchMedia(
            '(horizontal-viewport-segments: 2), (vertical-viewport-segments: 2)'
          ).matches
      );
    const isFoldableUA =
      /Pixel Fold|SM-F9|SM-F7|Galaxy Z Fold|Surface Duo/i.test(ua);
    const isFoldableUnfolded =
      minDim >= 600 && maxDim <= 1150 && aspectRatio <= 1.3;
    const isFoldableCover =
      (width >= 320 &&
        width <= 368 &&
        height >= 700 &&
        height / width >= 2.0) ||
      (height >= 320 &&
        height <= 368 &&
        width >= 700 &&
        width / height >= 2.0);

    const isIpad =
      /ipad/i.test(ua) ||
      (typeof navigator !== 'undefined' &&
        navigator.maxTouchPoints > 0 &&
        /macintosh/i.test(ua));
    let os = 'unknown';
    if (/iphone|ipod/i.test(ua)) os = 'iphone';
    else if (isIpad) os = 'ipad';
    else if (/googlebook|cros/i.test(ua)) os = 'googlebook';
    else if (/android/i.test(ua)) os = 'android';
    else if (/windows/i.test(ua)) os = 'windows';
    else if (/macintosh|mac os x/i.test(ua)) os = 'mac';
    else if (/linux/i.test(ua)) os = 'linux';

    let viewportCategory = 'desktop';
    if (
      hasSegments ||
      isFoldableUA ||
      isFoldableUnfolded ||
      isFoldableCover
    ) {
      viewportCategory = 'foldable';
    } else if (width < 600) {
      viewportCategory = 'mobile';
    } else if (width <= 1024 || isIpad) {
      viewportCategory = 'tablet';
    }

    return {
      viewportCategory: viewportCategory,
      formFactor: viewportCategory,
      viewportWidth: width,
      viewportHeight: height,
      orientation: orientation,
      operatingSystem: os,
    };
  }

  function __agyRecordPageView() {
    const currentPath = window.location.pathname || '/';
    if (window.__agyLastPageViewPath === currentPath) return;

    if (
      window.agyAnalytics &&
      typeof window.agyAnalytics.trackPageView === 'function'
    ) {
      window.agyAnalytics.trackPageView();
      return;
    }

    window.__agyLastPageViewPath = currentPath;
    const vp = __agyDetectViewport();
    const eventParams = {
      pagePath: currentPath,
      viewportCategory: vp.viewportCategory,
      formFactor: vp.formFactor,
      viewportWidth: vp.viewportWidth,
      viewportHeight: vp.viewportHeight,
      orientation: vp.orientation,
      operatingSystem: vp.operatingSystem,
    };
    if (document.title) {
      eventParams.pageTitle = document.title;
    }
    if (document.referrer) {
      eventParams.referrer = document.referrer;
    }
    try {
      const sp = new URLSearchParams(window.location.search || '');
      const utmSource = sp.get('utm_source');
      const utmMedium = sp.get('utm_medium');
      const utmCampaign = sp.get('utm_campaign');
      const utmTerm = sp.get('utm_term');
      const utmContent = sp.get('utm_content');
      if (utmSource) {
        eventParams.utm_source = utmSource;
        eventParams.utmSource = utmSource;
      }
      if (utmMedium) {
        eventParams.utm_medium = utmMedium;
        eventParams.utmMedium = utmMedium;
      }
      if (utmCampaign) {
        eventParams.utm_campaign = utmCampaign;
        eventParams.utmCampaign = utmCampaign;
      }
      if (utmTerm) {
        eventParams.utm_term = utmTerm;
        eventParams.utmTerm = utmTerm;
      }
      if (utmContent) {
        eventParams.utm_content = utmContent;
        eventParams.utmContent = utmContent;
      }
    } catch (_err) {
      // Ignore URLSearchParams errors
    }

    const entry = {
      event: 'page_view',
      pagePath: currentPath,
      viewportCategory: vp.viewportCategory,
      formFactor: vp.formFactor,
      viewportWidth: vp.viewportWidth,
      viewportHeight: vp.viewportHeight,
      orientation: vp.orientation,
      operatingSystem: vp.operatingSystem,
      eventParams: eventParams,
    };
    if (eventParams.pageTitle) entry.pageTitle = eventParams.pageTitle;
    if (eventParams.referrer) entry.referrer = eventParams.referrer;
    if (eventParams.utm_source) {
      entry.utm_source = eventParams.utm_source;
      entry.utmSource = eventParams.utmSource;
    }
    if (eventParams.utm_medium) {
      entry.utm_medium = eventParams.utm_medium;
      entry.utmMedium = eventParams.utmMedium;
    }
    if (eventParams.utm_campaign) {
      entry.utm_campaign = eventParams.utm_campaign;
      entry.utmCampaign = eventParams.utmCampaign;
    }
    if (eventParams.utm_term) {
      entry.utm_term = eventParams.utm_term;
      entry.utmTerm = eventParams.utmTerm;
    }
    if (eventParams.utm_content) {
      entry.utm_content = eventParams.utm_content;
      entry.utmContent = eventParams.utmContent;
    }

    window.dataLayer.push(entry);
    window.dataLayer.push({ eventParams: undefined });
  }

  __agyRecordPageView();

  window.glueCookieNotificationBarLoaded = function () {
    if (window.__agyGtmLoaded) return;
    window.__agyGtmLoaded = true;
    (function (w, d, s, l, i) {
      w[l] = w[l] || [];
      w[l].push({
        'gtm.start': new Date().getTime(),
        event: 'gtm.js',
      });
      const f = d.getElementsByTagName(s)[0],
        j = d.createElement(s),
        dl = l != 'dataLayer' ? '&l=' + l : '';
      j.async = true;
      j.src = 'https://www.googletagmanager.com/gtm.js?id=' + i + dl;
      f.parentNode.insertBefore(j, f);
    })(window, document, 'script', 'dataLayer', 'GTM-M4N2ZKXQ');
    __agyRecordPageView();
  };

  document.addEventListener('astro:page-load', function () {
    __agyRecordPageView();
  });
</script><script type="module" src="/_astro/AnalyticsBootstrap.astro_astro_type_script_index_0_lang.DQy9iQ8q.js"></script><script defer src="https://www.gstatic.com/glue/cookienotificationbar/cookienotificationbar.min.js" data-glue-cookie-notification-bar-category="2A" data-glue-cookie-notification-bar-language="" data-glue-cookie-notification-bar-site-id=""></script>\<script\>
	window.StarlightThemeProvider = (() => {
		const storedTheme =
			typeof localStorage !== 'undefined' && localStorage.getItem('starlight-theme');
		const theme =
			storedTheme ||
			(window.matchMedia('(prefers-color-scheme: light)').matches ? 'light' : 'dark');
		document.documentElement.dataset.theme = theme === 'light' ? 'light' : 'dark';
		return {
			updatePickers(theme = storedTheme || 'auto') {
				document.querySelectorAll('starlight-theme-select').forEach((picker) => {
					const select = picker.querySelector('select');
					if (select) select.value = theme;
					/** @type {HTMLTemplateElement | null} */
					const tmpl = document.querySelector(`#theme-icons`);
					const newIcon = tmpl && tmpl.content.querySelector('.' + theme);
					if (newIcon) {
						const oldIcon = picker.querySelector('svg.label-icon');
						if (oldIcon) {
							oldIcon.replaceChildren(...newIcon.cloneNode(true).childNodes);
						}
					}
				});
			},
		};
	})();
</script><template id="theme-icons"><svg aria-hidden="true" class="light astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1em;"><path d="M5 12a1 1 0 0 0-1-1H3a1 1 0 0 0 0 2h1a1 1 0 0 0 1-1Zm.64 5-.71.71a1 1 0 0 0 0 1.41 1 1 0 0 0 1.41 0l.71-.71A1 1 0 0 0 5.64 17ZM12 5a1 1 0 0 0 1-1V3a1 1 0 0 0-2 0v1a1 1 0 0 0 1 1Zm5.66 2.34a1 1 0 0 0 .7-.29l.71-.71a1 1 0 1 0-1.41-1.41l-.66.71a1 1 0 0 0 0 1.41 1 1 0 0 0 .66.29Zm-12-.29a1 1 0 0 0 1.41 0 1 1 0 0 0 0-1.41l-.71-.71a1.004 1.004 0 1 0-1.43 1.41l.73.71ZM21 11h-1a1 1 0 0 0 0 2h1a1 1 0 0 0 0-2Zm-2.64 6A1 1 0 0 0 17 18.36l.71.71a1 1 0 0 0 1.41 0 1 1 0 0 0 0-1.41l-.76-.66ZM12 6.5a5.5 5.5 0 1 0 5.5 5.5A5.51 5.51 0 0 0 12 6.5Zm0 9a3.5 3.5 0 1 1 0-7 3.5 3.5 0 0 1 0 7Zm0 3.5a1 1 0 0 0-1 1v1a1 1 0 0 0 2 0v-1a1 1 0 0 0-1-1Z"/></svg><svg aria-hidden="true" class="dark astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1em;"><path d="M21.64 13a1 1 0 0 0-1.05-.14 8.049 8.049 0 0 1-3.37.73 8.15 8.15 0 0 1-8.14-8.1 8.59 8.59 0 0 1 .25-2A1 1 0 0 0 8 2.36a10.14 10.14 0 1 0 14 11.69 1 1 0 0 0-.36-1.05Zm-9.5 6.69A8.14 8.14 0 0 1 7.08 5.22v.27a10.15 10.15 0 0 0 10.14 10.14 9.784 9.784 0 0 0 2.1-.22 8.11 8.11 0 0 1-7.18 4.32v-.04Z"/></svg><svg aria-hidden="true" class="auto astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1em;"><path d="M21 14h-1V7a3 3 0 0 0-3-3H7a3 3 0 0 0-3 3v7H3a1 1 0 0 0-1 1v2a3 3 0 0 0 3 3h14a3 3 0 0 0 3-3v-2a1 1 0 0 0-1-1ZM6 7a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v7H6V7Zm14 10a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1v-1h16v1Z"/></svg></template><link rel="stylesheet" href="/_astro/print.ehPL0gv-.css" media="print"><link rel="stylesheet" href="/_astro/index.Bb-U7Th2.css">\<style\>@layer starlight.components{svg:where(.astro-n5k4ipy7){color:var(--sl-icon-color);font-size:var(--sl-icon-size,1em);width:1em;height:1em}}
.button{font-size:var(--base-size);line-height:var(--cta-line-height);letter-spacing:var(--cta-letter-spacing);border-radius:var(--shape-corner-rounded,999px);cursor:pointer;text-align:center;border:1px solid #0000;justify-content:center;align-self:flex-start;align-items:center;height:fit-content;padding:10px 24px;font-family:Google Sans Flex,sans-serif;font-weight:450;text-decoration:none;transition:all .15s ease-out;display:inline-flex}.button span{text-decoration:none}.button .icon{color:currentColor;margin:0 var(--space-sm) 0 0;flex-shrink:0;align-items:center;line-height:0;display:inline-flex}.button:disabled{cursor:not-allowed}.button-primary{background:var(--theme-primary-primary,#1a73e8);color:var(--theme-primary-on-primary,#fff)}.button-primary:hover,.button-primary:focus-visible{background:var(--theme-button-states-primary-hovered,#1557b0);outline:none}.button-primary:disabled{background:var(--theme-button-states-disabled);color:var(--theme-button-states-on-disabled)}.button-primary-inverse{background:var(--palette-grey-0);color:var(--palette-grey-1200)}.button-primary-inverse:hover,.button-primary-inverse:focus-visible{background:var(--theme-surface-surface-container-high);outline:none}.button-primary-inverse:active{background:var(--theme-surface-surface-container-high)}.button-primary-inverse:disabled{background:var(--theme-button-states-disabled);color:var(--theme-button-states-on-disabled)}.button-secondary{color:var(--theme-surface-on-surface);background:var(--theme-secondary-button);backdrop-filter:blur(6px);border:1px solid var(--theme-outline-variant)}.button-secondary:hover,.button-secondary:focus-visible{background:var(--theme-button-secondary-hover);outline:none}.button-secondary:disabled{background:var(--theme-button-states-disabled);color:var(--theme-button-states-on-disabled)}.button-secondary-inverse{color:var(--theme-surface-surface);background:var(--theme-nav-button-hover);backdrop-filter:blur(8px);border:1px solid var(--theme-inverse-outline-variant)}.button-secondary-inverse:hover,.button-secondary-inverse:focus-visible{background:var(--theme-button-secondary-inverse-hover);outline:none}.button-secondary-inverse:disabled{background:var(--palette-grey-1100);color:var(--palette-grey-50-20)}.button-text,.button-nav{background:0 0}.button-text{color:var(--theme-text-on-text)}.button-text:hover,.button-text:focus-visible{background:var(--theme-button-states-hovered);outline:none}.button-text:active{background:var(--theme-button-states-pressed)}.button-text:disabled{color:var(--theme-button-states-on-disabled)}.button-compact{min-block-size:36px;padding:6px 16px}.download-button,.button-nav{font-size:var(--cta-sm-size);line-height:var(--cta-sm-line-height);letter-spacing:var(--cta-sm-letter-spacing);font-weight:450}.button-nav{color:var(--theme-surface-on-surface-variant)}.button-nav:hover,.button-nav:focus-visible{color:#000;background:var(--theme-nav-button-hover);outline:none}@media (width<=767px){.button-nav{width:100%}}.dropdown-nav{padding-right:8px}.dropdown-icon,.play-icon{letter-spacing:normal;text-transform:none;white-space:nowrap;word-wrap:normal;-webkit-font-smoothing:antialiased;text-rendering:optimizelegibility;font-feature-settings:"liga";-webkit-font-feature-settings:"liga";font-variation-settings:"FILL" 0, "wght" 300, "GRAD" 0, "ROND" 50, "opsz" 48;margin:0 0 0 var(--space-xs);transition:transform .3s all;direction:ltr;flex-shrink:0;justify-content:center;align-items:center;font-family:Google Symbols;font-size:1.5em;font-weight:300;line-height:1;display:inline-flex}.dropdown-icon svg,.play-icon svg{fill:currentColor;display:block}.button.dropdown-open .dropdown-icon,.button[aria-expanded=true] .dropdown-icon,.dropdown-open>.dropdown-icon{transform:rotate(-180deg)}.play-icon{font-variation-settings:"FILL" 1, "wght" 300, "GRAD" 0, "ROND" 50, "opsz" 48}@media (width<=1024px){.nav-button{padding:var(--space-md);width:100%}}
</style><script type="module" src="/_astro/page.LAbJoB63.js"></script></head><body class="astro-vdxcvpsm"><a class="sl-skip-link astro-vxeigawq" href="#_top">Skip to content</a><div class="page sl-flex astro-gqpr5s5a"><header class="header astro-gqpr5s5a"><div class="header-stack astro-mtusqnhg" data-analytics-nav="docs_header"><!-- Row 1: Logo and Search / Controls --><div class="header-main astro-mtusqnhg"><div class="title-wrapper sl-flex astro-mtusqnhg"><a href="/" class="site-title sl-flex astro-clw6ags5"><img class="light:sl-hidden print:hidden astro-clw6ags5" alt src="/_astro/antigravity_product_lockup_dark.YcYY_PSr.svg" width="1687" height="217"><img class="dark:sl-hidden print:block astro-clw6ags5" alt src="/_astro/antigravity_product_lockup_full_color.cUD_Se6a.svg" width="1687" height="217"><span class="sr-only astro-clw6ags5" translate="no">Google Antigravity Docs</span></a></div><div class="sl-flex search-wrapper print:hidden astro-mtusqnhg"><site-search class="astro-mtusqnhg astro-r5ar4xm7" data-translations="{"placeholder":"Search"}"><button data-open-modal disabled aria-label="Search" aria-keyshortcuts="Control+K" class="astro-r5ar4xm7"><svg aria-hidden="true" class="astro-r5ar4xm7 astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1em;"><path d="M21.71 20.29 18 16.61A9 9 0 1 0 16.61 18l3.68 3.68a.999.999 0 0 0 1.42 0 1 1 0 0 0 0-1.39ZM11 18a7 7 0 1 1 0-14 7 7 0 0 1 0 14Z"/></svg><span class="sl-hidden md:sl-block astro-r5ar4xm7" aria-hidden="true">Search</span><kbd class="sl-hidden md:sl-flex astro-r5ar4xm7" style="display: none;"><kbd class="astro-r5ar4xm7">Ctrl</kbd><kbd class="astro-r5ar4xm7">K</kbd></kbd></button><dialog style="padding:0" aria-label="Search" class="astro-r5ar4xm7"><div class="dialog-frame sl-flex astro-r5ar4xm7"><button data-close-modal class="sl-flex md:sl-hidden astro-r5ar4xm7">Cancel</button><div class="search-container astro-r5ar4xm7"><div id="starlight__search" class="astro-r5ar4xm7"></div></div></div></dialog></site-search>\<script\>
	(() => {
		const openBtn = document.querySelector('button[data-open-modal]');
		const shortcut = openBtn?.querySelector('kbd');
		if (!openBtn || !(shortcut instanceof HTMLElement)) return;
		const platformKey = shortcut.querySelector('kbd');
		if (platformKey && /(Mac|iPhone|iPod|iPad)/i.test(navigator.platform)) {
			platformKey.textContent = '⌘';
			openBtn.setAttribute('aria-keyshortcuts', 'Meta+K');
		}
		shortcut.style.display = '';
	})();
</script><script type="module" src="/_astro/Search.astro_astro_type_script_index_0_lang.vTFUHulS.js"></script></div><div class="sl-hidden md:sl-flex print:hidden right-group astro-mtusqnhg"><div class="header-cta-group astro-mtusqnhg"><a class="top-nav-btn remote-control-btn astro-mtusqnhg" href="https://antigravity.google.com" target="_blank" rel="noopener noreferrer" title="Launch Remote Control"><span class="btn-icon astro-mtusqnhg" aria-hidden="true">🚀</span> Remote Control</a><a href="/download" class="call-to-action--nav button button-primary button-compact download-button dropdown-nav" aria-haspopup="false" aria-expanded="false" data-analytics-event="download-cta" data-module-name="header" data-page-position="main nav" class="astro-mtusqnhg"><span>Download</span><span class="dropdown-icon" translate="no"><svg class viewBox="0 0 24 24" width="18" height="18" fill="currentColor" aria-hidden="true" xmlns="http://www.w3.org/2000/svg"><path d="M12 16L7 11L8.4 9.55L11 12.15V4H13V12.15L15.6 9.55L17 11L12 16ZM6 20C5.45 20 4.979 19.804 4.588 19.413C4.196 19.021 4.00067 18.55 4 18V15H6V18H18V15H20V18C20 18.55 19.804 19.021 19.413 19.413C19.021 19.804 18.55 20 18 20H6Z"></path></svg></span></a></div><div class="sl-flex social-icons astro-mtusqnhg"></div><starlight-theme-select><label style="--sl-select-width: 6.25em" class="astro-r2p65idu"><span class="sr-only astro-r2p65idu">Select theme</span><svg aria-hidden="true" class="icon label-icon astro-r2p65idu astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1em;"><path d="M21 14h-1V7a3 3 0 0 0-3-3H7a3 3 0 0 0-3 3v7H3a1 1 0 0 0-1 1v2a3 3 0 0 0 3 3h14a3 3 0 0 0 3-3v-2a1 1 0 0 0-1-1ZM6 7a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v7H6V7Zm14 10a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1v-1h16v1Z"/></svg><select autocomplete="off" class="astro-r2p65idu"><option value="dark" class="astro-r2p65idu">Dark</option><option value="light" class="astro-r2p65idu">Light</option><option value="auto" selected class="astro-r2p65idu">Auto</option></select><svg aria-hidden="true" class="icon caret astro-r2p65idu astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1em;"><path d="M17 9.17a1 1 0 0 0-1.41 0L12 12.71 8.46 9.17a1 1 0 1 0-1.41 1.42l4.24 4.24a1.002 1.002 0 0 0 1.42 0L17 10.59a1.002 1.002 0 0 0 0-1.42Z"/></svg></label></starlight-theme-select>\<script\>
	StarlightThemeProvider.updatePickers();
</script><script type="module">var e=`starlight-theme`,t=e=>e===`auto`||e===`dark`||e===`light`?e:`auto`,n=()=>t(typeof localStorage<`u`&&localStorage.getItem(e));function r(t){typeof localStorage<`u`&&localStorage.setItem(e,t===`light`||t===`dark`?t:``)}var i=()=>matchMedia(`(prefers-color-scheme: light)`).matches?`light`:`dark`;function a(e){StarlightThemeProvider.updatePickers(e),document.documentElement.dataset.theme=e===`auto`?i():e,r(e)}matchMedia(`(prefers-color-scheme: light)`).addEventListener(`change`,()=>{n()===`auto`&&a(`auto`)});var o=class extends HTMLElement{constructor(){super(),a(n()),this.querySelector(`select`)?.addEventListener(`change`,e=>{e.currentTarget instanceof HTMLSelectElement&&a(t(e.currentTarget.value))})}};customElements.define(`starlight-theme-select`,o);</script><script type="module">var e=class extends HTMLElement{constructor(){super();let e=this.querySelector(`select`);e&&(e.addEventListener(`change`,e=>{e.currentTarget instanceof HTMLSelectElement&&(window.location.pathname=e.currentTarget.value)}),window.addEventListener(`pageshow`,t=>{if(!t.persisted)return;let n=e.querySelector(`option[selected]`)?.index;n!==e.selectedIndex&&(e.selectedIndex=n??0)}))}};customElements.define(`starlight-lang-select`,e);</script></div></div><!-- Row 2: Subnav Bar below logo and controls (Get Started, Features, etc.) --><nav class="subnav-bar print:hidden astro-mtusqnhg" aria-label="Documentation sections"><div class="subnav-container astro-mtusqnhg"><div class="subnav-left astro-mtusqnhg"><a href="/docs/getting-started/" class="subnav-pill active astro-mtusqnhg" aria-current="page">Get Started</a><a href="/docs/features/" class="subnav-pill astro-mtusqnhg">Features</a><a href="/docs/sdk/overview/" class="subnav-pill astro-mtusqnhg">SDK</a></div><div class="subnav-right astro-mtusqnhg"><a href="/docs/changelog/" class="subnav-pill astro-mtusqnhg">Changelog</a><a href="/blog/" class="subnav-pill external-link blog-link astro-mtusqnhg">Blog <span class="arrow astro-mtusqnhg" aria-hidden="true">↗</span></a></div></div></nav></div></header><nav class="sidebar print:hidden astro-gqpr5s5a" aria-label="Main"><starlight-menu-button class="print:hidden astro-vvx7mffu"><button aria-expanded="false" aria-label="Menu" aria-controls="starlight__sidebar" class="sl-flex md:sl-hidden astro-vvx7mffu"><svg aria-hidden="true" class="open-menu astro-vvx7mffu astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1em;"><path d="M3 8h18a1 1 0 1 0 0-2H3a1 1 0 0 0 0 2Zm18 8H3a1 1 0 0 0 0 2h18a1 1 0 0 0 0-2Zm0-5H3a1 1 0 0 0 0 2h18a1 1 0 0 0 0-2Z"/></svg><svg aria-hidden="true" class="close-menu astro-vvx7mffu astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1em;"><path d="m13.41 12 6.3-6.29a1.004 1.004 0 1 0-1.42-1.42L12 10.59l-6.29-6.3a1.004 1.004 0 0 0-1.42 1.42l6.3 6.29-6.3 6.29a1 1 0 0 0 0 1.42.998.998 0 0 0 1.42 0l6.29-6.3 6.29 6.3a.999.999 0 0 0 1.42 0 1 1 0 0 0 0-1.42L13.41 12Z"/></svg></button></starlight-menu-button><script type="module">var e=class extends HTMLElement{btn=this.querySelector(`button`);constructor(){super(),this.btn.addEventListener(`click`,()=>this.toggleExpanded());let e=this.closest(`nav`);e&&e.addEventListener(`keyup`,e=>this.closeOnEscape(e)),matchMedia(`(min-width: 50em)`).addEventListener(`change`,()=>this.setExpanded(!1))}setExpanded(e){this.setAttribute(`aria-expanded`,String(e)),document.body.toggleAttribute(`data-mobile-menu-expanded`,e),document.querySelectorAll(`.main-frame, .sl-skip-link`).forEach(t=>{t.toggleAttribute(`inert`,e)})}toggleExpanded(){this.setExpanded(this.getAttribute(`aria-expanded`)!==`true`)}closeOnEscape(e){e.code===`Escape`&&(this.setExpanded(!1),this.btn.focus())}};customElements.define(`starlight-menu-button`,e);</script><div id="starlight__sidebar" class="sidebar-pane astro-gqpr5s5a"><div class="sidebar-content sl-flex astro-gqpr5s5a"><div class="sidebar-wrapper astro-s3sbtxwf" data-default-section="get-started" data-analytics-nav="docs_sidebar"><sl-sidebar-state-persist data-hash="1egz4k8" class="astro-f5yvicm7"><script aria-hidden="true">
		(() => {
			try {
				if (!matchMedia('(min-width: 50em)').matches) return;
				/** @type {HTMLElement | null} */
				const target = document.querySelector('sl-sidebar-state-persist');
				const state = JSON.parse(sessionStorage.getItem('sl-sidebar-state') || '0');
				if (!target || !state || target.dataset.hash !== state.hash) return;
				window._starlightScrollRestore = state.scroll;
				customElements.define(
					'sl-sidebar-restore',
					class SidebarRestore extends HTMLElement {
						connectedCallback() {
							try {
								const idx = parseInt(this.dataset.index || '');
								const details = this.closest('details');
								if (details && typeof state.open[idx] === 'boolean') details.open = state.open[idx];
							} catch {}
						}
					}
				);
			} catch {}
		})();
	</script><div class="sidebar-sections is-active astro-s3sbtxwf" data-sidebar-section="get-started"><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">GET STARTED</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/home/" class="astro-ojxuodky"><span class="astro-ojxuodky">Home</span></a></li><li class="astro-ojxuodky"><a href="/docs/getting-started/" class="astro-ojxuodky"><span class="astro-ojxuodky">Getting started</span></a></li></ul></section><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">ANTIGRAVITY 2.0</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/overview/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/build-with-google/" class="astro-ojxuodky"><span class="astro-ojxuodky">Build with Google</span></a></li><li class="astro-ojxuodky"><a href="/docs/models/" class="astro-ojxuodky"><span class="astro-ojxuodky">Models</span></a></li></ul></section><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">CLI</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/cli/overview/" class="astro-ojxuodky"><span class="astro-ojxuodky">CLI overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/install/" class="astro-ojxuodky"><span class="astro-ojxuodky">Installation and auth</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/tutorial/" class="astro-ojxuodky"><span class="astro-ojxuodky">Tutorial</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/using/" class="astro-ojxuodky"><span class="astro-ojxuodky">Using Antigravity CLI</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/features/" aria-current="page" class="astro-ojxuodky"><span class="astro-ojxuodky">Features</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/best-practices/" class="astro-ojxuodky"><span class="astro-ojxuodky">Best practices</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/troubleshooting/" class="astro-ojxuodky"><span class="astro-ojxuodky">Troubleshooting</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/reference/" class="astro-ojxuodky"><span class="astro-ojxuodky">CLI reference</span></a></li></ul></section><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">ANTIGRAVITY FOR IDES</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/ide/overview/" class="astro-ojxuodky"><span class="astro-ojxuodky">Antigravity IDE</span></a></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">IDE extensions</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="0"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/ide/extensions/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/extensions/vscode/" class="astro-ojxuodky"><span class="astro-ojxuodky">Visual Studio Code</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/extensions/visual-studio/" class="astro-ojxuodky"><span class="astro-ojxuodky">Visual Studio</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/extensions/jetbrains/" class="astro-ojxuodky"><span class="astro-ojxuodky">JetBrains</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/extensions/zed/" class="astro-ojxuodky"><span class="astro-ojxuodky">Zed</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/extensions/xcode/" class="astro-ojxuodky"><span class="astro-ojxuodky">Xcode</span></a></li></ul></details></li></ul></section><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">RESOURCES</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/faq/" class="astro-ojxuodky"><span class="astro-ojxuodky">FAQ</span></a></li><li class="astro-ojxuodky"><a href="/docs/plans/" class="astro-ojxuodky"><span class="astro-ojxuodky">Plans and AI credits</span></a></li><li class="astro-ojxuodky"><a href="/docs/enterprise/" class="astro-ojxuodky"><span class="astro-ojxuodky">Enterprise overview</span></a></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Migrations</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="1"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/firebase-studio-migration/" class="astro-ojxuodky"><span class="astro-ojxuodky">Firebase Studio migration</span></a></li><li class="astro-ojxuodky"><a href="/docs/migration/workflows-to-skills/" class="astro-ojxuodky"><span class="astro-ojxuodky">Workflows to skills migration</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/gcli-migration/" class="astro-ojxuodky"><span class="astro-ojxuodky">GCLI migration</span></a></li></ul></details></li></ul></section></div><div class="sidebar-sections astro-s3sbtxwf" data-sidebar-section="features"><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">LEARN</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/features/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/remote-control/" class="astro-ojxuodky"><span class="astro-ojxuodky">Remote Control</span></a></li><li class="astro-ojxuodky"><a href="/docs/marketplace/" class="astro-ojxuodky"><span class="astro-ojxuodky">Marketplace</span></a></li><li class="astro-ojxuodky"><a href="/docs/plugins/" class="astro-ojxuodky"><span class="astro-ojxuodky">Plugins</span></a></li><li class="astro-ojxuodky"><a href="/docs/sandbox/" class="astro-ojxuodky"><span class="astro-ojxuodky">Terminal sandbox</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/review-changes-editor/" class="astro-ojxuodky"><span class="astro-ojxuodky">Review changes</span></a></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Terminal</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="2"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/cli/modes/" class="astro-ojxuodky"><span class="astro-ojxuodky">Execution modes</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/headless/" class="astro-ojxuodky"><span class="astro-ojxuodky">Headless mode</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/statusline/" class="astro-ojxuodky"><span class="astro-ojxuodky">Status line customization</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/title/" class="astro-ojxuodky"><span class="astro-ojxuodky">Terminal title customization</span></a></li></ul></details></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Browser</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="3"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/ide/browser/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/browser-recordings/" class="astro-ojxuodky"><span class="astro-ojxuodky">Browser recordings</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/allowlist-denylist/" class="astro-ojxuodky"><span class="astro-ojxuodky">Allowlist and denylist</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/separate-chrome-profile/" class="astro-ojxuodky"><span class="astro-ojxuodky">Separate Chrome profile</span></a></li></ul></details></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Artifacts</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="4"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/artifacts/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/implementation-plan/" class="astro-ojxuodky"><span class="astro-ojxuodky">Implementation plan</span></a></li><li class="astro-ojxuodky"><a href="/docs/walkthrough/" class="astro-ojxuodky"><span class="astro-ojxuodky">Walkthrough</span></a></li><li class="astro-ojxuodky"><a href="/docs/screenshots/" class="astro-ojxuodky"><span class="astro-ojxuodky">Visual screenshots</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/artifacts/" class="astro-ojxuodky"><span class="astro-ojxuodky">Reviewing artifacts (CLI)</span></a></li></ul></details></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Projects</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="5"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/projects/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/prompting/" class="astro-ojxuodky"><span class="astro-ojxuodky">Prompting and interaction</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/conversations/" class="astro-ojxuodky"><span class="astro-ojxuodky">Managing conversations</span></a></li></ul></details></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Permissions</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="6"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/permissions/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/credits/" class="astro-ojxuodky"><span class="astro-ojxuodky">AI credits</span></a></li></ul></details></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Editor features</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="7"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/ide/tab/" class="astro-ojxuodky"><span class="astro-ojxuodky">Tab completion</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/agent-side-panel/" class="astro-ojxuodky"><span class="astro-ojxuodky">Agent side panel</span></a></li><li class="astro-ojxuodky"><a href="/docs/agent/" class="astro-ojxuodky"><span class="astro-ojxuodky">Agent overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/tools/" class="astro-ojxuodky"><span class="astro-ojxuodky">Task groups</span></a></li></ul></details></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Settings</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="8"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/settings/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/agent-settings/" class="astro-ojxuodky"><span class="astro-ojxuodky">Agent settings</span></a></li><li class="astro-ojxuodky"><a href="/docs/artifact-review/" class="astro-ojxuodky"><span class="astro-ojxuodky">Artifact review</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/vim-editor-mode/" class="astro-ojxuodky"><span class="astro-ojxuodky">Vim editor mode</span></a></li></ul></details></li></ul></section><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">SLASH COMMANDS</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/slash-commands/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/boost/" class="astro-ojxuodky"><span class="astro-ojxuodky">Boost (/boost)</span></a></li><li class="astro-ojxuodky"><a href="/docs/plan/" class="astro-ojxuodky"><span class="astro-ojxuodky">Plan (/plan)</span></a></li><li class="astro-ojxuodky"><a href="/docs/teamwork/" class="astro-ojxuodky"><span class="astro-ojxuodky">Teamwork (/teamwork-preview)</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/agents/" class="astro-ojxuodky"><span class="astro-ojxuodky">Agents (/agents)</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/codesearch/" class="astro-ojxuodky"><span class="astro-ojxuodky">Code search (/codesearch)</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/credits/" class="astro-ojxuodky"><span class="astro-ojxuodky">AI credits (/credits)</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/diff/" class="astro-ojxuodky"><span class="astro-ojxuodky">Diff (/diff)</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/permissions/" class="astro-ojxuodky"><span class="astro-ojxuodky">Permissions (/permissions)</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/resume/" class="astro-ojxuodky"><span class="astro-ojxuodky">Resume (/resume)</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/statusline/" class="astro-ojxuodky"><span class="astro-ojxuodky">Status line (/statusline)</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/title/" class="astro-ojxuodky"><span class="astro-ojxuodky">Window title (/title)</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/usage/" class="astro-ojxuodky"><span class="astro-ojxuodky">Model quotas (/usage)</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/voice/" class="astro-ojxuodky"><span class="astro-ojxuodky">Voice dictation (/voice)</span></a></li></ul></section><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">EXTEND</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/mcp/" class="astro-ojxuodky"><span class="astro-ojxuodky">MCP</span></a></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Skills</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="9"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/skills/" class="astro-ojxuodky"><span class="astro-ojxuodky">Agent skills</span></a></li><li class="astro-ojxuodky"><a href="/docs/rules/" class="astro-ojxuodky"><span class="astro-ojxuodky">Rules</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/workflows/" class="astro-ojxuodky"><span class="astro-ojxuodky">IDE workflows</span></a></li></ul></details></li><li class="astro-ojxuodky"><a href="/docs/subagents/" class="astro-ojxuodky"><span class="astro-ojxuodky">Custom subagents</span></a></li><li class="astro-ojxuodky"><a href="/docs/hooks/" class="astro-ojxuodky"><span class="astro-ojxuodky">Lifecycle hooks</span></a></li><li class="astro-ojxuodky"><a href="/docs/sidecars/" class="astro-ojxuodky"><span class="astro-ojxuodky">Sidecars</span></a></li></ul></section></div><div class="sidebar-sections astro-s3sbtxwf" data-sidebar-section="sdk"><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">ANTIGRAVITY SDK</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/sdk/overview/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview and quickstart</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/local-models/" class="astro-ojxuodky"><span class="astro-ojxuodky">Local models</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/personas/" class="astro-ojxuodky"><span class="astro-ojxuodky">Personas</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/tools/" class="astro-ojxuodky"><span class="astro-ojxuodky">Tools and skills</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/mcp/" class="astro-ojxuodky"><span class="astro-ojxuodky">MCP</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/policies/" class="astro-ojxuodky"><span class="astro-ojxuodky">Policies</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/subagents/" class="astro-ojxuodky"><span class="astro-ojxuodky">Subagents</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/structured-output/" class="astro-ojxuodky"><span class="astro-ojxuodky">Structured output</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/lifecycle/" class="astro-ojxuodky"><span class="astro-ojxuodky">Lifecycle and hooks</span></a></li></ul></section></div><div class="sidebar-sections astro-s3sbtxwf" data-sidebar-section="changelog"></div><script aria-hidden="true">
		(() => {
			const scroller = document.getElementById('starlight__sidebar');
			if (!window._starlightScrollRestore || !scroller) return;
			scroller.scrollTop = window._starlightScrollRestore;
			delete window._starlightScrollRestore;
		})();
	</script></sl-sidebar-state-persist><div class="md:sl-hidden astro-s3sbtxwf"><div class="mobile-preferences sl-flex astro-b5gddk54"><div class="social-icons astro-b5gddk54"></div><starlight-theme-select><label style="--sl-select-width: 6.25em" class="astro-r2p65idu"><span class="sr-only astro-r2p65idu">Select theme</span><svg aria-hidden="true" class="icon label-icon astro-r2p65idu astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1em;"><path d="M21 14h-1V7a3 3 0 0 0-3-3H7a3 3 0 0 0-3 3v7H3a1 1 0 0 0-1 1v2a3 3 0 0 0 3 3h14a3 3 0 0 0 3-3v-2a1 1 0 0 0-1-1ZM6 7a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v7H6V7Zm14 10a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1v-1h16v1Z"/></svg><select autocomplete="off" class="astro-r2p65idu"><option value="dark" class="astro-r2p65idu">Dark</option><option value="light" class="astro-r2p65idu">Light</option><option value="auto" selected class="astro-r2p65idu">Auto</option></select><svg aria-hidden="true" class="icon caret astro-r2p65idu astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1em;"><path d="M17 9.17a1 1 0 0 0-1.41 0L12 12.71 8.46 9.17a1 1 0 1 0-1.41 1.42l4.24 4.24a1.002 1.002 0 0 0 1.42 0L17 10.59a1.002 1.002 0 0 0 0-1.42Z"/></svg></label></starlight-theme-select>\<script\>
	StarlightThemeProvider.updatePickers();
</script></div></div></div></div></div></nav><div class="main-frame astro-gqpr5s5a"><script type="module">var e=document.getElementById(`starlight__sidebar`),t=e?.querySelector(`sl-sidebar-state-persist`),n=`sl-sidebar-state`,r=()=>{let r=[],i=t?.dataset.hash||``;try{let e=sessionStorage.getItem(n),t=JSON.parse(e||`{}`);Array.isArray(t.open)&&t.hash===i&&(r=t.open)}catch{}return{hash:i,open:r,scroll:e?.scrollTop||0}},i=e=>{try{sessionStorage.setItem(n,JSON.stringify(e))}catch{}},a=()=>i(r()),o=(e,t)=>{let n=r();n.open[t]=e,i(n)};t?.addEventListener(`click`,e=>{if(!(e.target instanceof Element))return;let t=e.target.closest(`summary`)?.closest(`details`);if(!t)return;let n=t.querySelector(`sl-sidebar-restore`),r=parseInt(n?.dataset.index||``);isNaN(r)||o(!t.open,r)}),addEventListener(`visibilitychange`,()=>{document.visibilityState===`hidden`&&a()});</script><div class="lg:sl-flex astro-abfeecmu"><aside class="right-sidebar-container print:hidden astro-abfeecmu"><div class="right-sidebar astro-abfeecmu"><div class="lg:sl-hidden astro-q5mffni3"><mobile-starlight-toc data-min-h="2" data-max-h="3" class="astro-sgnutmzh"><nav aria-labelledby="starlight__on-this-page--mobile" data-analytics-nav="docs_toc" class="astro-sgnutmzh"><details id="starlight__mobile-toc" class="astro-sgnutmzh"><summary id="starlight__on-this-page--mobile" class="sl-flex astro-sgnutmzh"><span class="toggle sl-flex astro-sgnutmzh">On this page<svg aria-hidden="true" class="caret astro-sgnutmzh" width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="
             m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1
             1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002
             1.002 0 0 0 0-1.42Z
            " class="astro-sgnutmzh"></path></svg></span><span class="display-current astro-sgnutmzh"></span></summary><div class="dropdown astro-sgnutmzh"><ul class="isMobile astro-sgnutmzh"><li class="astro-sgnutmzh"><a href="#_top" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Overview</span></a><ul class="astro-sgnutmzh"><li class="astro-sgnutmzh"><a href="#plugins" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Plugins</span></a></li><li class="astro-sgnutmzh"><a href="#manage-plugins-with-plugin-marketplace-and-installed" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Manage plugins with /plugin (Marketplace and installed)</span></a></li><li class="astro-sgnutmzh"><a href="#manage-plugins-from-your-shell-agy-plugin" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Manage plugins from your shell (agy plugin)</span></a></li><li class="astro-sgnutmzh"><a href="#terminal-sandbox" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Terminal sandbox</span></a></li><li class="astro-sgnutmzh"><a href="#cli-slash-commands-reference" class="astro-sgnutmzh"><span class="astro-sgnutmzh">CLI slash commands reference</span></a></li><li class="astro-sgnutmzh"><a href="#core-slash-commands" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Core slash commands</span></a></li><li class="astro-sgnutmzh"><a href="#advanced-customization-using-settingsjson" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Advanced customization using settings.json</span></a></li><li class="astro-sgnutmzh"><a href="#subagents-in-antigravity-cli" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Subagents in Antigravity CLI</span></a></li><li class="astro-sgnutmzh"><a href="#managing-agents-the-agents-panel" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Managing agents: the /agents panel</span></a></li></ul></li></ul></div></details></nav></mobile-starlight-toc><script type="module">var e=class extends HTMLElement{_current=null;_onScroll=null;_observer=null;_cleanup=[];set current(e){if(e===this._current)return;this._current&&this._current.removeAttribute(`aria-current`),e.setAttribute(`aria-current`,`true`),this._current=e;let t=this.querySelector(`.display-current`);t&&(t.textContent=e.textContent)}connectedCallback(){this.init()}disconnectedCallback(){this._onScroll&&=(window.removeEventListener(`scroll`,this._onScroll),null),this._observer&&=(this._observer.disconnect(),null),this._cleanup.forEach(e=>e()),this._cleanup=[]}init=()=>{let e=this.querySelector(`details`);if(e){let t=()=>{e.open=!1};e.querySelectorAll(`a`).forEach(e=>{e.addEventListener(`click`,t)}),window.addEventListener(`click`,n=>{e.contains(n.target)||t()}),window.addEventListener(`keydown`,n=>{n.key===`Escape`&&e.open&&(t(),e.querySelector(`summary`)?.focus())})}let t=Array.from(this.querySelectorAll(`a`));if(!t.length)return;let n=[],r=new Map,i=document.getElementById(`_top`);if(i){let e=t.find(e=>e.hash===`#_top`);if(e){r.set(`_top`,e);let t=e.closest(`li`);t&&n.push({link:e,li:t,heading:i,tabpanel:null})}}t.forEach(e=>{let t=decodeURIComponent(e.hash.slice(1));if(!t||t===`_top`)return;let i=document.getElementById(t),a=e.closest(`li`);if(i&&a){r.set(t,e);let o=i.closest(`[role="tabpanel"]`);n.push({link:e,li:a,heading:i,tabpanel:o})}}),t.forEach(e=>{e.addEventListener(`click`,()=>{let t=decodeURIComponent(e.hash.slice(1));if(!t)return;let n=document.getElementById(t);if(n){let e=n.closest(`starlight-tabs [role="tabpanel"]`);if(e){let t=e.closest(`starlight-tabs`);if(t){let n=Array.from(t.querySelectorAll(`:scope > [role="tabpanel"]`)).indexOf(e);n!==-1&&(t.querySelectorAll(`[role="tab"]`)[n]?.click(),requestAnimationFrame(()=>{s()}))}}}})});let a=!1,o=()=>{a=!1;let e=n.filter(e=>e.heading.id===`_top`||e.heading.offsetParent!==null&&(!e.tabpanel||!e.tabpanel.hasAttribute(`hidden`)&&!e.tabpanel.hidden&&e.tabpanel.offsetParent!==null));if(!e.length)return;let t=e[0];for(let n of e)if(n.heading.getBoundingClientRect().top<=130)t=n;else break;let i=r.get(t.heading.id);i&&(this.current=i)},s=()=>{n.forEach(e=>{if(!e.tabpanel){e.li.style.display=``;return}let t=e.tabpanel.hasAttribute(`hidden`)||e.tabpanel.hidden||e.tabpanel.offsetParent===null;e.li.style.display=t?`none`:``}),o()},c=document.querySelectorAll(`[role="tabpanel"]`);if(c.length>0){this._observer=new MutationObserver(()=>{s()}),c.forEach(e=>{this._observer?.observe(e,{attributes:!0,attributeFilter:[`hidden`,`style`,`class`]})});let e=e=>{let t=e.target;(t?.closest(`[role="tab"]`)||t?.closest(`.surface-tag-pill`)||t?.closest(`.tab-button`))&&requestAnimationFrame(()=>{s()})};document.addEventListener(`click`,e),this._cleanup.push(()=>{document.removeEventListener(`click`,e)})}this._onScroll=()=>{a||=(requestAnimationFrame(o),!0)},window.addEventListener(`scroll`,this._onScroll,{passive:!0}),s()}};customElements.get(`mobile-starlight-toc`)||customElements.define(`mobile-starlight-toc`,e);</script></div><div class="right-sidebar-panel sl-hidden lg:sl-block astro-q5mffni3"><div class="sl-container astro-q5mffni3"><starlight-toc data-min-h="2" data-max-h="3"><nav aria-labelledby="starlight__on-this-page" data-analytics-nav="docs_toc"><h2 id="starlight__on-this-page">On this page</h2><ul class="starlight-toc-list">\<li\><a href="#_top"><span>Overview</span></a>\<ul\>\<li\><a href="#plugins"><span>Plugins</span></a></li>\<li\><a href="#manage-plugins-with-plugin-marketplace-and-installed"><span>Manage plugins with /plugin (Marketplace and installed)</span></a></li>\<li\><a href="#manage-plugins-from-your-shell-agy-plugin"><span>Manage plugins from your shell (agy plugin)</span></a></li>\<li\><a href="#terminal-sandbox"><span>Terminal sandbox</span></a></li>\<li\><a href="#cli-slash-commands-reference"><span>CLI slash commands reference</span></a></li>\<li\><a href="#core-slash-commands"><span>Core slash commands</span></a></li>\<li\><a href="#advanced-customization-using-settingsjson"><span>Advanced customization using settings.json</span></a></li>\<li\><a href="#subagents-in-antigravity-cli"><span>Subagents in Antigravity CLI</span></a></li>\<li\><a href="#managing-agents-the-agents-panel"><span>Managing agents: the /agents panel</span></a></li></ul></li></ul></nav></starlight-toc><script type="module">var e=class extends HTMLElement{_current=null;_onScroll=null;_observer=null;_cleanup=[];set current(e){e!==this._current&&(this._current&&this._current.removeAttribute(`aria-current`),e.setAttribute(`aria-current`,`true`),this._current=e)}connectedCallback(){this.init()}disconnectedCallback(){this._onScroll&&=(window.removeEventListener(`scroll`,this._onScroll),null),this._observer&&=(this._observer.disconnect(),null),this._cleanup.forEach(e=>e()),this._cleanup=[]}init=()=>{let e=Array.from(this.querySelectorAll(`a`));if(!e.length)return;let t=[],n=new Map,r=document.getElementById(`_top`);if(r){let i=e.find(e=>e.hash===`#_top`);if(i){n.set(`_top`,i);let e=i.closest(`li`);e&&t.push({link:i,li:e,heading:r,tabpanel:null})}}e.forEach(e=>{let r=decodeURIComponent(e.hash.slice(1));if(!r||r===`_top`)return;let i=document.getElementById(r),a=e.closest(`li`);if(i&&a){n.set(r,e);let o=i.closest(`[role="tabpanel"]`);t.push({link:e,li:a,heading:i,tabpanel:o})}}),e.forEach(e=>{e.addEventListener(`click`,()=>{let t=decodeURIComponent(e.hash.slice(1));if(!t)return;let n=document.getElementById(t);if(n){let e=n.closest(`starlight-tabs [role="tabpanel"]`);if(e){let t=e.closest(`starlight-tabs`);if(t){let n=Array.from(t.querySelectorAll(`:scope > [role="tabpanel"]`)).indexOf(e);n!==-1&&(t.querySelectorAll(`[role="tab"]`)[n]?.click(),requestAnimationFrame(()=>{o()}))}}}})});let i=!1,a=()=>{i=!1;let e=t.filter(e=>e.heading.id===`_top`||e.heading.offsetParent!==null&&(!e.tabpanel||!e.tabpanel.hasAttribute(`hidden`)&&!e.tabpanel.hidden&&e.tabpanel.offsetParent!==null));if(!e.length)return;let r=e[0];for(let t of e)if(t.heading.getBoundingClientRect().top<=130)r=t;else break;let a=n.get(r.heading.id);a&&(this.current=a)},o=()=>{t.forEach(e=>{if(!e.tabpanel){e.li.style.display=``;return}let t=e.tabpanel.hasAttribute(`hidden`)||e.tabpanel.hidden||e.tabpanel.offsetParent===null;e.li.style.display=t?`none`:``}),a()},s=document.querySelectorAll(`[role="tabpanel"]`);if(s.length>0){this._observer=new MutationObserver(()=>{o()}),s.forEach(e=>{this._observer?.observe(e,{attributes:!0,attributeFilter:[`hidden`,`style`,`class`]})});let e=e=>{let t=e.target;(t?.closest(`[role="tab"]`)||t?.closest(`.surface-tag-pill`)||t?.closest(`.tab-button`))&&requestAnimationFrame(()=>{o()})};document.addEventListener(`click`,e),this._cleanup.push(()=>{document.removeEventListener(`click`,e)})}this._onScroll=()=>{i||=(requestAnimationFrame(a),!0)},window.addEventListener(`scroll`,this._onScroll,{passive:!0}),o()}};customElements.get(`starlight-toc`)||customElements.define(`starlight-toc`,e);</script></div></div></div></aside><div class="main-pane astro-abfeecmu"><main data-pagefind-body class="astro-vdxcvpsm" lang="en" dir="ltr"><div class="content-panel astro-oil7k7jf"><div class="sl-container astro-oil7k7jf"><div class="breadcrumbs-row astro-hkwnjoai"><nav aria-label="Breadcrumb" class="starlight-breadcrumbs astro-hkwnjoai"><span class="breadcrumb-item  astro-hkwnjoai" title="Get Started">Get Started</span><svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" class="breadcrumb-separator astro-hkwnjoai" aria-hidden="true"><polyline points="9 18 15 12 9 6" class="astro-hkwnjoai"></polyline></svg><span class="breadcrumb-item  astro-hkwnjoai" title="CLI">CLI</span><svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" class="breadcrumb-separator astro-hkwnjoai" aria-hidden="true"><polyline points="9 18 15 12 9 6" class="astro-hkwnjoai"></polyline></svg><span class="breadcrumb-item breadcrumb-last astro-hkwnjoai" title="Features">Features</span></nav><div id="md-actions-root" class="markdown-dropdown-wrapper astro-pyacvpuh" data-pathname="/docs/cli/features/"><button type="button" class="call-to-action--nav button button-primary button-compact markdown-trigger-btn dropdown-nav" aria-haspopup="true" aria-expanded="false" aria-label="Markdown Options" class="astro-pyacvpuh"><span>Markdown</span><span class="dropdown-icon" translate="no"><svg class viewBox="0 0 24 24" width="18" height="18" fill="currentColor" aria-hidden="true" xmlns="http://www.w3.org/2000/svg"><path d="M12 15L7.05 10.05L8.45 8.65L12 12.2L15.55 8.65L16.95 10.05L12 15Z"></path></svg></span></button><div class="markdown-dropdown-menu astro-pyacvpuh" data-md-menu hidden><button type="button" class="markdown-menu-item astro-pyacvpuh" data-copy-md><svg class="icon astro-pyacvpuh" viewBox="0 -960 960 960" width="16px" height="16px" fill="currentColor" aria-hidden="true" xmlns="http://www.w3.org/2000/svg"><path d="M318.46-230.77q-23.53,0-40.61-17.08t-17.08-40.61V-802.31q0-23.53 17.08-40.61T318.46-860H712.31q23.53,0 40.61,17.08T770-802.31v513.84q0,23.53-17.08,40.61t-40.61,17.08H318.46Zm0-45.38H712.31q4.62,0 8.46-3.85t3.85-8.46V-802.31q0-4.62-3.85-8.46t-8.46-3.85H318.46q-4.62,0-8.46,3.85t-3.85,8.46v513.84q0,4.62 3.85,8.46t8.46,3.85ZM207.69-120q-23.53,0-40.61-17.08T150-177.7V-736.92h45.38V-177.7q0,4.62 3.85,8.46t8.46,3.85H646.92V-120H207.69Zm98.46-156.15q0,0 0-3.85t0-8.46V-802.31q0-4.62 0-8.46t0-3.85q0,0 0,3.85t0,8.46v513.84q0,4.62 0,8.46t0,3.85Z"></path></svg><span class="item-label astro-pyacvpuh">Copy Markdown</span></button><a href="#" data-view-md-link target="_blank" rel="noopener noreferrer" class="markdown-menu-item astro-pyacvpuh"><svg class="icon astro-pyacvpuh" viewBox="0 -960 960 960" width="16px" height="16px" fill="currentColor" aria-hidden="true" xmlns="http://www.w3.org/2000/svg"><path d="M197.69-140q-23.53,0-40.61-17.08T140-197.69V-762.31q0-23.53 17.08-40.61T197.69-820H449.38v45.38H197.69q-4.62,0-8.46,3.85t-3.85,8.46v564.61q0,4.62 3.85,8.46t8.46,3.85H762.31q4.62,0 8.46-3.85t3.85-8.46V-449.38H820v251.69q0,23.53-17.08,40.61T762.31-140H197.69ZM384.31-351.69l-32-32.61L742.62-774.61H530.15V-820H820v289.84H774.61V-742L384.31-351.69Z"></path></svg><span class="item-label astro-pyacvpuh">View Markdown</span></a></div></div><script type="module">function e(){document.querySelectorAll(`.markdown-dropdown-wrapper`).forEach(e=>{let t=e.querySelector(`.markdown-trigger-btn, .button`),n=e.querySelector(`[data-md-menu]`),r=e.querySelector(`[data-copy-md]`),i=e.querySelector(`[data-view-md-link]`),a=`${(e.dataset.pathname||window.location.pathname).replace(/\/$/,``)}.md`;i&&(i.href=a),!(!t||!n)&&(t.addEventListener(`click`,e=>{e.stopPropagation(),n.hasAttribute(`hidden`)?(n.removeAttribute(`hidden`),t.setAttribute(`aria-expanded`,`true`)):(n.setAttribute(`hidden`,``),t.setAttribute(`aria-expanded`,`false`))}),document.addEventListener(`click`,r=>{e.contains(r.target)||(n.setAttribute(`hidden`,``),t.setAttribute(`aria-expanded`,`false`))}),r&&r.addEventListener(`click`,async e=>{e.stopPropagation();let i=r.querySelector(`.item-label`),o=i?.textContent||`Copy Markdown`;try{i&&(i.textContent=`Fetching...`);let e=await fetch(a);if(!e.ok)throw Error(`Failed to fetch markdown: ${e.status}`);let r=await e.text(),s=new DOMParser().parseFromString(r,`text/html`).querySelector(`pre`),c=s?s.textContent||``:r;await navigator.clipboard.writeText(c),i&&(i.textContent=`Copied!`,setTimeout(()=>{i.textContent=o,n.setAttribute(`hidden`,``),t.setAttribute(`aria-expanded`,`false`)},1200))}catch(e){console.error(`Error copying markdown:`,e),i&&(i.textContent=`Error copying`),setTimeout(()=>{i&&(i.textContent=o)},2e3)}}))})}document.readyState===`loading`?document.addEventListener(`DOMContentLoaded`,e):e(),document.addEventListener(`astro:page-load`,e);</script></div><script type="module" src="/_astro/StarlightPageTitleOverride.astro_astro_type_script_index_0_lang.DfB6sZL0.js"></script></div></div><div class="content-panel astro-oil7k7jf"><div class="sl-container astro-oil7k7jf"><div class="sl-markdown-content"><div class="sl-heading-wrapper level-h1"><h1 id="antigravity-cli-features">Antigravity CLI features</h1><a class="sl-anchor-link" href="#antigravity-cli-features"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Antigravity CLI features”</span></a></div>
<div class="sl-heading-wrapper level-h3"><h3 id="plugins">Plugins</h3><a class="sl-anchor-link" href="#plugins"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Plugins”</span></a></div>
<p><strong>How plugins work</strong>
Plugins are namespaced bundles that can contain skills, agents, rules, MCP
servers, and hooks as a single deployable unit.</p>
<p>When you install a plugin, the CLI stages the files in your home directory under <code dir="auto">~/.gemini/antigravity-cli/plugins/&#x3C;plugin_name>/</code>. The Antigravity agent automatically discovers and loads these staged customizations.</p>
<pre class="astro-code astro-code-themes github-light github-dark" style="background-color:#fff;--shiki-dark-bg:#24292e;color:#24292e;--shiki-dark:#e1e4e8; overflow-x: auto; white-space: pre-wrap; word-wrap: break-word;" tabindex="0" data-language="plaintext" dir="ltr"><code><span class="line"><span>~/.gemini/antigravity-cli/</span></span>
<span class="line"><span>├── plugins/</span></span>
<span class="line"><span>│   └── &#x3C;plugin_name>/</span></span>
<span class="line"><span>│       ├── plugin.json         # Required marker file</span></span>
<span class="line"><span>│       ├── mcp_config.json     # Optional MCP server definitions</span></span>
<span class="line"><span>│       ├── hooks.json          # Optional event hooks definition</span></span>
<span class="line"><span>│       ├── skills/             # Optional skills</span></span>
<span class="line"><span>│       ├── agents/             # Optional subagents</span></span>
<span class="line"><span>│       └── rules/              # Optional rules</span></span>
<span class="line"><span>└── import_manifest.json        # Tracking manifest</span></span></code></pre>
<div class="sl-heading-wrapper level-h3"><h3 id="manage-plugins-with-plugin-marketplace-and-installed">Manage plugins with <code dir="auto">/plugin</code> (Marketplace and installed)</h3><a class="sl-anchor-link" href="#manage-plugins-with-plugin-marketplace-and-installed"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Manage plugins with /plugin (Marketplace and installed)”</span></a></div>
<p>Trigger the interactive Plugins Manager in the CLI using the <code dir="auto">/plugin</code> slash command (alias <code dir="auto">/plugins</code>), and press \<kbd\>Tab</kbd> to switch between the <strong>Installed</strong> tab and the <strong>Discover</strong> tab:</p>
\<ul\>
\<li\>
<p><strong>Discover tab</strong>: Browse and install available plugins from the catalog:</p>
<p><img src="/assets/image/docs/plugins/plugin-discover-tab.png" alt="Plugin discover tab"></p>
\<ul\>
\<li\>Type to start searching for specific plugins.</li>
\<li\>Press \<kbd\>Enter</kbd> to select <strong>Install from local directory</strong> and provide a local path for plugin installation.</li>
\<li\>Press \<kbd\>Enter</kbd> on a plugin to expand its details view (includes description, components such as MCP and skills, marketplace, and version).</li>
\<li\>Press \<kbd\>Ctrl</kbd> + \<kbd\>S</kbd> on an uninstalled plugin to install it (or \<kbd\>Ctrl</kbd> + \<kbd\>S</kbd> on an installed plugin to uninstall it).</li>
\<li\><strong>After installation</strong>: A green dot (<code dir="auto">●</code>) appears beside the plugin name in the <strong>Discover</strong> tab denoting its installed status, and the plugin name appears in the <strong>Installed</strong> tab.</li>
</ul>
</li>
\<li\>
<p><strong>Installed tab</strong>: View and configure your installed plugins:</p>
<p><img src="/assets/image/docs/plugins/plugin-install-tab.png" alt="Plugin installed tab"></p>
\<ul\>
\<li\>Type to start searching for installed plugins.</li>
\<li\>Press \<kbd\>Space</kbd> to toggle enabling or disabling a plugin.</li>
\<li\>Press \<kbd\>Enter</kbd> on an installed plugin to expand its details view (includes description, skills if applicable, MCP if applicable, marketplace, local installation path, and version).</li>
</ul>
</li>
\<li\>
<p><strong>Inline commands</strong>: Manage plugins directly from the prompt without opening the interactive panel:</p>
\<ul\>
\<li\><code dir="auto">/plugin</code> provides inline <code dir="auto">install</code>, <code dir="auto">uninstall</code>, <code dir="auto">enable</code>, <code dir="auto">disable</code>, and <code dir="auto">list</code> subcommands without needing to open the interactive Plugins Manager panel.</li>
\<li\>To install inline, run <code dir="auto">/plugin install &#x3C;plugin-name>@&#x3C;marketplace-name></code> (<code dir="auto">&#x3C;marketplace-name></code> supports the official marketplace, <code dir="auto">antigravity-plugins-official</code>) or <code dir="auto">/plugin install &#x3C;local-path></code>. For example, running <code dir="auto">/plugin install firebase@antigravity-plugins-official</code> outputs:
<pre class="astro-code astro-code-themes github-light github-dark" style="background-color:#fff;--shiki-dark-bg:#24292e;color:#24292e;--shiki-dark:#e1e4e8; overflow-x: auto; white-space: pre-wrap; word-wrap: break-word;" tabindex="0" data-language="text" dir="ltr"><code><span class="line"><span>Successfully installed plugin "firebase" from "antigravity-plugins-official".</span></span></code></pre>
</li>
</ul>
</li>
</ul>
<div class="sl-heading-wrapper level-h3"><h3 id="manage-plugins-from-your-shell-agy-plugin">Manage plugins from your shell (<code dir="auto">agy plugin</code>)</h3><a class="sl-anchor-link" href="#manage-plugins-from-your-shell-agy-plugin"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Manage plugins from your shell (agy plugin)”</span></a></div>
<p>Outside an interactive TUI session, you can install and manage plugins from your terminal using <code dir="auto">agy plugin</code>:</p>
\<ul\>
\<li\><strong>Install from the official marketplace</strong>: Specify <code dir="auto">&#x3C;plugin-name>@antigravity-plugins-official</code> (<code dir="auto">&#x3C;marketplace-name></code> only supports the official marketplace, <code dir="auto">antigravity-plugins-official</code>), or pass a bare <code dir="auto">&#x3C;plugin-name></code>, which defaults to <code dir="auto">&#x3C;plugin-name>@antigravity-plugins-official</code>:
<pre class="astro-code astro-code-themes github-light github-dark" style="background-color:#fff;--shiki-dark-bg:#24292e;color:#24292e;--shiki-dark:#e1e4e8; overflow-x: auto; white-space: pre-wrap; word-wrap: break-word;" tabindex="0" data-language="bash" dir="ltr"><code><span class="line"><span style="color:#6F42C1;--shiki-dark:#B392F0">agy</span><span style="color:#032F62;--shiki-dark:#9ECBFF"> plugin</span><span style="color:#032F62;--shiki-dark:#9ECBFF"> install</span><span style="color:#D73A49;--shiki-dark:#F97583"> &#x3C;</span><span style="color:#032F62;--shiki-dark:#9ECBFF">plugin-nam</span><span style="color:#24292E;--shiki-dark:#E1E4E8">e</span><span style="color:#D73A49;--shiki-dark:#F97583">></span><span style="color:#032F62;--shiki-dark:#9ECBFF">@antigravity-plugins-official</span></span>
<span class="line"><span style="color:#6F42C1;--shiki-dark:#B392F0">agy</span><span style="color:#032F62;--shiki-dark:#9ECBFF"> plugin</span><span style="color:#032F62;--shiki-dark:#9ECBFF"> install</span><span style="color:#D73A49;--shiki-dark:#F97583"> &#x3C;</span><span style="color:#032F62;--shiki-dark:#9ECBFF">plugin-nam</span><span style="color:#24292E;--shiki-dark:#E1E4E8">e</span><span style="color:#D73A49;--shiki-dark:#F97583">></span></span></code></pre>
</li>
\<li\><strong>Install from a GitHub link</strong>: Clone and stage a plugin directly from a GitHub repository URL:
<pre class="astro-code astro-code-themes github-light github-dark" style="background-color:#fff;--shiki-dark-bg:#24292e;color:#24292e;--shiki-dark:#e1e4e8; overflow-x: auto; white-space: pre-wrap; word-wrap: break-word;" tabindex="0" data-language="bash" dir="ltr"><code><span class="line"><span style="color:#6F42C1;--shiki-dark:#B392F0">agy</span><span style="color:#032F62;--shiki-dark:#9ECBFF"> plugin</span><span style="color:#032F62;--shiki-dark:#9ECBFF"> install</span><span style="color:#032F62;--shiki-dark:#9ECBFF"> https://github.com/</span><span style="color:#D73A49;--shiki-dark:#F97583">&#x3C;</span><span style="color:#032F62;--shiki-dark:#9ECBFF">owne</span><span style="color:#24292E;--shiki-dark:#E1E4E8">r</span><span style="color:#D73A49;--shiki-dark:#F97583">></span><span style="color:#032F62;--shiki-dark:#9ECBFF">/</span><span style="color:#D73A49;--shiki-dark:#F97583">&#x3C;</span><span style="color:#032F62;--shiki-dark:#9ECBFF">rep</span><span style="color:#24292E;--shiki-dark:#E1E4E8">o</span><span style="color:#D73A49;--shiki-dark:#F97583">></span></span></code></pre>
</li>
\<li\><strong>Install from a local directory</strong>: Stage a local plugin package into your profile:
<pre class="astro-code astro-code-themes github-light github-dark" style="background-color:#fff;--shiki-dark-bg:#24292e;color:#24292e;--shiki-dark:#e1e4e8; overflow-x: auto; white-space: pre-wrap; word-wrap: break-word;" tabindex="0" data-language="bash" dir="ltr"><code><span class="line"><span style="color:#6F42C1;--shiki-dark:#B392F0">agy</span><span style="color:#032F62;--shiki-dark:#9ECBFF"> plugin</span><span style="color:#032F62;--shiki-dark:#9ECBFF"> install</span><span style="color:#D73A49;--shiki-dark:#F97583"> &#x3C;</span><span style="color:#032F62;--shiki-dark:#9ECBFF">/path/to/local/plugi</span><span style="color:#24292E;--shiki-dark:#E1E4E8">n</span><span style="color:#D73A49;--shiki-dark:#F97583">></span></span></code></pre>
</li>
\<li\><strong>List, enable, disable, or uninstall plugins</strong>:
<pre class="astro-code astro-code-themes github-light github-dark" style="background-color:#fff;--shiki-dark-bg:#24292e;color:#24292e;--shiki-dark:#e1e4e8; overflow-x: auto; white-space: pre-wrap; word-wrap: break-word;" tabindex="0" data-language="bash" dir="ltr"><code><span class="line"><span style="color:#6F42C1;--shiki-dark:#B392F0">agy</span><span style="color:#032F62;--shiki-dark:#9ECBFF"> plugin</span><span style="color:#032F62;--shiki-dark:#9ECBFF"> list</span></span>
<span class="line"><span style="color:#6F42C1;--shiki-dark:#B392F0">agy</span><span style="color:#032F62;--shiki-dark:#9ECBFF"> plugin</span><span style="color:#032F62;--shiki-dark:#9ECBFF"> enable</span><span style="color:#D73A49;--shiki-dark:#F97583"> &#x3C;</span><span style="color:#032F62;--shiki-dark:#9ECBFF">plugin-nam</span><span style="color:#24292E;--shiki-dark:#E1E4E8">e</span><span style="color:#D73A49;--shiki-dark:#F97583">></span></span>
<span class="line"><span style="color:#6F42C1;--shiki-dark:#B392F0">agy</span><span style="color:#032F62;--shiki-dark:#9ECBFF"> plugin</span><span style="color:#032F62;--shiki-dark:#9ECBFF"> disable</span><span style="color:#D73A49;--shiki-dark:#F97583"> &#x3C;</span><span style="color:#032F62;--shiki-dark:#9ECBFF">plugin-nam</span><span style="color:#24292E;--shiki-dark:#E1E4E8">e</span><span style="color:#D73A49;--shiki-dark:#F97583">></span></span>
<span class="line"><span style="color:#6F42C1;--shiki-dark:#B392F0">agy</span><span style="color:#032F62;--shiki-dark:#9ECBFF"> plugin</span><span style="color:#032F62;--shiki-dark:#9ECBFF"> uninstall</span><span style="color:#D73A49;--shiki-dark:#F97583"> &#x3C;</span><span style="color:#032F62;--shiki-dark:#9ECBFF">plugin-nam</span><span style="color:#24292E;--shiki-dark:#E1E4E8">e</span><span style="color:#D73A49;--shiki-dark:#F97583">></span></span></code></pre>
</li>
</ul>
<aside aria-label="Cross-surface synchronization" class="starlight-aside starlight-aside--note"><p class="starlight-aside__title" aria-hidden="true"><svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor" class="starlight-aside__icon"><path d="M12 11a1 1 0 0 0-1 1v4a1 1 0 0 0 2 0v-4a1 1 0 0 0-1-1Zm.38-3.92a1 1 0 0 0-.76 0 1 1 0 0 0-.33.21 1.15 1.15 0 0 0-.21.33 1 1 0 0 0 .21 1.09c.097.088.209.16.33.21A1 1 0 0 0 13 8a1.05 1.05 0 0 0-.29-.71 1 1 0 0 0-.33-.21ZM12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20Zm0 18a8 8 0 1 1 0-16.001A8 8 0 0 1 12 20Z"></path></svg>Cross-surface synchronization</p><div class="starlight-aside__content"><p>Plugins installed in Antigravity 2.0 are automatically updated and shown in the CLI’s <strong>Installed</strong> tab. For more details, refer to the <a href="/docs/marketplace?tab=cli">Marketplace guide</a>.</p></div></aside>
<p><strong>Accessing plugin components</strong>
Once staged and loaded, you can interact with the plugin components inside the
CLI using slash commands.</p>
<div class="sl-heading-wrapper level-h3"><h3 id="terminal-sandbox">Terminal sandbox</h3><a class="sl-anchor-link" href="#terminal-sandbox"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Terminal sandbox”</span></a></div>
<p>The terminal sandbox is a lightweight security isolation mechanism that protects your host system from potentially destructive file manipulations or unauthorized outbound network requests when the agent executes local shell commands.</p>
<p>Rather than running heavy virtual machines or containers, the CLI uses native operating system features (<code dir="auto">nsjail</code> on Linux, <code dir="auto">sandbox-exec</code> on macOS, and <code dir="auto">AppContainer</code> on Windows) to enforce strict containment boundaries with zero startup overhead.</p>
<p><strong>Configuration</strong>
You can configure the sandbox behavior in your <code dir="auto">settings.json</code> file (located at <code dir="auto">~/.gemini/antigravity-cli/settings.json</code>):</p>
<pre class="astro-code astro-code-themes github-light github-dark" style="background-color:#fff;--shiki-dark-bg:#24292e;color:#24292e;--shiki-dark:#e1e4e8; overflow-x: auto; white-space: pre-wrap; word-wrap: break-word;" tabindex="0" data-language="json" dir="ltr"><code><span class="line"><span style="color:#24292E;--shiki-dark:#E1E4E8">{</span></span>
<span class="line"><span style="color:#005CC5;--shiki-dark:#79B8FF">    "enableTerminalSandbox"</span><span style="color:#24292E;--shiki-dark:#E1E4E8">: </span><span style="color:#005CC5;--shiki-dark:#79B8FF">true</span></span>
<span class="line"><span style="color:#24292E;--shiki-dark:#E1E4E8">}</span></span></code></pre>
<p>The configuration supports the following option:</p>
\<ul\>
\<li\><strong><code dir="auto">enableTerminalSandbox</code></strong> (boolean, default: <code dir="auto">false</code>): Enables general execution containment barriers on all local agent processes.</li>
</ul>
<p><strong>Interactive approvals</strong>
When the agent proposes a terminal command that requires your confirmation, the
CLI prompt adapts dynamically based on your settings:</p>
\<ul\>
\<li\><strong>When the sandbox is enabled</strong>: The confirmation prompt includes a specific option to <strong>Yes, and run without sandbox restrictions</strong> if you need to temporarily bypass the containment boundary for a single trusted command.</li>
\<li\><strong>When the sandbox is disabled</strong>: The prompt includes an option to <strong>Yes, and run in sandbox</strong> if you want to force a specific, potentially risky command to execute within the safety boundary.</li>
</ul>
<div class="sl-heading-wrapper level-h3"><h3 id="cli-slash-commands-reference">CLI slash commands reference</h3><a class="sl-anchor-link" href="#cli-slash-commands-reference"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “CLI slash commands reference”</span></a></div>
<p>The Antigravity CLI supports a variety of slash commands typed directly into the prompt box to manage conversations, configure settings, and inspect agent capabilities.</p>
<div class="sl-heading-wrapper level-h3"><h3 id="core-slash-commands">Core slash commands</h3><a class="sl-anchor-link" href="#core-slash-commands"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Core slash commands”</span></a></div>




































































































<table><thead><tr><th align="left">Command</th><th align="left">Category</th><th align="left">Purpose</th></tr></thead>\<tbody\><tr><td align="left"><strong><code dir="auto">/resume</code></strong> <em>(alias <code dir="auto">/switch</code>)</em></td><td align="left">Conversation</td><td align="left">Open the conversation picker to resume or switch sessions.</td></tr><tr><td align="left"><strong><a href="/docs/boost"><code dir="auto">/boost &#x3C;task></code></a></strong></td><td align="left">Reasoning</td><td align="left">Multi-agent deep reasoning for complex bugs, race conditions, and algorithms.</td></tr><tr><td align="left"><strong><a href="/docs/teamwork"><code dir="auto">/teamwork-preview &#x3C;task></code></a></strong></td><td align="left">Reasoning</td><td align="left">Launch <a href="/docs/teamwork">collaborative multi-agent teams</a> for long-horizon projects (paid plans).</td></tr><tr><td align="left"><strong><code dir="auto">/rewind</code></strong> <em>(alias <code dir="auto">/undo</code>)</em></td><td align="left">Conversation</td><td align="left">Roll back conversation history to a previous checkpoint.</td></tr><tr><td align="left"><strong><code dir="auto">/rename &#x3C;name></code></strong></td><td align="left">Conversation</td><td align="left">Rename the active conversation thread for easier tracking.</td></tr><tr><td align="left"><strong><code dir="auto">/permissions</code></strong></td><td align="left">Configuration</td><td align="left">Select agent autonomy level (<code dir="auto">request-review</code>, <code dir="auto">always-proceed</code>, or <code dir="auto">strict</code>).</td></tr><tr><td align="left"><strong><code dir="auto">/model</code></strong></td><td align="left">Configuration</td><td align="left">Select the default reasoning model (persists across sessions).</td></tr><tr><td align="left"><strong><code dir="auto">/keybindings</code></strong></td><td align="left">Configuration</td><td align="left">Open the interactive keyboard shortcut editor.</td></tr><tr><td align="left"><strong><code dir="auto">/statusline</code></strong></td><td align="left">Configuration</td><td align="left">Customize real-time indicators displayed in the CLI status bar.</td></tr><tr><td align="left"><strong><code dir="auto">/tasks</code></strong></td><td align="left">Tools &#x26; Monitoring</td><td align="left">Monitor, view logs for, or terminate active background tasks.</td></tr><tr><td align="left"><strong><code dir="auto">/plugin</code></strong> <em>(alias <code dir="auto">/plugins</code>)</em></td><td align="left">Tools &#x26; Monitoring</td><td align="left">Open the interactive <a href="/docs/marketplace?tab=cli">Plugins Manager</a> to discover, install, enable, disable, or uninstall plugins.</td></tr><tr><td align="left"><strong><code dir="auto">/skills</code></strong></td><td align="left">Tools &#x26; Monitoring</td><td align="left">Browse local and global encapsulated agent workflows.</td></tr><tr><td align="left"><strong><code dir="auto">/mcp</code></strong></td><td align="left">Tools &#x26; Monitoring</td><td align="left">Open the panel to configure and manage Model Context Protocol servers.</td></tr><tr><td align="left"><strong><code dir="auto">/open &#x3C;path></code></strong></td><td align="left">Utility</td><td align="left">Immediately open a file in your preferred external editor.</td></tr><tr><td align="left"><strong><code dir="auto">/diff</code></strong></td><td align="left">Utility</td><td align="left">Open the <a href="/docs/cli/commands/diff">interactive diff viewer</a> to review changes and steer the agent.</td></tr><tr><td align="left"><strong><a href="/docs/remote-control?tab=cli#interactive-mode"><code dir="auto">/remote-control</code></a></strong></td><td align="left">Utility</td><td align="left">Turn on or off Remote Control for the active terminal session.</td></tr><tr><td align="left"><strong><code dir="auto">/usage</code></strong></td><td align="left">Utility</td><td align="left">Open the inline interactive help manual inside the terminal.</td></tr><tr><td align="left"><strong><code dir="auto">/logout</code></strong></td><td align="left">Account</td><td align="left">Log out of your Google session and clear cached credentials.</td></tr></tbody></table>
<div class="sl-heading-wrapper level-h3"><h3 id="advanced-customization-using-settingsjson">Advanced customization using <code dir="auto">settings.json</code></h3><a class="sl-anchor-link" href="#advanced-customization-using-settingsjson"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Advanced customization using settings.json”</span></a></div>
<p>For power users, several slash commands support deep customization using your <code dir="auto">~/.gemini/antigravity-cli/settings.json</code> configuration:</p>
\<ul\>
\<li\><strong>Fine-grained permissions</strong>: Instead of global levels, define specific allowed or denied commands:
<pre class="astro-code astro-code-themes github-light github-dark" style="background-color:#fff;--shiki-dark-bg:#24292e;color:#24292e;--shiki-dark:#e1e4e8; overflow-x: auto; white-space: pre-wrap; word-wrap: break-word;" tabindex="0" data-language="json" dir="ltr"><code><span class="line"><span style="color:#032F62;--shiki-dark:#9ECBFF">"permissions"</span><span style="color:#24292E;--shiki-dark:#E1E4E8">: {</span></span>
<span class="line"><span style="color:#005CC5;--shiki-dark:#79B8FF">  "allow"</span><span style="color:#24292E;--shiki-dark:#E1E4E8">: [</span><span style="color:#032F62;--shiki-dark:#9ECBFF">"command(git)"</span><span style="color:#24292E;--shiki-dark:#E1E4E8">, </span><span style="color:#032F62;--shiki-dark:#9ECBFF">"command(npm test)"</span><span style="color:#24292E;--shiki-dark:#E1E4E8">],</span></span>
<span class="line"><span style="color:#005CC5;--shiki-dark:#79B8FF">  "deny"</span><span style="color:#24292E;--shiki-dark:#E1E4E8">: [</span><span style="color:#032F62;--shiki-dark:#9ECBFF">"command(rm -rf)"</span><span style="color:#24292E;--shiki-dark:#E1E4E8">]</span></span>
<span class="line"><span style="color:#24292E;--shiki-dark:#E1E4E8">}</span></span></code></pre>
</li>
\<li\><strong>Custom status line and window titles</strong>: You can pipe live agent metadata (JSON format containing the current working directory, active model, token usage, state, and more) directly into your own custom shell scripts to generate dynamic status bars or terminal window titles.</li>
</ul>
<div class="sl-heading-wrapper level-h3"><h3 id="subagents-in-antigravity-cli">Subagents in Antigravity CLI</h3><a class="sl-anchor-link" href="#subagents-in-antigravity-cli"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Subagents in Antigravity CLI”</span></a></div>
<p>Antigravity CLI features an asynchronous subagents framework that allows the main agent to delegate parallel work, perform background research, and run system tests without blocking your active conversation.</p>
<p><strong>What are subagents?</strong>
Subagents are independent, concurrent agent sessions designed to tackle specific
background tasks in parallel with the main conversation:</p>
\<ul\>
\<li\><strong>Purpose</strong>: The main agent automatically spawns subagents to perform background operations such as looking up documentation, running builds, or validating a fix.</li>
\<li\><strong>Capabilities</strong>: Subagents have full access to tools such as code search, file editing, terminal commands, and web searches to complete their assigned tasks. The main agent decides what tools and permissions subagents get, including whether they can use MCP tools and whether they can write files.</li>
</ul>
<div class="sl-heading-wrapper level-h3"><h3 id="managing-agents-the-agents-panel">Managing agents: the <code dir="auto">/agents</code> panel</h3><a class="sl-anchor-link" href="#managing-agents-the-agents-panel"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Managing agents: the /agents panel”</span></a></div>
<p>Antigravity CLI provides an interactive terminal UI to view, manage, and approve actions for running subagents:</p>
\<ul\>
\<li\><strong>Access</strong>: Type <code dir="auto">/agents</code> in the prompt to open the subagents panel.</li>
\<li\><strong>Overview</strong>: The panel shows a list of active and completed subagents, including surface-level details such as their status (such as running, done, or killed) and the current step they’re executing.</li>
</ul>
<aside aria-label="Note" class="starlight-aside starlight-aside--note"><p class="starlight-aside__title" aria-hidden="true"><svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor" class="starlight-aside__icon"><path d="M12 11a1 1 0 0 0-1 1v4a1 1 0 0 0 2 0v-4a1 1 0 0 0-1-1Zm.38-3.92a1 1 0 0 0-.76 0 1 1 0 0 0-.33.21 1.15 1.15 0 0 0-.21.33 1 1 0 0 0 .21 1.09c.097.088.209.16.33.21A1 1 0 0 0 13 8a1.05 1.05 0 0 0-.29-.71 1 1 0 0 0-.33-.21ZM12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20Zm0 18a8 8 0 1 1 0-16.001A8 8 0 0 1 12 20Z"></path></svg>Note</p><div class="starlight-aside__content"><p>Selecting a subagent from the panel opens a full-screen detail view. This view shows the entirety of the subagent’s conversation, including its steps, thoughts, and tool execution logs.</p></div></aside>
<p><strong>Tool confirmations and approvals</strong>
When a subagent needs to execute a tool that requires your permission (such as
running a local command or writing a file), it surfaces the request. You can
manage approvals in two ways:</p>
\<ol\>
\<li\><strong>Detail view approvals</strong>: The subagent detail view features an interaction
section containing all pending approvals, where you can selectively approve
or deny requests.</li>
</ol>
<aside aria-label="Note" class="starlight-aside starlight-aside--note"><p class="starlight-aside__title" aria-hidden="true"><svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor" class="starlight-aside__icon"><path d="M12 11a1 1 0 0 0-1 1v4a1 1 0 0 0 2 0v-4a1 1 0 0 0-1-1Zm.38-3.92a1 1 0 0 0-.76 0 1 1 0 0 0-.33.21 1.15 1.15 0 0 0-.21.33 1 1 0 0 0 .21 1.09c.097.088.209.16.33.21A1 1 0 0 0 13 8a1.05 1.05 0 0 0-.29-.71 1 1 0 0 0-.33-.21ZM12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20Zm0 18a8 8 0 1 1 0-16.001A8 8 0 0 1 12 20Z"></path></svg>Note</p><div class="starlight-aside__content"><p><strong>Tip</strong>: Use the keyboard shortcut <code dir="auto">ctrl+j</code> to jump from the main conversation directly to the detailed view of the next subagent waiting for your approval.</p></div></aside>
<ol start="2">
\<li\><strong>Fast-path alerts</strong>: To keep you in your flow, Antigravity CLI displays a
fast-path alert directly above your prompt box when a subagent requests
permission.</li>
</ol>
<aside aria-label="Note" class="starlight-aside starlight-aside--note"><p class="starlight-aside__title" aria-hidden="true"><svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor" class="starlight-aside__icon"><path d="M12 11a1 1 0 0 0-1 1v4a1 1 0 0 0 2 0v-4a1 1 0 0 0-1-1Zm.38-3.92a1 1 0 0 0-.76 0 1 1 0 0 0-.33.21 1.15 1.15 0 0 0-.21.33 1 1 0 0 0 .21 1.09c.097.088.209.16.33.21A1 1 0 0 0 13 8a1.05 1.05 0 0 0-.29-.71 1 1 0 0 0-.33-.21ZM12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20Zm0 18a8 8 0 1 1 0-16.001A8 8 0 0 1 12 20Z"></path></svg>Note</p><div class="starlight-aside__content"><p><strong>Tip</strong>: You can approve a pending subagent permission instantly using <code dir="auto">ctrl+k</code> without ever having to switch away from the main conversation.</p></div></aside></div><footer class="sl-flex docs-footer astro-4mgpquaz"><div class="meta sl-flex astro-4mgpquaz"></div><div class="pagination-links print:hidden astro-jgq3vlii" dir="ltr"><a href="/docs/cli/using/" rel="prev" class="astro-jgq3vlii"><svg aria-hidden="true" class="astro-jgq3vlii astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.5rem;"><path d="M17 11H9.41l3.3-3.29a1.004 1.004 0 1 0-1.42-1.42l-5 5a1 1 0 0 0-.21.33 1 1 0 0 0 0 .76 1 1 0 0 0 .21.33l5 5a1.002 1.002 0 0 0 1.639-.325 1 1 0 0 0-.219-1.095L9.41 13H17a1 1 0 0 0 0-2Z"/></svg><span class="astro-jgq3vlii">Previous<br class="astro-jgq3vlii"><span class="link-title astro-jgq3vlii">Using Antigravity CLI</span></span></a><a href="/docs/cli/best-practices/" rel="next" class="astro-jgq3vlii"><svg aria-hidden="true" class="astro-jgq3vlii astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.5rem;"><path d="M17.92 11.62a1.001 1.001 0 0 0-.21-.33l-5-5a1.003 1.003 0 1 0-1.42 1.42l3.3 3.29H7a1 1 0 0 0 0 2h7.59l-3.3 3.29a1.002 1.002 0 0 0 .325 1.639 1 1 0 0 0 1.095-.219l5-5a1 1 0 0 0 .21-.33 1 1 0 0 0 0-.76Z"/></svg><span class="astro-jgq3vlii">Next<br class="astro-jgq3vlii"><span class="link-title astro-jgq3vlii">Best practices</span></span></a></div><nav class="docs-legal-nav print:hidden astro-4mgpquaz" data-analytics-nav="docs_footer" aria-label="Google legal and cookie links"><a class="docs-footer-link astro-4mgpquaz" href="https://about.google/" target="_blank" rel="noopener noreferrer">About Google</a><a class="docs-footer-link astro-4mgpquaz" href="https://about.google/products/" target="_blank" rel="noopener noreferrer">Google Products</a><a class="docs-footer-link astro-4mgpquaz" href="https://policies.google.com/privacy?hl=en-US" target="_blank" rel="noopener noreferrer">Privacy</a><a class="docs-footer-link astro-4mgpquaz" href="/terms">Terms</a><a aria-hidden="true" class="docs-footer-link glue-cookie-notification-bar-control astro-4mgpquaz">Manage cookies</a></nav></footer></div></div></main></div></div></div></div></body></html>
