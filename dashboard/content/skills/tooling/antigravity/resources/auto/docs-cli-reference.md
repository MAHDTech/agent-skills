+++
title = "docs-cli-reference"
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
</script><meta charset="utf-8"/><meta name="viewport" content="width=device-width, initial-scale=1"/>\<title\>CLI reference - Antigravity CLI | Google Antigravity Docs</title><link rel="canonical" href="https://antigravity.google/docs/cli/reference/"/><link rel="sitemap" href="/sitemap-index.xml"/><link rel="icon" type="image/x-icon" href="/favicon.ico" sizes="48x48"/><link rel="icon" type="image/png" href="/apple-touch-icon.png" sizes="180x180"/><link rel="icon" type="image/png" href="/assets/image/antigravity-logo.png" sizes="276x276"/><link rel="apple-touch-icon" href="/apple-touch-icon.png" sizes="180x180"/><link rel="apple-touch-icon-precomposed" href="/apple-touch-icon-precomposed.png" sizes="180x180"/><link rel="preconnect" href="https://fonts.googleapis.com"/><link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/><link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Google+Symbols:opsz,wght,FILL,GRAD,ROND@20..48,100..700,0..1,-50..200,0..100&display=block"/><link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Google+Sans+Flex:opsz,slnt,wdth,wght,ROND@8..144,-10..0,25..150,400..500,0..100&display=swap"/><link rel="shortcut icon" href="/favicon.svg" type="image/svg+xml"/><meta name="generator" content="Astro v7.2.9"/><meta name="generator" content="Starlight v0.41.5"/><meta property="og:title" content="CLI reference - Antigravity CLI | Google Antigravity Docs"/><meta property="og:type" content="article"/><meta property="og:url" content="https://antigravity.google/docs/cli/reference/"/><meta property="og:locale" content="en"/><meta property="og:site_name" content="Google Antigravity Docs"/><meta property="og:image" content="https://antigravity.google/assets/image/sitecards/sitecard-documentation.png"/><meta property="og:image:secure_url" content="https://antigravity.google/assets/image/sitecards/sitecard-documentation.png"/><meta name="twitter:image" content="https://antigravity.google/assets/image/sitecards/sitecard-documentation.png"/><meta name="twitter:card" content="summary_large_image"/><meta name="description" content="Review tables listing all TUI slash commands, default keyboard shortcuts, and JSON configuration parameters."/><meta property="og:description" content="Review tables listing all TUI slash commands, default keyboard shortcuts, and JSON configuration parameters."/><meta name="twitter:title" content="CLI reference - Antigravity CLI | Google Antigravity Docs"/><meta name="twitter:description" content="Review tables listing all TUI slash commands, default keyboard shortcuts, and JSON configuration parameters."/><script type="application/ld+json">{"@context":"https://schema.org","@graph":[{"@type":"TechArticle","headline":"CLI reference - Antigravity CLI","description":"Review tables listing all TUI slash commands, default keyboard shortcuts, and JSON configuration parameters.","url":"https://antigravity.google/docs/cli/reference/","mainEntityOfPage":{"@type":"WebPage","@id":"https://antigravity.google/docs/cli/reference/"},"author":{"@type":"Organization","name":"Google Antigravity","url":"https://antigravity.google/"},"publisher":{"@type":"Organization","name":"Google","url":"https://www.google.com/"}},{"@type":"BreadcrumbList","itemListElement":[{"@type":"ListItem","position":1,"name":"Docs","item":"https://antigravity.google/docs/home/"},{"@type":"ListItem","position":2,"name":"Antigravity CLI","item":"https://antigravity.google/docs/cli/overview/"},{"@type":"ListItem","position":3,"name":"CLI reference","item":"https://antigravity.google/docs/cli/reference/"}]}]}</script><link href="https://www.gstatic.com/glue/cookienotificationbar/cookienotificationbar.min.css" rel="stylesheet">\<script\>
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
	</script><div class="sidebar-sections is-active astro-s3sbtxwf" data-sidebar-section="get-started"><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">GET STARTED</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/home/" class="astro-ojxuodky"><span class="astro-ojxuodky">Home</span></a></li><li class="astro-ojxuodky"><a href="/docs/getting-started/" class="astro-ojxuodky"><span class="astro-ojxuodky">Getting started</span></a></li></ul></section><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">ANTIGRAVITY 2.0</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/overview/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/build-with-google/" class="astro-ojxuodky"><span class="astro-ojxuodky">Build with Google</span></a></li><li class="astro-ojxuodky"><a href="/docs/models/" class="astro-ojxuodky"><span class="astro-ojxuodky">Models</span></a></li></ul></section><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">CLI</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/cli/overview/" class="astro-ojxuodky"><span class="astro-ojxuodky">CLI overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/install/" class="astro-ojxuodky"><span class="astro-ojxuodky">Installation and auth</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/tutorial/" class="astro-ojxuodky"><span class="astro-ojxuodky">Tutorial</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/using/" class="astro-ojxuodky"><span class="astro-ojxuodky">Using Antigravity CLI</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/features/" class="astro-ojxuodky"><span class="astro-ojxuodky">Features</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/best-practices/" class="astro-ojxuodky"><span class="astro-ojxuodky">Best practices</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/troubleshooting/" class="astro-ojxuodky"><span class="astro-ojxuodky">Troubleshooting</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/reference/" aria-current="page" class="astro-ojxuodky"><span class="astro-ojxuodky">CLI reference</span></a></li></ul></section><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">ANTIGRAVITY FOR IDES</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/ide/overview/" class="astro-ojxuodky"><span class="astro-ojxuodky">Antigravity IDE</span></a></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">IDE extensions</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="0"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/ide/extensions/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/extensions/vscode/" class="astro-ojxuodky"><span class="astro-ojxuodky">Visual Studio Code</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/extensions/visual-studio/" class="astro-ojxuodky"><span class="astro-ojxuodky">Visual Studio</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/extensions/jetbrains/" class="astro-ojxuodky"><span class="astro-ojxuodky">JetBrains</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/extensions/zed/" class="astro-ojxuodky"><span class="astro-ojxuodky">Zed</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/extensions/xcode/" class="astro-ojxuodky"><span class="astro-ojxuodky">Xcode</span></a></li></ul></details></li></ul></section><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">RESOURCES</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/faq/" class="astro-ojxuodky"><span class="astro-ojxuodky">FAQ</span></a></li><li class="astro-ojxuodky"><a href="/docs/plans/" class="astro-ojxuodky"><span class="astro-ojxuodky">Plans and AI credits</span></a></li><li class="astro-ojxuodky"><a href="/docs/enterprise/" class="astro-ojxuodky"><span class="astro-ojxuodky">Enterprise overview</span></a></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Migrations</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="1"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/firebase-studio-migration/" class="astro-ojxuodky"><span class="astro-ojxuodky">Firebase Studio migration</span></a></li><li class="astro-ojxuodky"><a href="/docs/migration/workflows-to-skills/" class="astro-ojxuodky"><span class="astro-ojxuodky">Workflows to skills migration</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/gcli-migration/" class="astro-ojxuodky"><span class="astro-ojxuodky">GCLI migration</span></a></li></ul></details></li></ul></section></div><div class="sidebar-sections astro-s3sbtxwf" data-sidebar-section="features"><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">LEARN</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/features/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/remote-control/" class="astro-ojxuodky"><span class="astro-ojxuodky">Remote Control</span></a></li><li class="astro-ojxuodky"><a href="/docs/marketplace/" class="astro-ojxuodky"><span class="astro-ojxuodky">Marketplace</span></a></li><li class="astro-ojxuodky"><a href="/docs/plugins/" class="astro-ojxuodky"><span class="astro-ojxuodky">Plugins</span></a></li><li class="astro-ojxuodky"><a href="/docs/sandbox/" class="astro-ojxuodky"><span class="astro-ojxuodky">Terminal sandbox</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/review-changes-editor/" class="astro-ojxuodky"><span class="astro-ojxuodky">Review changes</span></a></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Terminal</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="2"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/cli/modes/" class="astro-ojxuodky"><span class="astro-ojxuodky">Execution modes</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/headless/" class="astro-ojxuodky"><span class="astro-ojxuodky">Headless mode</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/statusline/" class="astro-ojxuodky"><span class="astro-ojxuodky">Status line customization</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/title/" class="astro-ojxuodky"><span class="astro-ojxuodky">Terminal title customization</span></a></li></ul></details></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Browser</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="3"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/ide/browser/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/browser-recordings/" class="astro-ojxuodky"><span class="astro-ojxuodky">Browser recordings</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/allowlist-denylist/" class="astro-ojxuodky"><span class="astro-ojxuodky">Allowlist and denylist</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/separate-chrome-profile/" class="astro-ojxuodky"><span class="astro-ojxuodky">Separate Chrome profile</span></a></li></ul></details></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Artifacts</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="4"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/artifacts/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/implementation-plan/" class="astro-ojxuodky"><span class="astro-ojxuodky">Implementation plan</span></a></li><li class="astro-ojxuodky"><a href="/docs/walkthrough/" class="astro-ojxuodky"><span class="astro-ojxuodky">Walkthrough</span></a></li><li class="astro-ojxuodky"><a href="/docs/screenshots/" class="astro-ojxuodky"><span class="astro-ojxuodky">Visual screenshots</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/artifacts/" class="astro-ojxuodky"><span class="astro-ojxuodky">Reviewing artifacts (CLI)</span></a></li></ul></details></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Projects</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="5"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/projects/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/prompting/" class="astro-ojxuodky"><span class="astro-ojxuodky">Prompting and interaction</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/conversations/" class="astro-ojxuodky"><span class="astro-ojxuodky">Managing conversations</span></a></li></ul></details></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Permissions</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="6"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/permissions/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/credits/" class="astro-ojxuodky"><span class="astro-ojxuodky">AI credits</span></a></li></ul></details></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Editor features</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="7"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/ide/tab/" class="astro-ojxuodky"><span class="astro-ojxuodky">Tab completion</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/agent-side-panel/" class="astro-ojxuodky"><span class="astro-ojxuodky">Agent side panel</span></a></li><li class="astro-ojxuodky"><a href="/docs/agent/" class="astro-ojxuodky"><span class="astro-ojxuodky">Agent overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/tools/" class="astro-ojxuodky"><span class="astro-ojxuodky">Task groups</span></a></li></ul></details></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Settings</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="8"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/settings/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/agent-settings/" class="astro-ojxuodky"><span class="astro-ojxuodky">Agent settings</span></a></li><li class="astro-ojxuodky"><a href="/docs/artifact-review/" class="astro-ojxuodky"><span class="astro-ojxuodky">Artifact review</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/vim-editor-mode/" class="astro-ojxuodky"><span class="astro-ojxuodky">Vim editor mode</span></a></li></ul></details></li></ul></section><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">SLASH COMMANDS</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/slash-commands/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/boost/" class="astro-ojxuodky"><span class="astro-ojxuodky">Boost (/boost)</span></a></li><li class="astro-ojxuodky"><a href="/docs/plan/" class="astro-ojxuodky"><span class="astro-ojxuodky">Plan (/plan)</span></a></li><li class="astro-ojxuodky"><a href="/docs/teamwork/" class="astro-ojxuodky"><span class="astro-ojxuodky">Teamwork (/teamwork-preview)</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/agents/" class="astro-ojxuodky"><span class="astro-ojxuodky">Agents (/agents)</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/codesearch/" class="astro-ojxuodky"><span class="astro-ojxuodky">Code search (/codesearch)</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/credits/" class="astro-ojxuodky"><span class="astro-ojxuodky">AI credits (/credits)</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/diff/" class="astro-ojxuodky"><span class="astro-ojxuodky">Diff (/diff)</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/permissions/" class="astro-ojxuodky"><span class="astro-ojxuodky">Permissions (/permissions)</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/resume/" class="astro-ojxuodky"><span class="astro-ojxuodky">Resume (/resume)</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/statusline/" class="astro-ojxuodky"><span class="astro-ojxuodky">Status line (/statusline)</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/title/" class="astro-ojxuodky"><span class="astro-ojxuodky">Window title (/title)</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/usage/" class="astro-ojxuodky"><span class="astro-ojxuodky">Model quotas (/usage)</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/voice/" class="astro-ojxuodky"><span class="astro-ojxuodky">Voice dictation (/voice)</span></a></li></ul></section><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">EXTEND</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/mcp/" class="astro-ojxuodky"><span class="astro-ojxuodky">MCP</span></a></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Skills</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="9"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/skills/" class="astro-ojxuodky"><span class="astro-ojxuodky">Agent skills</span></a></li><li class="astro-ojxuodky"><a href="/docs/rules/" class="astro-ojxuodky"><span class="astro-ojxuodky">Rules</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/workflows/" class="astro-ojxuodky"><span class="astro-ojxuodky">IDE workflows</span></a></li></ul></details></li><li class="astro-ojxuodky"><a href="/docs/subagents/" class="astro-ojxuodky"><span class="astro-ojxuodky">Custom subagents</span></a></li><li class="astro-ojxuodky"><a href="/docs/hooks/" class="astro-ojxuodky"><span class="astro-ojxuodky">Lifecycle hooks</span></a></li><li class="astro-ojxuodky"><a href="/docs/sidecars/" class="astro-ojxuodky"><span class="astro-ojxuodky">Sidecars</span></a></li></ul></section></div><div class="sidebar-sections astro-s3sbtxwf" data-sidebar-section="sdk"><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">ANTIGRAVITY SDK</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/sdk/overview/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview and quickstart</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/local-models/" class="astro-ojxuodky"><span class="astro-ojxuodky">Local models</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/personas/" class="astro-ojxuodky"><span class="astro-ojxuodky">Personas</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/tools/" class="astro-ojxuodky"><span class="astro-ojxuodky">Tools and skills</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/mcp/" class="astro-ojxuodky"><span class="astro-ojxuodky">MCP</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/policies/" class="astro-ojxuodky"><span class="astro-ojxuodky">Policies</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/subagents/" class="astro-ojxuodky"><span class="astro-ojxuodky">Subagents</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/structured-output/" class="astro-ojxuodky"><span class="astro-ojxuodky">Structured output</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/lifecycle/" class="astro-ojxuodky"><span class="astro-ojxuodky">Lifecycle and hooks</span></a></li></ul></section></div><div class="sidebar-sections astro-s3sbtxwf" data-sidebar-section="changelog"></div><script aria-hidden="true">
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
            " class="astro-sgnutmzh"></path></svg></span><span class="display-current astro-sgnutmzh"></span></summary><div class="dropdown astro-sgnutmzh"><ul class="isMobile astro-sgnutmzh"><li class="astro-sgnutmzh"><a href="#_top" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Overview</span></a></li><li class="astro-sgnutmzh"><a href="#core-slash-commands" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Core slash commands</span></a></li><li class="astro-sgnutmzh"><a href="#default-keybindings" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Default keybindings</span></a><ul class="astro-sgnutmzh"><li class="astro-sgnutmzh"><a href="#global-controls" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Global controls</span></a></li><li class="astro-sgnutmzh"><a href="#prompt-focus-keys" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Prompt focus keys</span></a></li><li class="astro-sgnutmzh"><a href="#navigation-and-scrolling" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Navigation and scrolling</span></a></li><li class="astro-sgnutmzh"><a href="#tool-confirmations" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Tool confirmations</span></a></li></ul></li><li class="astro-sgnutmzh"><a href="#configuration-keys-settingsjson" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Configuration keys (settings.json)</span></a><ul class="astro-sgnutmzh"><li class="astro-sgnutmzh"><a href="#example-settingsjson" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Example settings.json</span></a></li></ul></li><li class="astro-sgnutmzh"><a href="#next-steps" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Next steps</span></a></li></ul></div></details></nav></mobile-starlight-toc><script type="module">var e=class extends HTMLElement{_current=null;_onScroll=null;_observer=null;_cleanup=[];set current(e){if(e===this._current)return;this._current&&this._current.removeAttribute(`aria-current`),e.setAttribute(`aria-current`,`true`),this._current=e;let t=this.querySelector(`.display-current`);t&&(t.textContent=e.textContent)}connectedCallback(){this.init()}disconnectedCallback(){this._onScroll&&=(window.removeEventListener(`scroll`,this._onScroll),null),this._observer&&=(this._observer.disconnect(),null),this._cleanup.forEach(e=>e()),this._cleanup=[]}init=()=>{let e=this.querySelector(`details`);if(e){let t=()=>{e.open=!1};e.querySelectorAll(`a`).forEach(e=>{e.addEventListener(`click`,t)}),window.addEventListener(`click`,n=>{e.contains(n.target)||t()}),window.addEventListener(`keydown`,n=>{n.key===`Escape`&&e.open&&(t(),e.querySelector(`summary`)?.focus())})}let t=Array.from(this.querySelectorAll(`a`));if(!t.length)return;let n=[],r=new Map,i=document.getElementById(`_top`);if(i){let e=t.find(e=>e.hash===`#_top`);if(e){r.set(`_top`,e);let t=e.closest(`li`);t&&n.push({link:e,li:t,heading:i,tabpanel:null})}}t.forEach(e=>{let t=decodeURIComponent(e.hash.slice(1));if(!t||t===`_top`)return;let i=document.getElementById(t),a=e.closest(`li`);if(i&&a){r.set(t,e);let o=i.closest(`[role="tabpanel"]`);n.push({link:e,li:a,heading:i,tabpanel:o})}}),t.forEach(e=>{e.addEventListener(`click`,()=>{let t=decodeURIComponent(e.hash.slice(1));if(!t)return;let n=document.getElementById(t);if(n){let e=n.closest(`starlight-tabs [role="tabpanel"]`);if(e){let t=e.closest(`starlight-tabs`);if(t){let n=Array.from(t.querySelectorAll(`:scope > [role="tabpanel"]`)).indexOf(e);n!==-1&&(t.querySelectorAll(`[role="tab"]`)[n]?.click(),requestAnimationFrame(()=>{s()}))}}}})});let a=!1,o=()=>{a=!1;let e=n.filter(e=>e.heading.id===`_top`||e.heading.offsetParent!==null&&(!e.tabpanel||!e.tabpanel.hasAttribute(`hidden`)&&!e.tabpanel.hidden&&e.tabpanel.offsetParent!==null));if(!e.length)return;let t=e[0];for(let n of e)if(n.heading.getBoundingClientRect().top<=130)t=n;else break;let i=r.get(t.heading.id);i&&(this.current=i)},s=()=>{n.forEach(e=>{if(!e.tabpanel){e.li.style.display=``;return}let t=e.tabpanel.hasAttribute(`hidden`)||e.tabpanel.hidden||e.tabpanel.offsetParent===null;e.li.style.display=t?`none`:``}),o()},c=document.querySelectorAll(`[role="tabpanel"]`);if(c.length>0){this._observer=new MutationObserver(()=>{s()}),c.forEach(e=>{this._observer?.observe(e,{attributes:!0,attributeFilter:[`hidden`,`style`,`class`]})});let e=e=>{let t=e.target;(t?.closest(`[role="tab"]`)||t?.closest(`.surface-tag-pill`)||t?.closest(`.tab-button`))&&requestAnimationFrame(()=>{s()})};document.addEventListener(`click`,e),this._cleanup.push(()=>{document.removeEventListener(`click`,e)})}this._onScroll=()=>{a||=(requestAnimationFrame(o),!0)},window.addEventListener(`scroll`,this._onScroll,{passive:!0}),s()}};customElements.get(`mobile-starlight-toc`)||customElements.define(`mobile-starlight-toc`,e);</script></div><div class="right-sidebar-panel sl-hidden lg:sl-block astro-q5mffni3"><div class="sl-container astro-q5mffni3"><starlight-toc data-min-h="2" data-max-h="3"><nav aria-labelledby="starlight__on-this-page" data-analytics-nav="docs_toc"><h2 id="starlight__on-this-page">On this page</h2><ul class="starlight-toc-list">\<li\><a href="#_top"><span>Overview</span></a></li>\<li\><a href="#core-slash-commands"><span>Core slash commands</span></a></li>\<li\><a href="#default-keybindings"><span>Default keybindings</span></a>\<ul\>\<li\><a href="#global-controls"><span>Global controls</span></a></li>\<li\><a href="#prompt-focus-keys"><span>Prompt focus keys</span></a></li>\<li\><a href="#navigation-and-scrolling"><span>Navigation and scrolling</span></a></li>\<li\><a href="#tool-confirmations"><span>Tool confirmations</span></a></li></ul></li>\<li\><a href="#configuration-keys-settingsjson"><span>Configuration keys (settings.json)</span></a>\<ul\>\<li\><a href="#example-settingsjson"><span>Example settings.json</span></a></li></ul></li>\<li\><a href="#next-steps"><span>Next steps</span></a></li></ul></nav></starlight-toc><script type="module">var e=class extends HTMLElement{_current=null;_onScroll=null;_observer=null;_cleanup=[];set current(e){e!==this._current&&(this._current&&this._current.removeAttribute(`aria-current`),e.setAttribute(`aria-current`,`true`),this._current=e)}connectedCallback(){this.init()}disconnectedCallback(){this._onScroll&&=(window.removeEventListener(`scroll`,this._onScroll),null),this._observer&&=(this._observer.disconnect(),null),this._cleanup.forEach(e=>e()),this._cleanup=[]}init=()=>{let e=Array.from(this.querySelectorAll(`a`));if(!e.length)return;let t=[],n=new Map,r=document.getElementById(`_top`);if(r){let i=e.find(e=>e.hash===`#_top`);if(i){n.set(`_top`,i);let e=i.closest(`li`);e&&t.push({link:i,li:e,heading:r,tabpanel:null})}}e.forEach(e=>{let r=decodeURIComponent(e.hash.slice(1));if(!r||r===`_top`)return;let i=document.getElementById(r),a=e.closest(`li`);if(i&&a){n.set(r,e);let o=i.closest(`[role="tabpanel"]`);t.push({link:e,li:a,heading:i,tabpanel:o})}}),e.forEach(e=>{e.addEventListener(`click`,()=>{let t=decodeURIComponent(e.hash.slice(1));if(!t)return;let n=document.getElementById(t);if(n){let e=n.closest(`starlight-tabs [role="tabpanel"]`);if(e){let t=e.closest(`starlight-tabs`);if(t){let n=Array.from(t.querySelectorAll(`:scope > [role="tabpanel"]`)).indexOf(e);n!==-1&&(t.querySelectorAll(`[role="tab"]`)[n]?.click(),requestAnimationFrame(()=>{o()}))}}}})});let i=!1,a=()=>{i=!1;let e=t.filter(e=>e.heading.id===`_top`||e.heading.offsetParent!==null&&(!e.tabpanel||!e.tabpanel.hasAttribute(`hidden`)&&!e.tabpanel.hidden&&e.tabpanel.offsetParent!==null));if(!e.length)return;let r=e[0];for(let t of e)if(t.heading.getBoundingClientRect().top<=130)r=t;else break;let a=n.get(r.heading.id);a&&(this.current=a)},o=()=>{t.forEach(e=>{if(!e.tabpanel){e.li.style.display=``;return}let t=e.tabpanel.hasAttribute(`hidden`)||e.tabpanel.hidden||e.tabpanel.offsetParent===null;e.li.style.display=t?`none`:``}),a()},s=document.querySelectorAll(`[role="tabpanel"]`);if(s.length>0){this._observer=new MutationObserver(()=>{o()}),s.forEach(e=>{this._observer?.observe(e,{attributes:!0,attributeFilter:[`hidden`,`style`,`class`]})});let e=e=>{let t=e.target;(t?.closest(`[role="tab"]`)||t?.closest(`.surface-tag-pill`)||t?.closest(`.tab-button`))&&requestAnimationFrame(()=>{o()})};document.addEventListener(`click`,e),this._cleanup.push(()=>{document.removeEventListener(`click`,e)})}this._onScroll=()=>{i||=(requestAnimationFrame(a),!0)},window.addEventListener(`scroll`,this._onScroll,{passive:!0}),o()}};customElements.get(`starlight-toc`)||customElements.define(`starlight-toc`,e);</script></div></div></div></aside><div class="main-pane astro-abfeecmu"><main data-pagefind-body class="astro-vdxcvpsm" lang="en" dir="ltr"><div class="content-panel astro-oil7k7jf"><div class="sl-container astro-oil7k7jf"><div class="breadcrumbs-row astro-hkwnjoai"><nav aria-label="Breadcrumb" class="starlight-breadcrumbs astro-hkwnjoai"><span class="breadcrumb-item  astro-hkwnjoai" title="Get Started">Get Started</span><svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" class="breadcrumb-separator astro-hkwnjoai" aria-hidden="true"><polyline points="9 18 15 12 9 6" class="astro-hkwnjoai"></polyline></svg><span class="breadcrumb-item  astro-hkwnjoai" title="CLI">CLI</span><svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" class="breadcrumb-separator astro-hkwnjoai" aria-hidden="true"><polyline points="9 18 15 12 9 6" class="astro-hkwnjoai"></polyline></svg><span class="breadcrumb-item breadcrumb-last astro-hkwnjoai" title="CLI reference">CLI reference</span></nav><div id="md-actions-root" class="markdown-dropdown-wrapper astro-pyacvpuh" data-pathname="/docs/cli/reference/"><button type="button" class="call-to-action--nav button button-primary button-compact markdown-trigger-btn dropdown-nav" aria-haspopup="true" aria-expanded="false" aria-label="Markdown Options" class="astro-pyacvpuh"><span>Markdown</span><span class="dropdown-icon" translate="no"><svg class viewBox="0 0 24 24" width="18" height="18" fill="currentColor" aria-hidden="true" xmlns="http://www.w3.org/2000/svg"><path d="M12 15L7.05 10.05L8.45 8.65L12 12.2L15.55 8.65L16.95 10.05L12 15Z"></path></svg></span></button><div class="markdown-dropdown-menu astro-pyacvpuh" data-md-menu hidden><button type="button" class="markdown-menu-item astro-pyacvpuh" data-copy-md><svg class="icon astro-pyacvpuh" viewBox="0 -960 960 960" width="16px" height="16px" fill="currentColor" aria-hidden="true" xmlns="http://www.w3.org/2000/svg"><path d="M318.46-230.77q-23.53,0-40.61-17.08t-17.08-40.61V-802.31q0-23.53 17.08-40.61T318.46-860H712.31q23.53,0 40.61,17.08T770-802.31v513.84q0,23.53-17.08,40.61t-40.61,17.08H318.46Zm0-45.38H712.31q4.62,0 8.46-3.85t3.85-8.46V-802.31q0-4.62-3.85-8.46t-8.46-3.85H318.46q-4.62,0-8.46,3.85t-3.85,8.46v513.84q0,4.62 3.85,8.46t8.46,3.85ZM207.69-120q-23.53,0-40.61-17.08T150-177.7V-736.92h45.38V-177.7q0,4.62 3.85,8.46t8.46,3.85H646.92V-120H207.69Zm98.46-156.15q0,0 0-3.85t0-8.46V-802.31q0-4.62 0-8.46t0-3.85q0,0 0,3.85t0,8.46v513.84q0,4.62 0,8.46t0,3.85Z"></path></svg><span class="item-label astro-pyacvpuh">Copy Markdown</span></button><a href="#" data-view-md-link target="_blank" rel="noopener noreferrer" class="markdown-menu-item astro-pyacvpuh"><svg class="icon astro-pyacvpuh" viewBox="0 -960 960 960" width="16px" height="16px" fill="currentColor" aria-hidden="true" xmlns="http://www.w3.org/2000/svg"><path d="M197.69-140q-23.53,0-40.61-17.08T140-197.69V-762.31q0-23.53 17.08-40.61T197.69-820H449.38v45.38H197.69q-4.62,0-8.46,3.85t-3.85,8.46v564.61q0,4.62 3.85,8.46t8.46,3.85H762.31q4.62,0 8.46-3.85t3.85-8.46V-449.38H820v251.69q0,23.53-17.08,40.61T762.31-140H197.69ZM384.31-351.69l-32-32.61L742.62-774.61H530.15V-820H820v289.84H774.61V-742L384.31-351.69Z"></path></svg><span class="item-label astro-pyacvpuh">View Markdown</span></a></div></div><script type="module">function e(){document.querySelectorAll(`.markdown-dropdown-wrapper`).forEach(e=>{let t=e.querySelector(`.markdown-trigger-btn, .button`),n=e.querySelector(`[data-md-menu]`),r=e.querySelector(`[data-copy-md]`),i=e.querySelector(`[data-view-md-link]`),a=`${(e.dataset.pathname||window.location.pathname).replace(/\/$/,``)}.md`;i&&(i.href=a),!(!t||!n)&&(t.addEventListener(`click`,e=>{e.stopPropagation(),n.hasAttribute(`hidden`)?(n.removeAttribute(`hidden`),t.setAttribute(`aria-expanded`,`true`)):(n.setAttribute(`hidden`,``),t.setAttribute(`aria-expanded`,`false`))}),document.addEventListener(`click`,r=>{e.contains(r.target)||(n.setAttribute(`hidden`,``),t.setAttribute(`aria-expanded`,`false`))}),r&&r.addEventListener(`click`,async e=>{e.stopPropagation();let i=r.querySelector(`.item-label`),o=i?.textContent||`Copy Markdown`;try{i&&(i.textContent=`Fetching...`);let e=await fetch(a);if(!e.ok)throw Error(`Failed to fetch markdown: ${e.status}`);let r=await e.text(),s=new DOMParser().parseFromString(r,`text/html`).querySelector(`pre`),c=s?s.textContent||``:r;await navigator.clipboard.writeText(c),i&&(i.textContent=`Copied!`,setTimeout(()=>{i.textContent=o,n.setAttribute(`hidden`,``),t.setAttribute(`aria-expanded`,`false`)},1200))}catch(e){console.error(`Error copying markdown:`,e),i&&(i.textContent=`Error copying`),setTimeout(()=>{i&&(i.textContent=o)},2e3)}}))})}document.readyState===`loading`?document.addEventListener(`DOMContentLoaded`,e):e(),document.addEventListener(`astro:page-load`,e);</script></div><script type="module" src="/_astro/StarlightPageTitleOverride.astro_astro_type_script_index_0_lang.DfB6sZL0.js"></script></div></div><div class="content-panel astro-oil7k7jf"><div class="sl-container astro-oil7k7jf"><div class="sl-markdown-content"><div class="sl-heading-wrapper level-h1"><h1 id="cli-reference">CLI reference</h1><a class="sl-anchor-link" href="#cli-reference"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “CLI reference”</span></a></div>
<p>Review tables listing all TUI slash commands, default keyboard shortcuts, and JSON configuration parameters.</p>
<div class="sl-heading-wrapper level-h2"><h2 id="core-slash-commands">Core slash commands</h2><a class="sl-anchor-link" href="#core-slash-commands"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Core slash commands”</span></a></div>
<p>Type <code dir="auto">/</code> inside the prompt box to open the typeahead command selection menu.</p>



































































































































































































































<table><thead><tr><th align="left">Command</th><th align="left">Category</th><th align="left">Alias</th><th align="left">Execution Purpose</th></tr></thead>\<tbody\><tr><td align="left"><strong><code dir="auto">/add-dir &#x3C;path></code></strong></td><td align="left">Utilities</td><td align="left">—</td><td align="left">Add a directory path to the active workspace.</td></tr><tr><td align="left"><strong><a href="/docs/cli/commands/agents"><code dir="auto">/agents</code></a></strong></td><td align="left">Tools &#x26; Tasks</td><td align="left">—</td><td align="left">Open the <a href="/docs/cli/commands/agents">Agent Manager panel</a> to switch custom agents and monitor background subagents.</td></tr><tr><td align="left"><strong><a href="/docs/boost"><code dir="auto">/boost</code></a></strong> <code dir="auto">&#x3C;task></code></td><td align="left">Reasoning</td><td align="left">—</td><td align="left">Run on-demand multi-agent deep reasoning loops.</td></tr><tr><td align="left"><strong><code dir="auto">/artifact</code></strong></td><td align="left">Tools &#x26; Tasks</td><td align="left">—</td><td align="left">Open the Artifact Review panel.</td></tr><tr><td align="left"><strong><code dir="auto">/btw &#x3C;query></code></strong></td><td align="left">Utilities</td><td align="left">—</td><td align="left">Ask a side question in the background without interrupting the main conversation.</td></tr><tr><td align="left"><strong><code dir="auto">/clear</code></strong></td><td align="left">Utilities</td><td align="left"><code dir="auto">/new</code></td><td align="left">Clear the terminal and reset active conversation contexts.</td></tr><tr><td align="left"><strong><code dir="auto">/config</code></strong></td><td align="left">Configurations</td><td align="left"><code dir="auto">/settings</code></td><td align="left">Open the interactive Settings Editor overlay.</td></tr><tr><td align="left"><strong><code dir="auto">/context</code></strong></td><td align="left">Utilities</td><td align="left">—</td><td align="left">Open the context usage visualization panel.</td></tr><tr><td align="left"><strong><code dir="auto">/copy</code></strong></td><td align="left">Utilities</td><td align="left">—</td><td align="left">Copy the last agent response to the system clipboard.</td></tr><tr><td align="left"><strong><a href="/docs/cli/commands/credits"><code dir="auto">/credits</code></a></strong></td><td align="left">Account</td><td align="left">—</td><td align="left">View remaining G1 credits and purchase links.</td></tr><tr><td align="left"><strong><a href="/docs/cli/commands/diff"><code dir="auto">/diff</code></a></strong></td><td align="left">Utilities</td><td align="left">—</td><td align="left">Open the <a href="/docs/cli/commands/diff">interactive diff viewer</a> to view changes, turns, and commits.</td></tr><tr><td align="left"><strong><code dir="auto">/exit</code></strong></td><td align="left">Core</td><td align="left"><code dir="auto">/quit</code></td><td align="left">Close the TUI session and restore your host shell.</td></tr><tr><td align="left"><strong><code dir="auto">/fast</code></strong></td><td align="left">Configurations</td><td align="left">—</td><td align="left">Enable fast mode (bypass reasoning plans) for quick actions.</td></tr><tr><td align="left"><strong><code dir="auto">/feedback</code></strong></td><td align="left">Utilities</td><td align="left">—</td><td align="left">Open the feedback submission panel.</td></tr><tr><td align="left"><strong><code dir="auto">/fork</code></strong></td><td align="left">Conversations</td><td align="left"><code dir="auto">/branch</code></td><td align="left">Clone the current conversation thread into a new parallel session.</td></tr><tr><td align="left"><strong><code dir="auto">/help</code></strong></td><td align="left">Utilities</td><td align="left">—</td><td align="left">Open the help panel showing commands and shortcuts.</td></tr><tr><td align="left"><strong><code dir="auto">/hooks</code></strong></td><td align="left">Tools &#x26; Tasks</td><td align="left">—</td><td align="left">Browse active pre-flight/post-format script hooks.</td></tr><tr><td align="left"><strong><code dir="auto">/keybindings</code></strong></td><td align="left">Configurations</td><td align="left">—</td><td align="left">Open the interactive Keyboard Shortcut Editor.</td></tr><tr><td align="left"><strong><code dir="auto">/logout</code></strong></td><td align="left">Account</td><td align="left">—</td><td align="left">Disconnect your profile and purge authentication tokens from the secure keyring.</td></tr><tr><td align="left"><strong><code dir="auto">/mcp</code></strong></td><td align="left">Tools &#x26; Tasks</td><td align="left">—</td><td align="left">Open the Model Context Protocol (MCP) server manager.</td></tr><tr><td align="left"><strong><code dir="auto">/model</code></strong></td><td align="left">Configurations</td><td align="left">—</td><td align="left">Choose your preferred reasoning model (persists across sessions).</td></tr><tr><td align="left"><strong><code dir="auto">/open &#x3C;path></code></strong></td><td align="left">Utilities</td><td align="left">—</td><td align="left">Force the path to open inside your default system editor.</td></tr><tr><td align="left"><strong><a href="/docs/cli/commands/permissions"><code dir="auto">/permissions</code></a></strong></td><td align="left">Configurations</td><td align="left">—</td><td align="left">Open the interactive tool permissions manager panel.</td></tr><tr><td align="left"><strong><code dir="auto">/planning</code></strong></td><td align="left">Configurations</td><td align="left">—</td><td align="left">Enable multi-turn plan generation mode for complex engineering tasks.</td></tr><tr><td align="left"><strong><a href="/docs/plugins?tab=cli"><code dir="auto">/plugin</code></a></strong></td><td align="left">Tools &#x26; Tasks</td><td align="left"><code dir="auto">/plugins</code></td><td align="left">Open the interactive <a href="/docs/marketplace?tab=cli">Plugins Manager</a> or run inline <code dir="auto">install</code>, <code dir="auto">uninstall</code>, <code dir="auto">enable</code>, <code dir="auto">disable</code>, and <code dir="auto">list</code> commands.</td></tr><tr><td align="left"><strong><code dir="auto">/rename &#x3C;name></code></strong></td><td align="left">Conversations</td><td align="left">—</td><td align="left">Rename the current session thread.</td></tr><tr><td align="left"><strong><a href="/docs/remote-control?tab=cli#interactive-mode"><code dir="auto">/remote-control</code></a> [on/off]</strong></td><td align="left">Utilities</td><td align="left">—</td><td align="left">Toggle Remote Control for the active terminal session.</td></tr><tr><td align="left"><strong><a href="/docs/cli/commands/resume"><code dir="auto">/resume</code></a></strong></td><td align="left">Conversations</td><td align="left"><code dir="auto">/switch</code>, <code dir="auto">/conversation</code></td><td align="left">Open the <a href="/docs/cli/commands/resume">conversation picker overlay</a> to select and load previous threads.</td></tr><tr><td align="left"><strong><code dir="auto">/rewind</code></strong></td><td align="left">Conversations</td><td align="left"><code dir="auto">/undo</code></td><td align="left">Roll back your conversation history to a previous message.</td></tr><tr><td align="left"><strong><code dir="auto">/skills</code></strong></td><td align="left">Tools &#x26; Tasks</td><td align="left">—</td><td align="left">Browse loaded local and global Agent Skills.</td></tr><tr><td align="left"><strong><a href="/docs/cli/commands/statusline"><code dir="auto">/statusline</code></a></strong></td><td align="left">Configurations</td><td align="left">—</td><td align="left">Open the Status Bar customization overlay.</td></tr><tr><td align="left"><strong><code dir="auto">/tasks</code></strong></td><td align="left">Tools &#x26; Tasks</td><td align="left">—</td><td align="left">Open the Task Manager panel to monitor background shell execution logs.</td></tr><tr><td align="left"><strong><a href="/docs/teamwork"><code dir="auto">/teamwork-preview</code></a></strong> <code dir="auto">&#x3C;task></code></td><td align="left">Reasoning</td><td align="left"><code dir="auto">/teamwork</code></td><td align="left">Launch <a href="/docs/teamwork">collaborative multi-agent teams</a> for long-horizon projects (paid plans).</td></tr><tr><td align="left"><strong><a href="/docs/cli/commands/title"><code dir="auto">/title</code></a> [on/off]</strong></td><td align="left">Configurations</td><td align="left">—</td><td align="left">Toggle or set terminal window title updates.</td></tr><tr><td align="left"><strong><a href="/docs/cli/commands/usage"><code dir="auto">/usage</code></a></strong></td><td align="left">Utilities</td><td align="left"><code dir="auto">/quota</code></td><td align="left">Display model quota usage.</td></tr><tr><td align="left"><strong><a href="/docs/cli/commands/voice"><code dir="auto">/voice</code></a></strong></td><td align="left">Utilities</td><td align="left"><code dir="auto">/record</code></td><td align="left">Dictate a prompt using your microphone.</td></tr></tbody></table>
<div class="sl-heading-wrapper level-h2"><h2 id="default-keybindings">Default keybindings</h2><a class="sl-anchor-link" href="#default-keybindings"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Default keybindings”</span></a></div>
<p>The following tables list the keyboard shortcut commands mapping global, prompt, navigation, and approval operations.</p>
<div class="sl-heading-wrapper level-h3"><h3 id="global-controls">Global controls</h3><a class="sl-anchor-link" href="#global-controls"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Global controls”</span></a></div>
<p>These hotkeys are always active regardless of which panel, overlay, or prompt is currently focused.</p>






























<table><thead><tr><th align="left">Key</th><th align="left">TUI Command</th><th align="left">Action Behavior</th></tr></thead>\<tbody\><tr><td align="left"><strong><code dir="auto">Esc</code></strong></td><td align="left"><code dir="auto">cli.escape</code></td><td align="left">Closes active panels, halts active streams, or clears empty prompts.</td></tr><tr><td align="left"><strong><code dir="auto">Ctrl+C</code></strong></td><td align="left"><code dir="auto">cli.exit</code></td><td align="left">Terminates the CLI session (prompts for confirmation if agent is working).</td></tr><tr><td align="left"><strong><code dir="auto">Ctrl+D</code></strong></td><td align="left"><code dir="auto">cli.exit</code></td><td align="left">Exits the CLI session (only when the prompt box is empty).</td></tr><tr><td align="left"><strong><code dir="auto">Ctrl+L</code></strong></td><td align="left"><code dir="auto">cli.clear_screen</code></td><td align="left">Refreshes and clears the visual terminal buffer.</td></tr></tbody></table>
<div class="sl-heading-wrapper level-h3"><h3 id="prompt-focus-keys">Prompt focus keys</h3><a class="sl-anchor-link" href="#prompt-focus-keys"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Prompt focus keys”</span></a></div>
<p>These keys are active when writing instructions inside the prompt box.</p>
















































































<table><thead><tr><th align="left">Key</th><th align="left">TUI Command</th><th align="left">Action Behavior</th></tr></thead>\<tbody\><tr><td align="left"><strong><code dir="auto">Enter</code></strong></td><td align="left"><code dir="auto">prompt.submit</code></td><td align="left">Submits your prompt or active menu selection to the agent.</td></tr><tr><td align="left"><strong><code dir="auto">Shift+Enter</code></strong> / <strong><code dir="auto">Ctrl+J</code></strong></td><td align="left"><code dir="auto">prompt.newline</code></td><td align="left">Inserts a clean newline without submitting.</td></tr><tr><td align="left"><strong><code dir="auto">Ctrl+V</code></strong></td><td align="left"><code dir="auto">prompt.paste</code></td><td align="left">Pastes graphic media files or clipboard blocks into the prompt.</td></tr><tr><td align="left"><strong><code dir="auto">Ctrl+O</code></strong></td><td align="left"><code dir="auto">prompt.toggle_trajectory</code></td><td align="left">Expands or collapses detailed tool reasoning outputs.</td></tr><tr><td align="left"><strong><code dir="auto">Ctrl+R</code></strong></td><td align="left"><code dir="auto">prompt.open_review</code></td><td align="left">Opens the Artifact Review panel.</td></tr><tr><td align="left"><strong><code dir="auto">Ctrl+G</code></strong></td><td align="left"><code dir="auto">prompt.external_editor</code></td><td align="left">Launches your default <code dir="auto">$EDITOR</code> shell to compose your prompt.</td></tr><tr><td align="left"><strong><code dir="auto">Alt+J</code></strong></td><td align="left"><code dir="auto">prompt.teleport_agent</code></td><td align="left">Instantly switches focus to the next subagent awaiting confirmation.</td></tr><tr><td align="left"><strong><code dir="auto">Ctrl+K</code></strong></td><td align="left"><code dir="auto">prompt.fast_approve</code></td><td align="left">Instantly approves the pending subagent action listed in the status alert.</td></tr><tr><td align="left"><strong><code dir="auto">Ctrl+A</code></strong></td><td align="left"><code dir="auto">prompt.cursor_start</code></td><td align="left">Moves the prompt insertion cursor to the beginning of the line.</td></tr><tr><td align="left"><strong><code dir="auto">Ctrl+E</code></strong></td><td align="left"><code dir="auto">prompt.cursor_end</code></td><td align="left">Moves the prompt insertion cursor to the end of the line.</td></tr><tr><td align="left"><strong><code dir="auto">Ctrl+Z</code></strong></td><td align="left"><code dir="auto">prompt.undo_text</code></td><td align="left">Reverts the last edit.</td></tr><tr><td align="left"><strong><code dir="auto">Ctrl+Shift+Z</code></strong></td><td align="left"><code dir="auto">prompt.redo_text</code></td><td align="left">Redoes the last undone text operation.</td></tr><tr><td align="left"><strong><code dir="auto">Ctrl+D</code></strong></td><td align="left"><code dir="auto">—</code></td><td align="left">Forward delete (only when the prompt box is non-empty).</td></tr><tr><td align="left"><strong><code dir="auto">F5</code></strong></td><td align="left"><code dir="auto">voice.start_dictation</code></td><td align="left">Starts or stops <a href="/docs/cli/commands/voice">voice dictation</a>.</td></tr></tbody></table>
<div class="sl-heading-wrapper level-h3"><h3 id="navigation-and-scrolling">Navigation and scrolling</h3><a class="sl-anchor-link" href="#navigation-and-scrolling"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Navigation and scrolling”</span></a></div>
<p>These keys are active inside select panels, menus, and scrollable text boxes.</p>



































<table><thead><tr><th align="left">Key</th><th align="left">TUI Command</th><th align="left">Action Behavior</th></tr></thead>\<tbody\><tr><td align="left"><strong><code dir="auto">↑</code></strong> / <strong><code dir="auto">↓</code></strong></td><td align="left"><code dir="auto">navigation.up</code> / <code dir="auto">navigation.down</code></td><td align="left">Scrolls highlighted selections up or down by one item.</td></tr><tr><td align="left"><strong><code dir="auto">PgUp</code></strong> / <strong><code dir="auto">Shift+↑</code></strong></td><td align="left"><code dir="auto">navigation.page_up</code></td><td align="left">Scrolls the active text viewport up by one page block.</td></tr><tr><td align="left"><strong><code dir="auto">PgDn</code></strong> / <strong><code dir="auto">Shift+↓</code></strong></td><td align="left"><code dir="auto">navigation.page_down</code></td><td align="left">Scrolls the active text viewport down by one page block.</td></tr><tr><td align="left"><strong><code dir="auto">←</code></strong> / <strong><code dir="auto">→</code></strong></td><td align="left"><code dir="auto">navigation.left</code> / <code dir="auto">navigation.right</code></td><td align="left">Swaps pages inside multipage structures (like the Session Picker).</td></tr><tr><td align="left"><strong><code dir="auto">Tab</code></strong></td><td align="left"><code dir="auto">navigation.tab</code></td><td align="left">Confirms the highlighted slash-command autofill option.</td></tr></tbody></table>
<div class="sl-heading-wrapper level-h3"><h3 id="tool-confirmations">Tool confirmations</h3><a class="sl-anchor-link" href="#tool-confirmations"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Tool confirmations”</span></a></div>
<p>These keys are active during confirmation prompts.</p>

























<table><thead><tr><th align="left">Key</th><th align="left">TUI Command</th><th align="left">Action Behavior</th></tr></thead>\<tbody\><tr><td align="left"><strong><code dir="auto">y</code></strong></td><td align="left"><code dir="auto">confirm.yes</code></td><td align="left">Authorizes the proposed tool, command, or active artifact.</td></tr><tr><td align="left"><strong><code dir="auto">n</code></strong></td><td align="left"><code dir="auto">confirm.no</code></td><td align="left">Rejects the proposed tool, command, or active artifact.</td></tr><tr><td align="left"><strong><code dir="auto">A</code></strong></td><td align="left"><code dir="auto">—</code></td><td align="left">(Inside Review panel) Approves all generated artifacts in one action (built-in shortcut).</td></tr></tbody></table>
<div class="sl-heading-wrapper level-h2"><h2 id="configuration-keys-settingsjson">Configuration keys (<code dir="auto">settings.json</code>)</h2><a class="sl-anchor-link" href="#configuration-keys-settingsjson"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Configuration keys (settings.json)”</span></a></div>
<p>This section lists the primary settings key names, data types, system defaults, and expected parameters.</p>
<div class="sl-heading-wrapper level-h3"><h3 id="example-settingsjson">Example <code dir="auto">settings.json</code></h3><a class="sl-anchor-link" href="#example-settingsjson"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Example settings.json”</span></a></div>
<pre class="astro-code astro-code-themes github-light github-dark" style="background-color:#fff;--shiki-dark-bg:#24292e;color:#24292e;--shiki-dark:#e1e4e8; overflow-x: auto; white-space: pre-wrap; word-wrap: break-word;" tabindex="0" data-language="json" dir="ltr"><code><span class="line"><span style="color:#24292E;--shiki-dark:#E1E4E8">{</span></span>
<span class="line"><span style="color:#005CC5;--shiki-dark:#79B8FF">    "colorScheme"</span><span style="color:#24292E;--shiki-dark:#E1E4E8">: </span><span style="color:#032F62;--shiki-dark:#9ECBFF">"tokyo night"</span><span style="color:#24292E;--shiki-dark:#E1E4E8">,</span></span>
<span class="line"><span style="color:#005CC5;--shiki-dark:#79B8FF">    "altScreenMode"</span><span style="color:#24292E;--shiki-dark:#E1E4E8">: </span><span style="color:#032F62;--shiki-dark:#9ECBFF">"always"</span><span style="color:#24292E;--shiki-dark:#E1E4E8">,</span></span>
<span class="line"><span style="color:#005CC5;--shiki-dark:#79B8FF">    "toolPermission"</span><span style="color:#24292E;--shiki-dark:#E1E4E8">: </span><span style="color:#032F62;--shiki-dark:#9ECBFF">"request-review"</span><span style="color:#24292E;--shiki-dark:#E1E4E8">,</span></span>
<span class="line"><span style="color:#005CC5;--shiki-dark:#79B8FF">    "notifications"</span><span style="color:#24292E;--shiki-dark:#E1E4E8">: </span><span style="color:#005CC5;--shiki-dark:#79B8FF">true</span><span style="color:#24292E;--shiki-dark:#E1E4E8">,</span></span>
<span class="line"><span style="color:#005CC5;--shiki-dark:#79B8FF">    "enableTerminalSandbox"</span><span style="color:#24292E;--shiki-dark:#E1E4E8">: </span><span style="color:#005CC5;--shiki-dark:#79B8FF">true</span></span>
<span class="line"><span style="color:#24292E;--shiki-dark:#E1E4E8">}</span></span></code></pre>











































































































<table><thead><tr><th align="left">Option Key Name</th><th align="left">Value Type</th><th align="left">System Default</th><th align="left">Parameter Characteristics &#x26; Options</th></tr></thead>\<tbody\><tr><td align="left"><strong><code dir="auto">colorScheme</code></strong></td><td align="left">string</td><td align="left"><code dir="auto">"terminal"</code></td><td align="left">Color theme: <code dir="auto">"light"</code>, <code dir="auto">"solarized light"</code>, <code dir="auto">"colorblind-friendly light"</code>, <code dir="auto">"dark"</code>, <code dir="auto">"solarized dark"</code>, <code dir="auto">"colorblind-friendly dark"</code>, <code dir="auto">"tokyo night"</code>, or <code dir="auto">"terminal"</code> (inherits native shell colors).</td></tr><tr><td align="left"><strong><code dir="auto">altScreenMode</code></strong></td><td align="left">string</td><td align="left"><code dir="auto">"default"</code></td><td align="left">Screen buffer usage: <code dir="auto">"default"</code> (adaptive inline/altscreen), <code dir="auto">"always"</code> (force alternate screen buffer), or <code dir="auto">"never"</code> (force inline sequential output).</td></tr><tr><td align="left"><strong><code dir="auto">toolPermission</code></strong></td><td align="left">string</td><td align="left"><code dir="auto">"request-review"</code></td><td align="left">Global safety presets: <code dir="auto">"request-review"</code> (prompts for write/bash/web tools), <code dir="auto">"proceed-in-sandbox"</code> (auto-proceed inside sandbox), <code dir="auto">"always-proceed"</code> (never prompts), or <code dir="auto">"strict"</code> (prompts for all non-read tools).</td></tr><tr><td align="left"><strong><code dir="auto">artifactReviewPolicy</code></strong></td><td align="left">string</td><td align="left"><code dir="auto">"asks-for-review"</code></td><td align="left">Code review policy: <code dir="auto">"asks-for-review"</code> (always prompts before writing code), <code dir="auto">"agent-decides"</code> (prompts dynamically), or <code dir="auto">"always-proceed"</code> (never prompts).</td></tr><tr><td align="left"><strong><code dir="auto">notifications</code></strong></td><td align="left">boolean</td><td align="left"><code dir="auto">false</code></td><td align="left">Emits system desktop and terminal bell chime notifications upon task completions.</td></tr><tr><td align="left"><strong><code dir="auto">showTips</code></strong></td><td align="left">boolean</td><td align="left"><code dir="auto">true</code></td><td align="left">Displays helpful agentic tips above the prompt panel during generation turns.</td></tr><tr><td align="left"><strong><code dir="auto">showFeedbackSurvey</code></strong></td><td align="left">boolean</td><td align="left"><code dir="auto">true</code></td><td align="left">Displays periodic quality feedback surveys upon active task completions.</td></tr><tr><td align="left"><strong><code dir="auto">editor</code></strong></td><td align="left">string</td><td align="left"><code dir="auto">"auto"</code></td><td align="left">Target text editor utility: <code dir="auto">"auto"</code> (consults system <code dir="auto">$EDITOR</code>), <code dir="auto">"vim"</code>, <code dir="auto">"emacs"</code>, or custom text labels.</td></tr><tr><td align="left"><strong><code dir="auto">editorMode</code></strong></td><td align="left">string</td><td align="left"><code dir="auto">"default"</code></td><td align="left">Prompt editing model: <code dir="auto">"default"</code> (flat text editing) or <code dir="auto">"vim"</code> (modal editing). Distinct from <code dir="auto">editor</code>, which selects an external program. Refer to <a href="/docs/cli/vim-editor-mode">Vim editor mode</a>.</td></tr><tr><td align="left"><strong><code dir="auto">vimInsertFirst</code></strong></td><td align="left">boolean</td><td align="left"><code dir="auto">false</code></td><td align="left">Starts Vim editing in Insert mode and makes a bare <code dir="auto">Enter</code> submit. Requires <code dir="auto">editorMode</code> set to <code dir="auto">"vim"</code>.</td></tr><tr><td align="left"><strong><code dir="auto">allowNonWorkspaceAccess</code></strong></td><td align="left">boolean</td><td align="left"><code dir="auto">false</code></td><td align="left">Permits the agent’s file read and write tools to navigate outside recognized Git/workspace roots.</td></tr><tr><td align="left"><strong><code dir="auto">enableTerminalSandbox</code></strong></td><td align="left">boolean</td><td align="left"><code dir="auto">false</code></td><td align="left">Restricts all local execution commands launched by agents to OS containment rings.</td></tr><tr><td align="left"><strong><code dir="auto">useG1Credits</code></strong></td><td align="left">boolean</td><td align="left"><code dir="auto">false</code></td><td align="left"><em>External builds only.</em> Uses personal AI credits for model calls once plan quotas are exhausted.</td></tr><tr><td align="left"><strong><code dir="auto">enableTelemetry</code></strong></td><td align="left">boolean</td><td align="left"><code dir="auto">true</code></td><td align="left">Permits metric collection and crash log streaming to improve tool reliability.</td></tr><tr><td align="left"><strong><code dir="auto">verbosity</code></strong></td><td align="left">string</td><td align="left"><code dir="auto">"high"</code></td><td align="left">Visual verbosity level: <code dir="auto">"high"</code> (renders full thoughts and tool outputs) or <code dir="auto">"low"</code> (displays only minimal visual progress indicators).</td></tr><tr><td align="left"><strong><code dir="auto">runningLightSpeed</code></strong></td><td align="left">string</td><td align="left"><code dir="auto">"medium"</code></td><td align="left">Visual running light progress animation speed: <code dir="auto">"fast"</code>, <code dir="auto">"medium"</code>, <code dir="auto">"slow"</code>, or <code dir="auto">"off"</code>.</td></tr></tbody></table>
<div class="sl-heading-wrapper level-h2"><h2 id="next-steps">Next steps</h2><a class="sl-anchor-link" href="#next-steps"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Next steps”</span></a></div>
<p>Learn how to safely deploy permission policies, sandboxes, and customize plugins:</p>
\<ul\>
\<li\><strong><a href="/docs/cli/sandbox">Permissions and sandbox</a></strong>: Enforce command-line containment rules.</li>
\<li\><strong><a href="/docs/cli/plugins">Plugins and skills</a></strong>: Create your own custom skills slash commands.</li>
\<li\><strong><a href="/docs/cli/install">Installation and auth</a></strong>: Update your CLI installation.</li>
</ul></div><footer class="sl-flex docs-footer astro-4mgpquaz"><div class="meta sl-flex astro-4mgpquaz"></div><div class="pagination-links print:hidden astro-jgq3vlii" dir="ltr"><a href="/docs/cli/troubleshooting/" rel="prev" class="astro-jgq3vlii"><svg aria-hidden="true" class="astro-jgq3vlii astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.5rem;"><path d="M17 11H9.41l3.3-3.29a1.004 1.004 0 1 0-1.42-1.42l-5 5a1 1 0 0 0-.21.33 1 1 0 0 0 0 .76 1 1 0 0 0 .21.33l5 5a1.002 1.002 0 0 0 1.639-.325 1 1 0 0 0-.219-1.095L9.41 13H17a1 1 0 0 0 0-2Z"/></svg><span class="astro-jgq3vlii">Previous<br class="astro-jgq3vlii"><span class="link-title astro-jgq3vlii">Troubleshooting</span></span></a><a href="/docs/ide/overview/" rel="next" class="astro-jgq3vlii"><svg aria-hidden="true" class="astro-jgq3vlii astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.5rem;"><path d="M17.92 11.62a1.001 1.001 0 0 0-.21-.33l-5-5a1.003 1.003 0 1 0-1.42 1.42l3.3 3.29H7a1 1 0 0 0 0 2h7.59l-3.3 3.29a1.002 1.002 0 0 0 .325 1.639 1 1 0 0 0 1.095-.219l5-5a1 1 0 0 0 .21-.33 1 1 0 0 0 0-.76Z"/></svg><span class="astro-jgq3vlii">Next<br class="astro-jgq3vlii"><span class="link-title astro-jgq3vlii">Antigravity IDE</span></span></a></div><nav class="docs-legal-nav print:hidden astro-4mgpquaz" data-analytics-nav="docs_footer" aria-label="Google legal and cookie links"><a class="docs-footer-link astro-4mgpquaz" href="https://about.google/" target="_blank" rel="noopener noreferrer">About Google</a><a class="docs-footer-link astro-4mgpquaz" href="https://about.google/products/" target="_blank" rel="noopener noreferrer">Google Products</a><a class="docs-footer-link astro-4mgpquaz" href="https://policies.google.com/privacy?hl=en-US" target="_blank" rel="noopener noreferrer">Privacy</a><a class="docs-footer-link astro-4mgpquaz" href="/terms">Terms</a><a aria-hidden="true" class="docs-footer-link glue-cookie-notification-bar-control astro-4mgpquaz">Manage cookies</a></nav></footer></div></div></main></div></div></div></div></body></html>
