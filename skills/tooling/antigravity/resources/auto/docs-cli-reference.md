<!DOCTYPE html><html lang="en" dir="ltr" data-theme="dark" data-has-toc data-has-sidebar class="astro-vdxcvpsm">\<head\><!-- Anti-FOUC masking style: hides text elements while non-default language strings hydrate -->\<style\>
  /* Native cross-document View Transitions declaration enabled at byte 0 */
  @view-transition {
    navigation: auto;
  }
  @media (prefers-reduced-motion: reduce) {
    @view-transition {
      navigation: none;
    }
  }

  html[data-i18n-pending="true"] main,
  html[data-i18n-pending="true"] footer,
  html[data-i18n-pending="true"] nav.sidebar,
  html[data-i18n-pending="true"] .right-sidebar,
  html[data-i18n-pending="true"] [data-header] nav,
  html[data-i18n-pending="true"] [data-header] .download-button,
  html[data-i18n-pending="true"] [data-header] .cta-button,
  html[data-i18n-pending="true"] .docs-nav-links,
  html[data-i18n-pending="true"] .header-download-btn,
  html[data-i18n-pending="true"] .header-cta-btn,
  html[data-i18n-pending="true"] .rc-pill,
  html[data-i18n-pending="true"] .subnav-bar,
  html[data-i18n-pending="true"] site-search button > span {
    opacity: 0 !important;
  }
</style><!-- Synchronous early bootstrap: canonical redirect, slow-connection detection, locale sync, and speculationrules -->\<script\>
  // 0. Suppress benign Chromium View Transition abort / skip rejections during rapid clicks
  window.addEventListener('unhandledrejection', function (event) {
    if (
      event &&
      event.reason &&
      typeof event.reason.message === 'string' &&
      (event.reason.message.indexOf('Transition was aborted') !== -1 ||
        event.reason.message.indexOf('Transition was skipped') !== -1)
    ) {
      event.preventDefault();
    }
  });

  // 1. Canonical hostname redirect (www.antigravity.google -> antigravity.google)
  try {
    var host = (window.location && window.location.hostname) || '';
    if (host === 'www.antigravity.google') {
      var pathname = window.location.pathname || '/';
      var search = window.location.search || '';
      var hash = window.location.hash || '';
      window.location.replace(
        'https://antigravity.google' + pathname + search + hash
      );
    }
  } catch (_) {}

  // 2. Early slow-connection & Save-Data detection
  try {
    var __agyConn =
      navigator.connection ||
      navigator.mozConnection ||
      navigator.webkitConnection;
    var __agySaveData = Boolean(__agyConn && __agyConn.saveData);
    var __agyHasRtt =
      !__agyConn || typeof __agyConn.rtt !== 'number' || __agyConn.rtt > 0;
    var __agyEffType = __agyConn && __agyConn.effectiveType;
    var __agyIsSlowEff =
      __agyHasRtt &&
      (__agyEffType === 'slow-2g' ||
        __agyEffType === '2g' ||
        __agyEffType === '3g');
    var __agyIsLowDownlink = Boolean(
      __agyHasRtt &&
        __agyConn &&
        !__agyEffType &&
        typeof __agyConn.downlink === 'number' &&
        __agyConn.downlink > 0 &&
        __agyConn.downlink < 1.5
    );
    var __agyIsSlow = __agySaveData || __agyIsSlowEff || __agyIsLowDownlink;
    var __agyTier = __agySaveData
      ? 'save-data'
      : __agyIsSlow
        ? 'slow'
        : 'fast';

    document.documentElement.setAttribute('data-connection', __agyTier);
    if (document.documentElement.dataset) {
      document.documentElement.dataset.connection = __agyTier;
    }

    window.__agyIsSlowConnection = function () {
      try {
        var c =
          navigator.connection ||
          navigator.mozConnection ||
          navigator.webkitConnection;
        if (c) {
          if (c.saveData) {
            document.documentElement.setAttribute(
              'data-connection',
              'save-data'
            );
            return true;
          }
          var hasRtt = typeof c.rtt !== 'number' || c.rtt > 0;
          if (
            hasRtt &&
            (c.effectiveType === 'slow-2g' ||
              c.effectiveType === '2g' ||
              c.effectiveType === '3g' ||
              (!c.effectiveType &&
                typeof c.downlink === 'number' &&
                c.downlink > 0 &&
                c.downlink < 1.5))
          ) {
            document.documentElement.setAttribute('data-connection', 'slow');
            return true;
          }
          document.documentElement.setAttribute('data-connection', 'fast');
          return false;
        }
      } catch (_) {}
      var attr = document.documentElement.getAttribute('data-connection');
      return attr === 'slow' || attr === 'save-data';
    };
  } catch (e) {
    window.__agyIsSlowConnection = function () {
      return false;
    };
  }

  // 3. Sync ?hl= / ?lang= URL parameters to localStorage and set anti-FOUC masking
  try {
    var __agyNonDefault = [
      'en-GB',
      'en-CA',
      'hi',
      'ja',
      'ko',
      'zh-CN',
      'zh-TW',
      'id',
      'th',
      'de',
      'fr-FR',
      'it',
      'pl',
      'pt-BR',
      'es-ES',
      'es-MX'
    ];
    var __agyParams = new URLSearchParams(window.location.search);
    var __agyHl = __agyParams.get('hl') || __agyParams.get('lang');
    if (
      __agyHl &&
      (__agyHl === 'en-US' || __agyNonDefault.indexOf(__agyHl) !== -1)
    ) {
      window.localStorage.setItem('antigravity_locale', __agyHl);
    }
    var __agyLoc = window.localStorage.getItem('antigravity_locale');
    if (__agyLoc && __agyNonDefault.indexOf(__agyLoc) !== -1) {
      document.documentElement.lang = __agyLoc;
      document.documentElement.setAttribute('data-i18n-pending', 'true');
      window.__agyI18nTimer = window.setTimeout(function () {
        document.documentElement.removeAttribute('data-i18n-pending');
      }, 350);
    }
  } catch (e) {}

  // 4. Adaptive Speculation Rules: dynamically injected ONLY on fast connections
  try {
    if (
      typeof window.__agyIsSlowConnection === 'function' &&
      !window.__agyIsSlowConnection() &&
      'supports' in HTMLScriptElement &&
      HTMLScriptElement.supports('speculationrules')
    ) {
      var __agySpecScript = document.createElement('script');
      __agySpecScript.type = 'speculationrules';
      __agySpecScript.textContent = JSON.stringify({
        prefetch: [
          {
            where: {
              and: [
                { href_matches: '/*' },
                { not: { href_matches: '/api/*' } },
                { not: { selector_matches: '[download], [target=_blank]' } }
              ]
            },
            eagerness: 'moderate'
          }
        ]
      });
      (document.head || document.documentElement).appendChild(__agySpecScript);
    }
  } catch (e) {}
</script><!-- Client runtime modules: i18n dictionary runtime and adaptive network/transitions manager --><script type="module" src="/_astro/SharedHeadBootstrap.astro_astro_type_script_index_0_lang.DdvTdmaD.js"></script><!-- Google Glue Cookie Notification Bar stylesheet (deferred print->all) --><link href="https://www.gstatic.com/glue/cookienotificationbar/cookienotificationbar.min.css" rel="stylesheet" media="print" onload="this.media='all'" />\<noscript\><link href="https://www.gstatic.com/glue/cookienotificationbar/cookienotificationbar.min.css" rel="stylesheet" /></noscript><meta charset="utf-8"/><meta name="viewport" content="width=device-width, initial-scale=1"/>\<title\>CLI commands - Antigravity CLI | Google Antigravity Docs</title><link rel="canonical" href="https://antigravity.google/docs/cli/reference/"/><link rel="sitemap" href="/sitemap-index.xml"/><link rel="icon" type="image/x-icon" href="/favicon.ico" sizes="48x48"/><link rel="icon" type="image/png" href="/apple-touch-icon.png" sizes="180x180"/><link rel="icon" type="image/png" href="/assets/image/antigravity-logo.png" sizes="276x276"/><link rel="apple-touch-icon" href="/apple-touch-icon.png" sizes="180x180"/><link rel="apple-touch-icon-precomposed" href="/apple-touch-icon-precomposed.png" sizes="180x180"/><link rel="preconnect" href="https://fonts.googleapis.com"/><link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/><link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Google+Symbols:opsz,wght,FILL,GRAD,ROND@20..48,100..700,0..1,-50..200,0..100&display=block" media="print" onload="this.media='all'"/><link rel="shortcut icon" href="/favicon.svg" type="image/svg+xml"/><meta name="generator" content="Astro v7.2.9"/><meta name="generator" content="Starlight v0.41.5"/><meta property="og:title" content="CLI commands - Antigravity CLI | Google Antigravity Docs"/><meta property="og:type" content="article"/><meta property="og:url" content="https://antigravity.google/docs/cli/reference/"/><meta property="og:locale" content="en"/><meta property="og:site_name" content="Google Antigravity Docs"/><meta property="og:image" content="https://antigravity.google/assets/image/sitecards/sitecard-documentation.png"/><meta property="og:image:secure_url" content="https://antigravity.google/assets/image/sitecards/sitecard-documentation.png"/><meta name="twitter:image" content="https://antigravity.google/assets/image/sitecards/sitecard-documentation.png"/><meta name="twitter:card" content="summary_large_image"/><meta name="description" content="Review tables listing all TUI slash commands, default keyboard shortcuts, command-line flags, and JSON configuration parameters."/><meta property="og:description" content="Review tables listing all TUI slash commands, default keyboard shortcuts, command-line flags, and JSON configuration parameters."/><meta name="twitter:title" content="CLI commands - Antigravity CLI | Google Antigravity Docs"/><meta name="twitter:description" content="Review tables listing all TUI slash commands, default keyboard shortcuts, command-line flags, and JSON configuration parameters."/><script type="application/ld+json">{"@context":"https://schema.org","@graph":[{"@type":"TechArticle","headline":"CLI commands - Antigravity CLI","description":"Review tables listing all TUI slash commands, default keyboard shortcuts, command-line flags, and JSON configuration parameters.","url":"https://antigravity.google/docs/cli/reference/","mainEntityOfPage":{"@type":"WebPage","@id":"https://antigravity.google/docs/cli/reference/"},"author":{"@type":"Organization","name":"Google Antigravity","url":"https://antigravity.google/"},"publisher":{"@type":"Organization","name":"Google","url":"https://www.google.com/"}},{"@type":"BreadcrumbList","itemListElement":[{"@type":"ListItem","position":1,"name":"Docs","item":"https://antigravity.google/docs/overview/"},{"@type":"ListItem","position":2,"name":"Antigravity CLI","item":"https://antigravity.google/docs/cli/overview/"},{"@type":"ListItem","position":3,"name":"CLI commands","item":"https://antigravity.google/docs/cli/reference/"}]}]}</script>\<script\>
  window.dataLayer = window.dataLayer || [];
  window.__agyGtmLoaded = window.__agyGtmLoaded || false;
  window.__agyLastPageViewPath = window.__agyLastPageViewPath || '';

  var __agySegmentMql =
    typeof window !== 'undefined' && window.matchMedia
      ? window.matchMedia(
          '(horizontal-viewport-segments: 2), (vertical-viewport-segments: 2)'
        )
      : null;
  var __agyUa =
    typeof navigator !== 'undefined' ? navigator.userAgent || '' : '';
  var __agyIsFoldableUA =
    /Pixel Fold|SM-F9|SM-F7|Galaxy Z Fold|Surface Duo/i.test(__agyUa);
  var __agyIsIpad =
    /ipad/i.test(__agyUa) ||
    (typeof navigator !== 'undefined' &&
      navigator.maxTouchPoints > 0 &&
      /macintosh/i.test(__agyUa));
  var __agyOs = 'unknown';
  if (/iphone|ipod/i.test(__agyUa)) __agyOs = 'iphone';
  else if (__agyIsIpad) __agyOs = 'ipad';
  else if (/googlebook|cros/i.test(__agyUa)) __agyOs = 'googlebook';
  else if (/android/i.test(__agyUa)) __agyOs = 'android';
  else if (/windows/i.test(__agyUa)) __agyOs = 'windows';
  else if (/macintosh|mac os x/i.test(__agyUa)) __agyOs = 'mac';
  else if (/linux/i.test(__agyUa)) __agyOs = 'linux';

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
      Boolean(__agySegmentMql && __agySegmentMql.matches);
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

    let viewportCategory = 'desktop';
    if (
      hasSegments ||
      __agyIsFoldableUA ||
      isFoldableUnfolded ||
      isFoldableCover
    ) {
      viewportCategory = 'foldable';
    } else if (width < 600) {
      viewportCategory = 'mobile';
    } else if (width <= 1024 || __agyIsIpad) {
      viewportCategory = 'tablet';
    }

    return {
      viewportCategory: viewportCategory,
      formFactor: viewportCategory,
      viewportWidth: width,
      viewportHeight: height,
      orientation: orientation,
      operatingSystem: __agyOs,
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
      locale: document.documentElement.lang || 'en-US',
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
</script><script type="module" src="/_astro/AnalyticsBootstrap.astro_astro_type_script_index_0_lang.ucmpGtgL.js"></script><script defer src="https://www.gstatic.com/glue/cookienotificationbar/cookienotificationbar.min.js" data-glue-cookie-notification-bar-category="2A" data-glue-cookie-notification-bar-language="" data-glue-cookie-notification-bar-site-id=""></script>\<script\>
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
</script><template id="theme-icons"><svg aria-hidden="true" class="light astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1em;"><path d="M5 12a1 1 0 0 0-1-1H3a1 1 0 0 0 0 2h1a1 1 0 0 0 1-1Zm.64 5-.71.71a1 1 0 0 0 0 1.41 1 1 0 0 0 1.41 0l.71-.71A1 1 0 0 0 5.64 17ZM12 5a1 1 0 0 0 1-1V3a1 1 0 0 0-2 0v1a1 1 0 0 0 1 1Zm5.66 2.34a1 1 0 0 0 .7-.29l.71-.71a1 1 0 1 0-1.41-1.41l-.66.71a1 1 0 0 0 0 1.41 1 1 0 0 0 .66.29Zm-12-.29a1 1 0 0 0 1.41 0 1 1 0 0 0 0-1.41l-.71-.71a1.004 1.004 0 1 0-1.43 1.41l.73.71ZM21 11h-1a1 1 0 0 0 0 2h1a1 1 0 0 0 0-2Zm-2.64 6A1 1 0 0 0 17 18.36l.71.71a1 1 0 0 0 1.41 0 1 1 0 0 0 0-1.41l-.76-.66ZM12 6.5a5.5 5.5 0 1 0 5.5 5.5A5.51 5.51 0 0 0 12 6.5Zm0 9a3.5 3.5 0 1 1 0-7 3.5 3.5 0 0 1 0 7Zm0 3.5a1 1 0 0 0-1 1v1a1 1 0 0 0 2 0v-1a1 1 0 0 0-1-1Z"/></svg><svg aria-hidden="true" class="dark astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1em;"><path d="M21.64 13a1 1 0 0 0-1.05-.14 8.049 8.049 0 0 1-3.37.73 8.15 8.15 0 0 1-8.14-8.1 8.59 8.59 0 0 1 .25-2A1 1 0 0 0 8 2.36a10.14 10.14 0 1 0 14 11.69 1 1 0 0 0-.36-1.05Zm-9.5 6.69A8.14 8.14 0 0 1 7.08 5.22v.27a10.15 10.15 0 0 0 10.14 10.14 9.784 9.784 0 0 0 2.1-.22 8.11 8.11 0 0 1-7.18 4.32v-.04Z"/></svg><svg aria-hidden="true" class="auto astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1em;"><path d="M21 14h-1V7a3 3 0 0 0-3-3H7a3 3 0 0 0-3 3v7H3a1 1 0 0 0-1 1v2a3 3 0 0 0 3 3h14a3 3 0 0 0 3-3v-2a1 1 0 0 0-1-1ZM6 7a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v7H6V7Zm14 10a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1v-1h16v1Z"/></svg></template><link rel="stylesheet" href="/_astro/print.ehPL0gv-.css" media="print"><link rel="stylesheet" href="/_astro/index.BfSTt1zY.css"><link rel="stylesheet" href="/_astro/SharedHeadBootstrap.Ca7GHXLM.css">\<style\>@layer starlight.components{svg:where(.astro-n5k4ipy7){color:var(--sl-icon-color);font-size:var(--sl-icon-size,1em);width:1em;height:1em}}
</style><link rel="stylesheet" href="/_astro/AnnouncementPill.ubgfVsjK.css">\<style\>.button{font-size:var(--base-size);line-height:var(--cta-line-height);letter-spacing:var(--cta-letter-spacing);border-radius:var(--shape-corner-rounded,999px);cursor:pointer;text-align:center;border:1px solid #0000;justify-content:center;align-self:flex-start;align-items:center;height:fit-content;padding:10px 24px;font-family:Google Sans Flex,sans-serif;font-weight:450;text-decoration:none;transition:all .15s ease-out;display:inline-flex}.button span{text-decoration:none}.button .icon{color:currentColor;margin:0 var(--space-sm) 0 0;flex-shrink:0;align-items:center;line-height:0;display:inline-flex}.button:disabled{cursor:not-allowed}.button-primary{background:var(--theme-primary-primary,#1a73e8);color:var(--theme-primary-on-primary,#fff)}.button-primary:hover,.button-primary:focus-visible{background:var(--theme-button-states-primary-hovered,#1557b0);outline:none}.button-primary:disabled{background:var(--theme-button-states-disabled);color:var(--theme-button-states-on-disabled)}.button-primary-inverse{background:var(--palette-grey-0);color:var(--palette-grey-1200)}.button-primary-inverse:hover,.button-primary-inverse:focus-visible{background:var(--theme-surface-surface-container-high);outline:none}.button-primary-inverse:active{background:var(--theme-surface-surface-container-high)}.button-primary-inverse:disabled{background:var(--theme-button-states-disabled);color:var(--theme-button-states-on-disabled)}.button-secondary{color:var(--theme-surface-on-surface);background:var(--theme-secondary-button);backdrop-filter:blur(6px);border:1px solid var(--theme-outline-variant)}.button-secondary:hover,.button-secondary:focus-visible{background:var(--theme-button-secondary-hover);outline:none}.button-secondary:disabled{background:var(--theme-button-states-disabled);color:var(--theme-button-states-on-disabled)}.button-secondary-inverse{color:var(--theme-surface-surface);background:var(--theme-nav-button-hover);backdrop-filter:blur(8px);border:1px solid var(--theme-inverse-outline-variant)}.button-secondary-inverse:hover,.button-secondary-inverse:focus-visible{background:var(--theme-button-secondary-inverse-hover);outline:none}.button-secondary-inverse:disabled{background:var(--palette-grey-1100);color:var(--palette-grey-50-20)}.button-text,.button-nav{background:0 0}.button-text{color:var(--theme-text-on-text)}.button-text:hover,.button-text:focus-visible{background:var(--theme-button-states-hovered);outline:none}.button-text:active{background:var(--theme-button-states-pressed)}.button-text:disabled{color:var(--theme-button-states-on-disabled)}.button-compact{min-block-size:36px;padding:6px 16px}.download-button,.button-nav{font-size:var(--cta-sm-size);line-height:var(--cta-sm-line-height);letter-spacing:var(--cta-sm-letter-spacing);font-weight:450}.button-nav{color:var(--theme-surface-on-surface-variant)}.button-nav:hover,.button-nav:focus-visible{color:#000;background:var(--theme-nav-button-hover);outline:none}@media (width<=767px){.button-nav{width:100%}}.dropdown-nav{padding-right:8px}.dropdown-icon,.play-icon{letter-spacing:normal;text-transform:none;white-space:nowrap;word-wrap:normal;-webkit-font-smoothing:antialiased;text-rendering:optimizelegibility;font-feature-settings:"liga";-webkit-font-feature-settings:"liga";font-variation-settings:"FILL" 0, "wght" 300, "GRAD" 0, "ROND" 50, "opsz" 48;margin:0 0 0 var(--space-xs);transition:transform .3s all;direction:ltr;flex-shrink:0;justify-content:center;align-items:center;font-family:Google Symbols;font-size:1.5em;font-weight:300;line-height:1;display:inline-flex}.dropdown-icon svg,.play-icon svg{fill:currentColor;display:block}.button.dropdown-open .dropdown-icon,.button[aria-expanded=true] .dropdown-icon,.dropdown-open>.dropdown-icon{transform:rotate(-180deg)}.play-icon{font-variation-settings:"FILL" 1, "wght" 300, "GRAD" 0, "ROND" 50, "opsz" 48}@media (width<=1024px){.nav-button{padding:var(--space-md);width:100%}}
</style><script type="module" src="/_astro/page.LAbJoB63.js"></script></head><body class="astro-vdxcvpsm"><a class="sl-skip-link astro-vxeigawq" href="#_top">Skip to content</a><div class="page sl-flex astro-gqpr5s5a"><header class="header astro-gqpr5s5a"><div class="header-stack astro-mtusqnhg" data-analytics-nav="docs_header"><!-- Row 1: Logo and Search / Controls --><div class="header-main astro-mtusqnhg"><div class="title-wrapper sl-flex astro-mtusqnhg"><a href="/" class="site-title sl-flex astro-clw6ags5"><img class="light:sl-hidden print:hidden astro-clw6ags5" alt src="/_astro/antigravity_product_lockup_dark.YcYY_PSr.svg" width="1687" height="217"><img class="dark:sl-hidden print:block astro-clw6ags5" alt src="/_astro/antigravity_product_lockup_full_color.cUD_Se6a.svg" width="1687" height="217"><span class="sr-only astro-clw6ags5" translate="no">Google Antigravity Docs</span></a></div><div class="sl-flex search-wrapper print:hidden astro-mtusqnhg"><site-search id="doc-search-root" class="agy-hybrid-search" data-i18n-ignore="true" data-index-version="60cca000"><button type="button" data-open-modal id="doc-search-trigger" class="agy-search-trigger" aria-label="Search documentation" aria-keyshortcuts="Control+K Meta+K /"><span class="agy-search-trigger__left"><svg class="agy-search-trigger__icon" aria-hidden="true" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg><span class="agy-search-trigger__text">Search<span class="agy-search-trigger__text-suffix"> docs...</span></span></span><kbd class="agy-search-trigger__kbd" aria-hidden="true">⌘K</kbd></button><dialog id="doc-search-modal" class="agy-search-dialog" aria-label="Search"><div class="dialog-frame sl-flex"><div id="starlight__search" class="agy-search-container"><div class="agy-search-header"><div class="agy-search-input-wrap"><svg class="agy-search-input-icon" aria-hidden="true" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg><input id="doc-search-input" class="pagefind-ui__search-input" type="search" placeholder="Search documentation..." autocomplete="off" autocorrect="off" autocapitalize="off" spellcheck="false" aria-label="Search documentation" aria-controls="doc-search-results-list"><button type="button" class="agy-search-clear" data-clear-search hidden aria-label="Clear search query" title="Clear search"><svg aria-hidden="true" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><line x1="15" y1="9" x2="9" y2="15"></line><line x1="9" y1="9" x2="15" y2="15"></line></svg></button></div><div class="agy-search-header__actions"><button type="button" class="agy-shortcuts-toggle agy-search-help-btn" data-toggle-shortcuts aria-expanded="false" aria-controls="doc-search-shortcuts-panel" aria-label="Keyboard shortcuts" title="Keyboard shortcuts"><span aria-hidden="true">?</span></button><button type="button" data-close-modal class="agy-search-close" aria-label="Close search"><span class="agy-search-close__kbd" aria-hidden="true">ESC</span><span class="agy-search-close__touch" aria-hidden="true">Cancel</span></button></div></div><div class="agy-search-filters" role="toolbar" aria-label="Filter results by product surface"><button type="button" class="agy-surface-pill active" data-surface-filter="all" data-surface-label="All" aria-pressed="true"><span class="agy-surface-pill__label">All</span></button><button type="button" class="agy-surface-pill" data-surface-filter="2-0" data-surface-label="Antigravity App" aria-pressed="false"><span class="agy-surface-pill__label">App</span><span class="agy-surface-pill__mute" data-exclude-surface="2-0" aria-hidden="true" title="Exclude Antigravity App from results">×</span></button><button type="button" class="agy-surface-pill" data-surface-filter="cli" data-surface-label="Antigravity CLI" aria-pressed="false"><span class="agy-surface-pill__label">CLI</span><span class="agy-surface-pill__mute" data-exclude-surface="cli" aria-hidden="true" title="Exclude Antigravity CLI from results">×</span></button><button type="button" class="agy-surface-pill" data-surface-filter="antigravity-ide" data-surface-label="Antigravity IDE" aria-pressed="false"><span class="agy-surface-pill__label">IDE</span><span class="agy-surface-pill__mute" data-exclude-surface="antigravity-ide" aria-hidden="true" title="Exclude Antigravity IDE from results">×</span></button><button type="button" class="agy-surface-pill" data-surface-filter="ide" data-surface-label="IDE Extensions" aria-pressed="false"><span class="agy-surface-pill__label">Extensions</span><span class="agy-surface-pill__mute" data-exclude-surface="ide" aria-hidden="true" title="Exclude IDE Extensions from results">×</span></button><button type="button" class="agy-surface-pill" data-surface-filter="sdk" data-surface-label="Antigravity SDK" aria-pressed="false"><span class="agy-surface-pill__label">SDK</span><span class="agy-surface-pill__mute" data-exclude-surface="sdk" aria-hidden="true" title="Exclude Antigravity SDK from results">×</span></button><button type="button" class="agy-surface-pill" data-surface-filter="blog" data-surface-label="Blog" aria-pressed="false"><span class="agy-surface-pill__label">Blog</span><span class="agy-surface-pill__mute" data-exclude-surface="blog" aria-hidden="true" title="Exclude Blog from results">×</span></button><button type="button" class="agy-surface-pill" data-surface-filter="releases" data-surface-label="Releases" aria-pressed="false"><span class="agy-surface-pill__label">Releases</span><span class="agy-surface-pill__mute" data-exclude-surface="releases" aria-hidden="true" title="Exclude Releases from results">×</span></button></div><div id="doc-search-status" class="agy-search-status" data-search-status data-i18n-ignore="true" role="status" aria-live="polite" aria-atomic="true" hidden></div><div id="doc-search-results" class="pagefind-ui__results-area agy-search-results-area" data-results-container data-i18n-ignore="true"><div class="agy-search-suggestions" data-search-suggestions><p class="agy-search-suggestions__heading" data-discovery-heading>Explore Antigravity</p><div class="agy-discover-grid" data-discovery-grid><a class="agy-discover-card agy-search-card" data-doc-card="getting-started" href="/docs/getting-started/"><span class="agy-discover-card__body"><span class="agy-discover-card__title agy-search-card-title">Getting Started</span><span class="agy-discover-card__desc agy-search-card-desc">Install & launch your first agent</span></span><span class="agy-discover-card__arrow" aria-hidden="true">→</span></a><a class="agy-discover-card agy-search-card" data-doc-card="remote-control" href="/docs/remote-control/"><span class="agy-discover-card__body"><span class="agy-discover-card__title agy-search-card-title">Remote Control</span><span class="agy-discover-card__desc agy-search-card-desc">Drive desktop & CLI sessions from any browser</span></span><span class="agy-discover-card__arrow" aria-hidden="true">→</span></a><a class="agy-discover-card agy-search-card" data-doc-card="skills" href="/docs/skills/"><span class="agy-discover-card__body"><span class="agy-discover-card__title agy-search-card-title">Agent Skills & Rules</span><span class="agy-discover-card__desc agy-search-card-desc">Reusable workflows & workspace instructions</span></span><span class="agy-discover-card__arrow" aria-hidden="true">→</span></a><a class="agy-discover-card agy-search-card" data-doc-card="subagents" href="/docs/subagents/"><span class="agy-discover-card__body"><span class="agy-discover-card__title agy-search-card-title">Subagents & Teamwork</span><span class="agy-discover-card__desc agy-search-card-desc">Parallel multi-agent orchestration</span></span><span class="agy-discover-card__arrow" aria-hidden="true">→</span></a><a class="agy-discover-card agy-search-card" data-doc-card="sdk" href="/docs/sdk/overview/"><span class="agy-discover-card__body"><span class="agy-discover-card__title agy-search-card-title">Antigravity SDK</span><span class="agy-discover-card__desc agy-search-card-desc">Build custom Python agents & hooks</span></span><span class="agy-discover-card__arrow" aria-hidden="true">→</span></a><a class="agy-discover-card agy-search-card" data-doc-card="permissions" href="/docs/permissions/"><span class="agy-discover-card__body"><span class="agy-discover-card__title agy-search-card-title">Permissions & Sandbox</span><span class="agy-discover-card__desc agy-search-card-desc">File access controls & safe execution</span></span><span class="agy-discover-card__arrow" aria-hidden="true">→</span></a></div><div class="agy-try-searching"><span class="agy-try-searching__label" data-try-searching-label>Try searching</span><div class="agy-try-searching__chips" data-suggestion-chips><button type="button" class="agy-suggestion-chip agy-search-chip" data-search-suggestion="Remote Control" data-query="Remote Control">Remote Control</button><button type="button" class="agy-suggestion-chip agy-search-chip" data-search-suggestion="agent skills" data-query="agent skills">Agent skills</button><button type="button" class="agy-suggestion-chip agy-search-chip" data-search-suggestion="custom subagents" data-query="custom subagents">Custom subagents</button><button type="button" class="agy-suggestion-chip agy-search-chip" data-search-suggestion="IDE extensions" data-query="IDE extensions">IDE extensions</button><button type="button" class="agy-suggestion-chip agy-search-chip" data-search-suggestion="sandbox permissions" data-query="sandbox permissions">Sandbox permissions</button></div></div></div><ol id="doc-search-results-list" class="pagefind-ui__results agy-search-results-list" data-results-list data-i18n-ignore="true"></ol></div><div class="agy-related-topics" data-related-topics data-i18n-ignore="true" hidden></div><div id="doc-search-shortcuts-panel" class="agy-search-footer agy-search-help-popover" data-shortcuts-panel hidden><div class="agy-search-footer__hints"><span class="agy-footer-hint">\<kbd\>↑</kbd>\<kbd\>↓</kbd><span data-footer-hint="navigate">Navigate</span></span><span class="agy-footer-hint">\<kbd\>↵</kbd> <span data-footer-hint="open">Open</span></span><span class="agy-footer-hint">\<kbd\>esc</kbd> <span data-footer-hint="close">Close</span></span></div></div></div></div></dialog></site-search><script type="module" src="/_astro/SearchOverride.astro_astro_type_script_index_0_lang.DyJlY0iY.js"></script></div><div class="sl-hidden md:sl-flex print:hidden right-group astro-mtusqnhg"><div class="header-cta-group astro-mtusqnhg"><a class="top-nav-btn remote-control-btn astro-mtusqnhg" href="https://antigravity.google.com" target="_blank" rel="noopener noreferrer" title="Launch Remote Control"><span class="btn-icon astro-mtusqnhg" aria-hidden="true">🚀</span> Remote Control</a><a href="/download" class="call-to-action--nav button button-primary button-compact download-button dropdown-nav" aria-haspopup="false" aria-expanded="false" data-analytics-event="download-cta" data-module-name="header" data-page-position="main nav" class="astro-mtusqnhg"><span>Download</span><span class="dropdown-icon" translate="no"><svg class viewBox="0 0 24 24" width="18" height="18" fill="currentColor" aria-hidden="true" xmlns="http://www.w3.org/2000/svg"><path d="M12 16L7 11L8.4 9.55L11 12.15V4H13V12.15L15.6 9.55L17 11L12 16ZM6 20C5.45 20 4.979 19.804 4.588 19.413C4.196 19.021 4.00067 18.55 4 18V15H6V18H18V15H20V18C20 18.55 19.804 19.021 19.413 19.413C19.021 19.804 18.55 20 18 20H6Z"></path></svg></span></a><script type="module">var e=/^\d+\.\d+\.\d+$/,t=/^\d+$/;function n(n){let r=Array.isArray(n)?n:n&&typeof n==`object`&&Array.isArray(n.versions)?n.versions:null;if(!r||r.length===0)return null;let i=r[0];if(!i||typeof i!=`object`)return null;let{version:a,execution_id:o}=i;return typeof a!=`string`||!e.test(a)||typeof o!=`string`||!t.test(o)?null:{version:a,executionId:o}}function r(e,t,n){let r=`https://storage.googleapis.com/antigravity-public/antigravity-hub/${e}-${t}`;return n===`darwin-arm`||n===`darwin-x64`?`${r}/${n}/Antigravity.dmg`:n===`windows-x64`?`${r}/windows-x64/${e===`2.0.0`?`Antigravity.exe`:`Antigravity-x64.exe`}`:n===`windows-arm64`?`${r}/windows-arm/${e===`2.0.0`?`Antigravity.exe`:`Antigravity-arm64.exe`}`:n===`linux-x64`?`${r}/linux-x64/Antigravity.tar.gz`:n===`linux-arm`?`${r}/linux-arm/Antigravity.tar.gz`:`${r}/${n}/Antigravity.dmg`}var i=null,a=null;async function o(){return i&&Date.now()-i.fetchedAt<3e5?i.build:a||(a=(async()=>{let e=new AbortController,t=setTimeout(()=>e.abort(),8e3);try{let t=await fetch(`https://antigravity-hub-auto-updater-974169037036.us-central1.run.app/releases`,{cache:`no-store`,signal:e.signal});if(!t.ok)return null;let r=n(await t.json());return r&&(i={build:r,fetchedAt:Date.now()}),r}catch{return null}finally{clearTimeout(t),a=null}})(),a)}function s(e,t=document){t.querySelectorAll(`a[data-hub-platform]`).forEach(t=>{let n=t.getAttribute(`data-hub-platform`);n&&(t.href=r(e.version,e.executionId,n),t.setAttribute(`data-hub-version`,e.version))}),t.querySelectorAll(`[data-hub-latest-version]`).forEach(t=>{t.textContent=`v${e.version}`})}async function c(){if(!document.querySelector(`a[data-hub-platform]`))return;let e=await o();e&&s(e)}var l=!1;function u(){document.readyState===`loading`?document.addEventListener(`DOMContentLoaded`,()=>{c()}):c(),l||(l=!0,document.addEventListener(`astro:page-load`,()=>{c()}),window.addEventListener(`agy:page-ready`,()=>{c()}))}u();</script></div><div class="sl-flex social-icons astro-mtusqnhg"></div><starlight-theme-select><label style="--sl-select-width: 6.25em" class="astro-r2p65idu"><span class="sr-only astro-r2p65idu">Select theme</span><svg aria-hidden="true" class="icon label-icon astro-r2p65idu astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1em;"><path d="M21 14h-1V7a3 3 0 0 0-3-3H7a3 3 0 0 0-3 3v7H3a1 1 0 0 0-1 1v2a3 3 0 0 0 3 3h14a3 3 0 0 0 3-3v-2a1 1 0 0 0-1-1ZM6 7a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v7H6V7Zm14 10a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1v-1h16v1Z"/></svg><select autocomplete="off" class="astro-r2p65idu"><option value="dark" class="astro-r2p65idu">Dark</option><option value="light" class="astro-r2p65idu">Light</option><option value="auto" selected class="astro-r2p65idu">Auto</option></select><svg aria-hidden="true" class="icon caret astro-r2p65idu astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1em;"><path d="M17 9.17a1 1 0 0 0-1.41 0L12 12.71 8.46 9.17a1 1 0 1 0-1.41 1.42l4.24 4.24a1.002 1.002 0 0 0 1.42 0L17 10.59a1.002 1.002 0 0 0 0-1.42Z"/></svg></label></starlight-theme-select>\<script\>
	StarlightThemeProvider.updatePickers();
</script><script type="module">var e=`starlight-theme`,t=e=>e===`auto`||e===`dark`||e===`light`?e:`auto`,n=()=>t(typeof localStorage<`u`&&localStorage.getItem(e));function r(t){typeof localStorage<`u`&&localStorage.setItem(e,t===`light`||t===`dark`?t:``)}var i=()=>matchMedia(`(prefers-color-scheme: light)`).matches?`light`:`dark`;function a(e){StarlightThemeProvider.updatePickers(e),document.documentElement.dataset.theme=e===`auto`?i():e,r(e)}matchMedia(`(prefers-color-scheme: light)`).addEventListener(`change`,()=>{n()===`auto`&&a(`auto`)});var o=class extends HTMLElement{constructor(){super(),a(n()),this.querySelector(`select`)?.addEventListener(`change`,e=>{e.currentTarget instanceof HTMLSelectElement&&a(t(e.currentTarget.value))})}};customElements.define(`starlight-theme-select`,o);</script></div><div class="sl-flex print:hidden docs-lang-wrapper astro-mtusqnhg"><div class="language-selector docs-language-selector astro-mtusqnhg astro-fe4j42sm" data-language-selector data-custom-select-wrapper="language" data-custom-select-open="false" tabindex="0" role="combobox" aria-haspopup="listbox" aria-expanded="false" aria-label="Select language"><span class="language-selector__icon astro-fe4j42sm" aria-hidden="true" translate="no"><svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" class="astro-fe4j42sm"><circle cx="12" cy="12" r="10" class="astro-fe4j42sm"></circle><path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20" class="astro-fe4j42sm"></path><path d="M2 12h20" class="astro-fe4j42sm"></path></svg></span><span class="language-selector__code astro-fe4j42sm" data-language-code aria-hidden="true" translate="no">EN</span><span class="language-selector__chevron astro-fe4j42sm" aria-hidden="true" translate="no"><svg xmlns="http://www.w3.org/2000/svg" width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round" class="astro-fe4j42sm"><polyline points="6 9 12 15 18 9" class="astro-fe4j42sm"></polyline></svg></span><select class="language-selector__select astro-fe4j42sm" data-language-select tabindex="-1" aria-label="Select language" title="English (United States)"><option value="en-US" selected class="astro-fe4j42sm">English (United States)</option><option value="id" class="astro-fe4j42sm">Bahasa Indonesia (Indonesia)</option><option value="de" class="astro-fe4j42sm">Deutsch (Deutschland)</option><option value="en-CA" class="astro-fe4j42sm">English (Canada)</option><option value="en-GB" class="astro-fe4j42sm">English (United Kingdom)</option><option value="es-ES" class="astro-fe4j42sm">Español (España)</option><option value="es-MX" class="astro-fe4j42sm">Español (México)</option><option value="fr-FR" class="astro-fe4j42sm">Français (France)</option><option value="it" class="astro-fe4j42sm">Italiano (Italia)</option><option value="pl" class="astro-fe4j42sm">Polski (Polska)</option><option value="pt-BR" class="astro-fe4j42sm">Português (Brasil)</option><option value="hi" class="astro-fe4j42sm">हिन्दी (भारत)</option><option value="th" class="astro-fe4j42sm">ไทย (ประเทศไทย)</option><option value="ja" class="astro-fe4j42sm">日本語 (日本)</option><option value="zh-CN" class="astro-fe4j42sm">简体中文 (中国)</option><option value="zh-TW" class="astro-fe4j42sm">繁體中文 (台灣)</option><option value="ko" class="astro-fe4j42sm">한국어 (대한민국)</option></select></div>\<script\>
  (function () {
    try {
      var valid = [
        'en-US',
        'id',
        'de',
        'en-CA',
        'en-GB',
        'es-ES',
        'es-MX',
        'fr-FR',
        'it',
        'pl',
        'pt-BR',
        'hi',
        'th',
        'ja',
        'zh-CN',
        'zh-TW',
        'ko',
      ];
      var shortLabels = {
        'en-US': 'EN',
        'en-GB': 'EN-GB',
        'en-CA': 'EN-CA',
        hi: 'HI',
        ja: 'JA',
        ko: 'KO',
        'zh-CN': 'ZH-CN',
        'zh-TW': 'ZH-TW',
        id: 'ID',
        th: 'TH',
        de: 'DE',
        'fr-FR': 'FR',
        it: 'IT',
        pl: 'PL',
        'pt-BR': 'PT',
        'es-ES': 'ES',
        'es-MX': 'ES-MX',
      };
      var active = window.__agyActiveLocale;
      if (!active) {
        var stored = window.localStorage.getItem('antigravity_locale');
        active = 'en-US';
        if (stored !== null) {
          if (valid.indexOf(stored) !== -1) {
            active = stored;
          } else {
            window.localStorage.setItem('antigravity_locale', 'en-US');
            active = 'en-US';
          }
        } else {
          var langs =
            navigator.languages && navigator.languages.length
              ? navigator.languages
              : [navigator.language || 'en-US'];
          for (var i = 0; i < langs.length; i++) {
            var raw = String(langs[i] || '')
              .trim()
              .replace(/_/g, '-');
            if (!raw) continue;
            var lower = raw.toLowerCase();
            var exactIdx = -1;
            for (var j = 0; j < valid.length; j++) {
              if (valid[j].toLowerCase() === lower) {
                exactIdx = j;
                break;
              }
            }
            if (exactIdx !== -1) {
              active = valid[exactIdx];
              break;
            }
            var parts = lower.split('-');
            var base = parts[0];
            var subtag = parts[1] || '';
            var region = parts[2] || subtag;
            if (base === 'zh') {
              active =
                subtag === 'tw' ||
                subtag === 'hk' ||
                subtag === 'mo' ||
                subtag === 'hant' ||
                region === 'tw' ||
                region === 'hk' ||
                region === 'mo'
                  ? 'zh-TW'
                  : 'zh-CN';
              break;
            }
            if (base === 'id' || base === 'in') {
              active = 'id';
              break;
            }
            if (base === 'th') {
              active = 'th';
              break;
            }
            if (base === 'it') {
              active = 'it';
              break;
            }
            if (base === 'pl') {
              active = 'pl';
              break;
            }
            if (base === 'es') {
              active = subtag === 'es' ? 'es-ES' : 'es-MX';
              break;
            }
            if (base === 'pt') {
              active = 'pt-BR';
              break;
            }
            if (base === 'fr') {
              active = 'fr-FR';
              break;
            }
            if (base === 'de') {
              active = 'de';
              break;
            }
            if (base === 'ja') {
              active = 'ja';
              break;
            }
            if (base === 'ko') {
              active = 'ko';
              break;
            }
            if (base === 'hi') {
              active = 'hi';
              break;
            }
            if (base === 'en') {
              active =
                subtag === 'gb' ||
                subtag === 'uk' ||
                subtag === 'au' ||
                subtag === 'nz' ||
                subtag === 'ie'
                  ? 'en-GB'
                  : subtag === 'ca'
                    ? 'en-CA'
                    : 'en-US';
              break;
            }
          }
          if (active !== 'en-US') {
            window.localStorage.setItem('antigravity_locale', active);
            window.dataLayer = window.dataLayer || [];
            window.dataLayer.push({
              event: 'language_auto_detect',
              eventParams: {
                locale: active,
                browserLanguage: langs[0] || '',
                pagePath: window.location.pathname,
              },
            });
            window.dataLayer.push({ eventParams: undefined });
          }
        }
        window.__agyActiveLocale = active;
      }
      if (document.documentElement.lang !== active) {
        document.documentElement.lang = active;
      }
      var selects = document.querySelectorAll('select[data-language-select]');
      for (var k = 0; k < selects.length; k++) {
        if (selects[k].value !== active) {
          selects[k].value = active;
        }
      }
      var codes = document.querySelectorAll('[data-language-code]');
      var shortCode = shortLabels[active] || 'EN';
      for (var m = 0; m < codes.length; m++) {
        if (codes[m].textContent !== shortCode) {
          codes[m].textContent = shortCode;
        }
      }
    } catch (e) {
      document.documentElement.lang = 'en-US';
    }
  })();
</script><script type="module" src="/_astro/LanguageSelector.astro_astro_type_script_index_0_lang.B8alFdNs.js"></script></div></div><!-- Row 2: Subnav Bar below logo and controls (Get Started, Features, etc.) --><nav class="subnav-bar print:hidden astro-mtusqnhg" aria-label="Documentation sections"><div class="subnav-container astro-mtusqnhg"><div class="subnav-left astro-mtusqnhg"><a href="/docs/overview/" class="subnav-pill active astro-mtusqnhg" aria-current="page">Get Started</a><a href="/docs/features/" class="subnav-pill astro-mtusqnhg">Features</a><a href="/docs/sdk/overview/" class="subnav-pill astro-mtusqnhg">SDK</a></div><div class="subnav-right astro-mtusqnhg"><a href="/docs/changelog/" class="subnav-pill astro-mtusqnhg">Changelog</a><a href="/blog/" class="subnav-pill external-link blog-link astro-mtusqnhg">Blog <span class="arrow astro-mtusqnhg" aria-hidden="true">↗</span></a></div></div></nav><!-- Row 3: Dismissible Top Banners under Docs Menu --><div class="docs-top-banner-slot print:hidden astro-mtusqnhg" data-docs-announcement-slot="top-banner"><script type="module" src="/_astro/AnnouncementPill.astro_astro_type_script_index_0_lang.BbFE-USi.js"></script><div class="announcement-pill announcement-pill--surface-docs announcement-pill--host-docs announcement-pill--placement-top-banner announcement-pill--dark is-interactive astro-mtusqnhg astro-pyiv6lj6" role="region" aria-label="New announcement" data-announcement-pill data-pill-surface="docs" data-pill-host-surface="docs" data-pill-placement="top-banner" data-pill-variant="dark" data-pill-icon-name="spark" data-pill-badge="New" data-cta-text="Learn more" data-destination="/blog/gemini-4-argon-in-google-antigravity" data-pill-storage-key="agy_announcement_dismissed_docs" data-pill-fingerprint="New|Gemini 4 Argon available for Google AI Pro and Ultra in Google Antigravity|/blog/gemini-4-argon-in-google-antigravity" data-pill-ttl-ms="604800000" data-analytics-event="announcement-pill" data-module-name="docs_top_banner" data-page-position="top-banner"><a href="/blog/gemini-4-argon-in-google-antigravity" class="pill-main-link astro-pyiv6lj6" data-pill-link><span class="pill-badge astro-pyiv6lj6" data-pill-badge><span class="pill-icon astro-pyiv6lj6" data-pill-icon aria-hidden="true"><svg class="pill-spark-icon astro-pyiv6lj6" viewBox="0 0 16 16" width="12" height="12" aria-hidden="true"><path d="M8 0.5C8.5 4.6 11.4 7.5 15.5 8C11.4 8.5 8.5 11.4 8 15.5C7.5 11.4 4.6 8.5 0.5 8C4.6 7.5 7.5 4.6 8 0.5Z" fill="currentColor" class="astro-pyiv6lj6"></path></svg></span><span class="pill-badge-label astro-pyiv6lj6" data-pill-badge-label>New</span></span><span class="pill-text astro-pyiv6lj6" data-pill-text>Gemini 4 Argon available for Google AI Pro and Ultra in Google Antigravity</span><span class="pill-cta astro-pyiv6lj6" data-pill-cta><span class="pill-cta-label astro-pyiv6lj6" data-pill-cta-label>Learn more</span><svg class="pill-arrow astro-pyiv6lj6" viewBox="0 0 16 16" width="13" height="13" fill="none" aria-hidden="true"><path d="M3.5 8H12.5M12.5 8L8.5 4M12.5 8L8.5 12" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" class="astro-pyiv6lj6"></path></svg></span></a><button type="button" class="pill-dismiss astro-pyiv6lj6" data-pill-dismiss data-analytics-event="" aria-label="Dismiss New announcement" title="Dismiss announcement"><svg viewBox="0 0 16 16" width="14" height="14" fill="none" aria-hidden="true" class="astro-pyiv6lj6"><path d="M4 4L12 12M12 4L4 12" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" class="astro-pyiv6lj6"></path></svg></button></div>\<script\>
        (() => {
          try {
            const pills = document.querySelectorAll(
              '[data-announcement-pill]:not([data-prechecked])',
            );
            pills.forEach((pill) => {
              pill.setAttribute('data-prechecked', 'true');
              const key = pill.getAttribute('data-pill-storage-key');
              const fp = pill.getAttribute('data-pill-fingerprint');
              if (!key) return;
              const raw = window.localStorage.getItem(key);
              if (!raw) return;
              const parsed = JSON.parse(raw);
              if (
                parsed &&
                parsed.fingerprint === fp &&
                typeof parsed.expiresAt === 'number' &&
                Date.now() < parsed.expiresAt
              ) {
                pill.hidden = true;
                pill.setAttribute('data-pill-dismissed', 'true');
              }
            });

            const homeSlot = document.querySelector('[data-home-banner-slot]');
            if (homeSlot) {
              const visibleHome = homeSlot.querySelectorAll(
                '[data-announcement-pill]:not([hidden])',
              );
              if (visibleHome.length === 0) {
                document.documentElement.style.setProperty(
                  '--home-banner-offset',
                  '0px',
                );
              } else {
                const slotHeightPx = Math.ceil(
                  homeSlot.getBoundingClientRect().height,
                );
                if (slotHeightPx > 0) {
                  document.documentElement.style.setProperty(
                    '--home-banner-offset',
                    `${slotHeightPx}px`,
                  );
                }
              }
            }

            const docsSlot = document.querySelector(
              '[data-docs-announcement-slot="top-banner"]',
            );
            const docsHeader = document.querySelector('header.header');
            if (docsSlot && docsHeader) {
              const visibleDocs = docsSlot.querySelectorAll(
                '[data-announcement-pill]:not([hidden])',
              );
              if (visibleDocs.length === 0) {
                document.documentElement.style.removeProperty(
                  '--sl-nav-height',
                );
              } else {
                const headerHeightPx = Math.ceil(
                  docsHeader.getBoundingClientRect().height,
                );
                if (headerHeightPx > 0) {
                  document.documentElement.style.setProperty(
                    '--sl-nav-height',
                    `${headerHeightPx}px`,
                    'important',
                  );
                }
              }
            }
          } catch {}
        })();
      </script></div></div></header><nav class="sidebar print:hidden astro-gqpr5s5a" aria-label="Main"><starlight-menu-button class="print:hidden astro-vvx7mffu"><button aria-expanded="false" aria-label="Menu" aria-controls="starlight__sidebar" class="sl-flex md:sl-hidden astro-vvx7mffu"><svg aria-hidden="true" class="open-menu astro-vvx7mffu astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1em;"><path d="M3 8h18a1 1 0 1 0 0-2H3a1 1 0 0 0 0 2Zm18 8H3a1 1 0 0 0 0 2h18a1 1 0 0 0 0-2Zm0-5H3a1 1 0 0 0 0 2h18a1 1 0 0 0 0-2Z"/></svg><svg aria-hidden="true" class="close-menu astro-vvx7mffu astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1em;"><path d="m13.41 12 6.3-6.29a1.004 1.004 0 1 0-1.42-1.42L12 10.59l-6.29-6.3a1.004 1.004 0 0 0-1.42 1.42l6.3 6.29-6.3 6.29a1 1 0 0 0 0 1.42.998.998 0 0 0 1.42 0l6.29-6.3 6.29 6.3a.999.999 0 0 0 1.42 0 1 1 0 0 0 0-1.42L13.41 12Z"/></svg></button></starlight-menu-button><script type="module">var e=class extends HTMLElement{btn=this.querySelector(`button`);constructor(){super(),this.btn.addEventListener(`click`,()=>this.toggleExpanded());let e=this.closest(`nav`);e&&e.addEventListener(`keyup`,e=>this.closeOnEscape(e)),matchMedia(`(min-width: 50em)`).addEventListener(`change`,()=>this.setExpanded(!1))}setExpanded(e){this.setAttribute(`aria-expanded`,String(e)),document.body.toggleAttribute(`data-mobile-menu-expanded`,e),document.querySelectorAll(`.main-frame, .sl-skip-link`).forEach(t=>{t.toggleAttribute(`inert`,e)})}toggleExpanded(){this.setExpanded(this.getAttribute(`aria-expanded`)!==`true`)}closeOnEscape(e){e.code===`Escape`&&(this.setExpanded(!1),this.btn.focus())}};customElements.define(`starlight-menu-button`,e);</script><div id="starlight__sidebar" class="sidebar-pane astro-gqpr5s5a"><div class="sidebar-content sl-flex astro-gqpr5s5a"><div class="sidebar-wrapper astro-s3sbtxwf" data-default-section="get-started" data-analytics-nav="docs_sidebar"><sl-sidebar-state-persist data-hash="1m4v1wn" class="astro-f5yvicm7"><script aria-hidden="true">
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
	</script><div class="sidebar-sections is-active astro-s3sbtxwf" data-sidebar-section="get-started"><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">Get Started</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/overview/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/getting-started/" class="astro-ojxuodky"><span class="astro-ojxuodky">Quickstart</span></a></li><li class="astro-ojxuodky"><a href="/docs/install/" class="astro-ojxuodky"><span class="astro-ojxuodky">Authenticate</span></a></li><li class="astro-ojxuodky"><a href="/docs/models/" class="astro-ojxuodky"><span class="astro-ojxuodky">Choose a model</span></a></li><li class="astro-ojxuodky"><a href="/docs/build-with-google/" class="astro-ojxuodky"><span class="astro-ojxuodky">Build with Google</span></a></li></ul></section><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">IDE Extensions</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/ide/extensions/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/extensions/vscode/" class="astro-ojxuodky"><span class="astro-ojxuodky">Visual Studio Code</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/extensions/visual-studio/" class="astro-ojxuodky"><span class="astro-ojxuodky">Visual Studio</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/extensions/jetbrains/" class="astro-ojxuodky"><span class="astro-ojxuodky">JetBrains IDEs</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/extensions/zed/" class="astro-ojxuodky"><span class="astro-ojxuodky">Zed</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/extensions/xcode/" class="astro-ojxuodky"><span class="astro-ojxuodky">Xcode</span></a></li></ul></section><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">CLI reference</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/cli/reference/" aria-current="page" class="astro-ojxuodky"><span class="astro-ojxuodky">CLI commands</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/features/" class="astro-ojxuodky"><span class="astro-ojxuodky">CLI features</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/using/" class="astro-ojxuodky"><span class="astro-ojxuodky">CLI settings and tips</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/troubleshooting/" class="astro-ojxuodky"><span class="astro-ojxuodky">CLI troubleshooting</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/best-practices/" class="astro-ojxuodky"><span class="astro-ojxuodky">CLI best practices</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/credits/" class="astro-ojxuodky"><span class="astro-ojxuodky">CLI AI credits</span></a></li></ul></section><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">Migrations</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/cli/gcli-migration/" class="astro-ojxuodky"><span class="astro-ojxuodky">Gemini CLI</span></a></li><li class="astro-ojxuodky"><a href="/docs/firebase-studio-migration/" class="astro-ojxuodky"><span class="astro-ojxuodky">Firebase Studio</span></a></li><li class="astro-ojxuodky"><a href="/docs/migration/workflows-to-skills/" class="astro-ojxuodky"><span class="astro-ojxuodky">Workflows to Skills</span></a></li></ul></section><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">Resources</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/plans/" class="astro-ojxuodky"><span class="astro-ojxuodky">Plans</span></a></li><li class="astro-ojxuodky"><a href="/docs/enterprise/" class="astro-ojxuodky"><span class="astro-ojxuodky">Enterprise Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/faq/" class="astro-ojxuodky"><span class="astro-ojxuodky">FAQ</span></a></li></ul></section></div><div class="sidebar-sections astro-s3sbtxwf" data-sidebar-section="features"><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">LEARN</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/features/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/remote-control/" class="astro-ojxuodky"><span class="astro-ojxuodky">Remote Control</span></a></li><li class="astro-ojxuodky"><a href="/docs/marketplace/" class="astro-ojxuodky"><span class="astro-ojxuodky">Marketplace</span></a></li><li class="astro-ojxuodky"><a href="/docs/plugins/" class="astro-ojxuodky"><span class="astro-ojxuodky">Plugins</span></a></li><li class="astro-ojxuodky"><a href="/docs/sandbox/" class="astro-ojxuodky"><span class="astro-ojxuodky">Terminal sandbox</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/review-changes-editor/" class="astro-ojxuodky"><span class="astro-ojxuodky">Review changes</span></a></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Terminal</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="0"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/cli/modes/" class="astro-ojxuodky"><span class="astro-ojxuodky">Execution modes</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/headless/" class="astro-ojxuodky"><span class="astro-ojxuodky">Headless mode</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/statusline/" class="astro-ojxuodky"><span class="astro-ojxuodky">Status line customization</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/title/" class="astro-ojxuodky"><span class="astro-ojxuodky">Terminal title customization</span></a></li></ul></details></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Browser</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="1"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/ide/browser/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/browser-recordings/" class="astro-ojxuodky"><span class="astro-ojxuodky">Browser recordings</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/allowlist-denylist/" class="astro-ojxuodky"><span class="astro-ojxuodky">Allowlist and denylist</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/separate-chrome-profile/" class="astro-ojxuodky"><span class="astro-ojxuodky">Separate Chrome profile</span></a></li></ul></details></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Artifacts</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="2"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/artifacts/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/implementation-plan/" class="astro-ojxuodky"><span class="astro-ojxuodky">Implementation plan</span></a></li><li class="astro-ojxuodky"><a href="/docs/walkthrough/" class="astro-ojxuodky"><span class="astro-ojxuodky">Walkthrough</span></a></li><li class="astro-ojxuodky"><a href="/docs/screenshots/" class="astro-ojxuodky"><span class="astro-ojxuodky">Visual screenshots</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/artifacts/" class="astro-ojxuodky"><span class="astro-ojxuodky">Reviewing artifacts (CLI)</span></a></li></ul></details></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Projects</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="3"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/projects/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/prompting/" class="astro-ojxuodky"><span class="astro-ojxuodky">Prompting and interaction</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/conversations/" class="astro-ojxuodky"><span class="astro-ojxuodky">Managing conversations</span></a></li></ul></details></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Permissions</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="4"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/permissions/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li></ul></details></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Editor features</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="5"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/ide/tab/" class="astro-ojxuodky"><span class="astro-ojxuodky">Tab completion</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/agent-side-panel/" class="astro-ojxuodky"><span class="astro-ojxuodky">Agent side panel</span></a></li><li class="astro-ojxuodky"><a href="/docs/agent/" class="astro-ojxuodky"><span class="astro-ojxuodky">Agent overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/tools/" class="astro-ojxuodky"><span class="astro-ojxuodky">Task groups</span></a></li></ul></details></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Settings</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="6"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/settings/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/agent-settings/" class="astro-ojxuodky"><span class="astro-ojxuodky">Agent settings</span></a></li><li class="astro-ojxuodky"><a href="/docs/artifact-review/" class="astro-ojxuodky"><span class="astro-ojxuodky">Artifact review</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/vim-editor-mode/" class="astro-ojxuodky"><span class="astro-ojxuodky">Vim editor mode</span></a></li></ul></details></li></ul></section><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">SLASH COMMANDS</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/slash-commands/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/agents/" class="astro-ojxuodky"><span class="astro-ojxuodky">/agents</span></a></li><li class="astro-ojxuodky"><a href="/docs/boost/" class="astro-ojxuodky"><span class="astro-ojxuodky">/boost</span></a></li><li class="astro-ojxuodky"><a href="/docs/commands/browser/" class="astro-ojxuodky"><span class="astro-ojxuodky">/browser</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/btw/" class="astro-ojxuodky"><span class="astro-ojxuodky">/btw</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/changelog/" class="astro-ojxuodky"><span class="astro-ojxuodky">/changelog</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/codesearch/" class="astro-ojxuodky"><span class="astro-ojxuodky">/codesearch</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/context/" class="astro-ojxuodky"><span class="astro-ojxuodky">/context</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/credits/" class="astro-ojxuodky"><span class="astro-ojxuodky">/credits</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/diff/" class="astro-ojxuodky"><span class="astro-ojxuodky">/diff</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/effort/" class="astro-ojxuodky"><span class="astro-ojxuodky">/effort</span></a></li><li class="astro-ojxuodky"><a href="/docs/commands/goal/" class="astro-ojxuodky"><span class="astro-ojxuodky">/goal</span></a></li><li class="astro-ojxuodky"><a href="/docs/commands/grill-me/" class="astro-ojxuodky"><span class="astro-ojxuodky">/grill-me</span></a></li><li class="astro-ojxuodky"><a href="/docs/commands/learn/" class="astro-ojxuodky"><span class="astro-ojxuodky">/learn</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/permissions/" class="astro-ojxuodky"><span class="astro-ojxuodky">/permissions</span></a></li><li class="astro-ojxuodky"><a href="/docs/plan/" class="astro-ojxuodky"><span class="astro-ojxuodky">/plan</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/resume/" class="astro-ojxuodky"><span class="astro-ojxuodky">/resume</span></a></li><li class="astro-ojxuodky"><a href="/docs/commands/schedule/" class="astro-ojxuodky"><span class="astro-ojxuodky">/schedule</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/statusline/" class="astro-ojxuodky"><span class="astro-ojxuodky">/statusline</span></a></li><li class="astro-ojxuodky"><a href="/docs/teamwork/" class="astro-ojxuodky"><span class="astro-ojxuodky">/teamwork-preview</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/title/" class="astro-ojxuodky"><span class="astro-ojxuodky">/title</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/usage/" class="astro-ojxuodky"><span class="astro-ojxuodky">/usage</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/voice/" class="astro-ojxuodky"><span class="astro-ojxuodky">/voice</span></a></li></ul></section><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">EXTEND</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/mcp/" class="astro-ojxuodky"><span class="astro-ojxuodky">MCP</span></a></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Skills</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="7"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/skills/" class="astro-ojxuodky"><span class="astro-ojxuodky">Agent skills</span></a></li><li class="astro-ojxuodky"><a href="/docs/rules/" class="astro-ojxuodky"><span class="astro-ojxuodky">Rules</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/workflows/" class="astro-ojxuodky"><span class="astro-ojxuodky">IDE workflows</span></a></li></ul></details></li><li class="astro-ojxuodky"><a href="/docs/subagents/" class="astro-ojxuodky"><span class="astro-ojxuodky">Custom subagents</span></a></li><li class="astro-ojxuodky"><a href="/docs/hooks/" class="astro-ojxuodky"><span class="astro-ojxuodky">Lifecycle hooks</span></a></li><li class="astro-ojxuodky"><a href="/docs/sidecars/" class="astro-ojxuodky"><span class="astro-ojxuodky">Sidecars</span></a></li></ul></section></div><div class="sidebar-sections astro-s3sbtxwf" data-sidebar-section="sdk"><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">ANTIGRAVITY SDK</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/sdk/overview/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview and quickstart</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/local-models/" class="astro-ojxuodky"><span class="astro-ojxuodky">Local models</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/personas/" class="astro-ojxuodky"><span class="astro-ojxuodky">Personas</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/tools/" class="astro-ojxuodky"><span class="astro-ojxuodky">Tools and skills</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/mcp/" class="astro-ojxuodky"><span class="astro-ojxuodky">MCP</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/policies/" class="astro-ojxuodky"><span class="astro-ojxuodky">Policies</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/subagents/" class="astro-ojxuodky"><span class="astro-ojxuodky">Subagents</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/structured-output/" class="astro-ojxuodky"><span class="astro-ojxuodky">Structured output</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/lifecycle/" class="astro-ojxuodky"><span class="astro-ojxuodky">Lifecycle and hooks</span></a></li></ul></section></div><div class="sidebar-sections astro-s3sbtxwf" data-sidebar-section="changelog"></div><script aria-hidden="true">
		(() => {
			const scroller = document.getElementById('starlight__sidebar');
			if (!window._starlightScrollRestore || !scroller) return;
			scroller.scrollTop = window._starlightScrollRestore;
			delete window._starlightScrollRestore;
		})();
	</script></sl-sidebar-state-persist><div class="md:sl-hidden astro-s3sbtxwf"><div class="mobile-preferences sl-flex astro-b5gddk54"><div class="social-icons astro-b5gddk54"></div><starlight-theme-select><label style="--sl-select-width: 6.25em" class="astro-r2p65idu"><span class="sr-only astro-r2p65idu">Select theme</span><svg aria-hidden="true" class="icon label-icon astro-r2p65idu astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1em;"><path d="M21 14h-1V7a3 3 0 0 0-3-3H7a3 3 0 0 0-3 3v7H3a1 1 0 0 0-1 1v2a3 3 0 0 0 3 3h14a3 3 0 0 0 3-3v-2a1 1 0 0 0-1-1ZM6 7a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v7H6V7Zm14 10a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1v-1h16v1Z"/></svg><select autocomplete="off" class="astro-r2p65idu"><option value="dark" class="astro-r2p65idu">Dark</option><option value="light" class="astro-r2p65idu">Light</option><option value="auto" selected class="astro-r2p65idu">Auto</option></select><svg aria-hidden="true" class="icon caret astro-r2p65idu astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1em;"><path d="M17 9.17a1 1 0 0 0-1.41 0L12 12.71 8.46 9.17a1 1 0 1 0-1.41 1.42l4.24 4.24a1.002 1.002 0 0 0 1.42 0L17 10.59a1.002 1.002 0 0 0 0-1.42Z"/></svg></label></starlight-theme-select>\<script\>
	StarlightThemeProvider.updatePickers();
</script><script type="module">var e=class extends HTMLElement{constructor(){super();let e=this.querySelector(`select`);e&&(e.addEventListener(`change`,e=>{e.currentTarget instanceof HTMLSelectElement&&(window.location.pathname=e.currentTarget.value)}),window.addEventListener(`pageshow`,t=>{if(!t.persisted)return;let n=e.querySelector(`option[selected]`)?.index;n!==e.selectedIndex&&(e.selectedIndex=n??0)}))}};customElements.define(`starlight-lang-select`,e);</script></div></div></div></div></div></nav><div class="main-frame astro-gqpr5s5a"><script type="module">var e=document.getElementById(`starlight__sidebar`),t=e?.querySelector(`sl-sidebar-state-persist`),n=`sl-sidebar-state`,r=()=>{let r=[],i=t?.dataset.hash||``;try{let e=sessionStorage.getItem(n),t=JSON.parse(e||`{}`);Array.isArray(t.open)&&t.hash===i&&(r=t.open)}catch{}return{hash:i,open:r,scroll:e?.scrollTop||0}},i=e=>{try{sessionStorage.setItem(n,JSON.stringify(e))}catch{}},a=()=>i(r()),o=(e,t)=>{let n=r();n.open[t]=e,i(n)};t?.addEventListener(`click`,e=>{if(!(e.target instanceof Element))return;let t=e.target.closest(`summary`)?.closest(`details`);if(!t)return;let n=t.querySelector(`sl-sidebar-restore`),r=parseInt(n?.dataset.index||``);isNaN(r)||o(!t.open,r)}),addEventListener(`visibilitychange`,()=>{document.visibilityState===`hidden`&&a()});</script><div class="lg:sl-flex astro-abfeecmu"><aside class="right-sidebar-container print:hidden astro-abfeecmu"><div class="right-sidebar astro-abfeecmu"><div class="lg:sl-hidden astro-q5mffni3"><mobile-starlight-toc data-min-h="2" data-max-h="3" class="astro-sgnutmzh"><nav aria-labelledby="starlight__on-this-page--mobile" data-analytics-nav="docs_toc" class="astro-sgnutmzh"><details id="starlight__mobile-toc" class="astro-sgnutmzh"><summary id="starlight__on-this-page--mobile" class="sl-flex astro-sgnutmzh"><span class="toggle sl-flex astro-sgnutmzh">On this page<svg aria-hidden="true" class="caret astro-sgnutmzh" width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="
             m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1
             1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002
             1.002 0 0 0 0-1.42Z
            " class="astro-sgnutmzh"></path></svg></span><span class="display-current astro-sgnutmzh"></span></summary><div class="dropdown astro-sgnutmzh"><ul class="isMobile astro-sgnutmzh"><li class="astro-sgnutmzh"><a href="#_top" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Overview</span></a></li><li class="astro-sgnutmzh"><a href="#core-slash-commands" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Core slash commands</span></a></li><li class="astro-sgnutmzh"><a href="#default-keybindings" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Default keybindings</span></a><ul class="astro-sgnutmzh"><li class="astro-sgnutmzh"><a href="#global-controls" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Global controls</span></a></li><li class="astro-sgnutmzh"><a href="#prompt-focus-keys" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Prompt focus keys</span></a></li><li class="astro-sgnutmzh"><a href="#navigation-and-scrolling" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Navigation and scrolling</span></a></li><li class="astro-sgnutmzh"><a href="#tool-confirmations" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Tool confirmations</span></a></li></ul></li><li class="astro-sgnutmzh"><a href="#command-line-flags" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Command-line flags</span></a></li><li class="astro-sgnutmzh"><a href="#configuration-keys-settingsjson" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Configuration keys (settings.json)</span></a><ul class="astro-sgnutmzh"><li class="astro-sgnutmzh"><a href="#example-settingsjson" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Example settings.json</span></a></li></ul></li><li class="astro-sgnutmzh"><a href="#next-steps" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Next steps</span></a></li></ul></div></details></nav></mobile-starlight-toc><script type="module" src="/_astro/MobileTableOfContentsOverride.astro_astro_type_script_index_0_lang.BhmNkhRh.js"></script></div><div class="right-sidebar-panel sl-hidden lg:sl-block astro-q5mffni3"><div class="sl-container astro-q5mffni3"><starlight-toc data-min-h="2" data-max-h="3"><nav aria-labelledby="starlight__on-this-page" data-analytics-nav="docs_toc"><h2 id="starlight__on-this-page">On this page</h2><ul class="starlight-toc-list">\<li\><a href="#_top"><span>Overview</span></a></li>\<li\><a href="#core-slash-commands"><span>Core slash commands</span></a></li>\<li\><a href="#default-keybindings"><span>Default keybindings</span></a>\<ul\>\<li\><a href="#global-controls"><span>Global controls</span></a></li>\<li\><a href="#prompt-focus-keys"><span>Prompt focus keys</span></a></li>\<li\><a href="#navigation-and-scrolling"><span>Navigation and scrolling</span></a></li>\<li\><a href="#tool-confirmations"><span>Tool confirmations</span></a></li></ul></li>\<li\><a href="#command-line-flags"><span>Command-line flags</span></a></li>\<li\><a href="#configuration-keys-settingsjson"><span>Configuration keys (settings.json)</span></a>\<ul\>\<li\><a href="#example-settingsjson"><span>Example settings.json</span></a></li></ul></li>\<li\><a href="#next-steps"><span>Next steps</span></a></li></ul></nav></starlight-toc><script type="module">var e=class extends HTMLElement{_current=null;_onScroll=null;_observer=null;_cleanup=[];set current(e){e!==this._current&&(this._current&&this._current.removeAttribute(`aria-current`),e.setAttribute(`aria-current`,`true`),this._current=e)}connectedCallback(){this.init(),window.addEventListener(`agy:page-ready`,this.reinit)}disconnectedCallback(){window.removeEventListener(`agy:page-ready`,this.reinit),this.cleanup()}cleanup(){this._current=null,this._onScroll&&=(window.removeEventListener(`scroll`,this._onScroll),null),this._observer&&=(this._observer.disconnect(),null),this._cleanup.forEach(e=>e()),this._cleanup=[]}reinit=()=>{this.cleanup(),this.init()};init=()=>{let e=Array.from(this.querySelectorAll(`a`));if(!e.length)return;let t=[],n=new Map,r=document.getElementById(`_top`);if(r){let i=e.find(e=>e.hash===`#_top`);if(i){n.set(`_top`,i);let e=i.closest(`li`);e&&t.push({link:i,li:e,heading:r,tabpanel:null,surfacePane:null})}}e.forEach(e=>{let r=decodeURIComponent(e.hash.slice(1));if(!r||r===`_top`)return;let i=document.getElementById(r),a=e.closest(`li`);if(i&&a){n.set(r,e);let o=i.closest(`[role="tabpanel"]`),s=i.closest(`.surface-pane`);t.push({link:e,li:a,heading:i,tabpanel:o,surfacePane:s})}});let i=e=>e?e.hidden||e.hasAttribute(`hidden`)||e.style.display===`none`:!1,a=e=>{if(!e)return!1;if(e.hidden||e.hasAttribute(`hidden`))return!0;if(e.style.display)return e.style.display===`none`;let t=e.getAttribute(`data-surface-pane`);return t?t!==`hub`:getComputedStyle(e).display===`none`},o=!1,s=()=>{o||(o=!0,requestAnimationFrame(c))},c=()=>{if(o=!1,window.innerWidth<800||this.offsetParent===null)return;let e=t.filter(e=>e.heading.id===`_top`||!e.heading.hidden&&e.heading.style.display!==`none`&&!i(e.tabpanel)&&!a(e.surfacePane));if(!e.length)return;let r=e[0];if(window.scrollY>10)for(let t of e)if(t.heading.getBoundingClientRect().top<=130)r=t;else break;let s=n.get(r.heading.id);s&&(this.current=s)},l=(e=!1)=>{window.innerWidth<800||(t.forEach(e=>{let t=i(e.tabpanel)||a(e.surfacePane),n=t?`none`:``;e.li.style.display!==n&&(e.li.style.display=n),t?e.li.getAttribute(`data-tab-hidden`)!==`true`&&e.li.setAttribute(`data-tab-hidden`,`true`):e.li.hasAttribute(`data-tab-hidden`)&&e.li.removeAttribute(`data-tab-hidden`)}),e?c():s())},u=e=>{let t=e.target?.closest(`a`);if(!t||!this.contains(t))return;let n=decodeURIComponent(t.hash.slice(1));if(!n)return;let r=document.getElementById(n);if(r){let e=r.closest(`.surface-pane[data-surface-pane]`);if(e&&a(e)){let t=e.getAttribute(`data-surface-pane`);t&&(document.querySelector(`.surface-card-btn[data-surface-id="${t}"]`)?.click(),requestAnimationFrame(()=>{l()}))}let t=r.closest(`starlight-tabs [role="tabpanel"]`);if(t){let e=t.closest(`starlight-tabs`);if(e){let n=Array.from(e.querySelectorAll(`:scope > [role="tabpanel"]`)).indexOf(t);n!==-1&&(e.querySelectorAll(`[role="tab"]`)[n]?.click(),requestAnimationFrame(()=>{l()}))}}}};this.addEventListener(`click`,u),this._cleanup.push(()=>{this.removeEventListener(`click`,u)});let d=document.querySelectorAll(`[role="tabpanel"], .surface-pane`);if(d.length>0){let e=0,t,n=()=>{e||=requestAnimationFrame(()=>{e=0,clearTimeout(t),t=setTimeout(()=>{l()},0)})};this._observer=new MutationObserver(n),d.forEach(e=>{this._observer?.observe(e,{attributes:!0,attributeFilter:[`hidden`,`style`,`class`]})}),document.addEventListener(`ag-surface-changed`,n),this._cleanup.push(()=>{document.removeEventListener(`ag-surface-changed`,n)})}this._onScroll=s,window.addEventListener(`scroll`,this._onScroll,{passive:!0}),l(!0)}};customElements.get(`starlight-toc`)||customElements.define(`starlight-toc`,e);</script></div></div></div></aside><div class="main-pane astro-abfeecmu"><main data-pagefind-body class="astro-vdxcvpsm" lang="en" dir="ltr"><div class="content-panel astro-oil7k7jf"><div class="sl-container astro-oil7k7jf"><div class="pagefind-search-meta sr-only astro-hkwnjoai" data-i18n-ignore="true"><span data-pagefind-meta="title" class="astro-hkwnjoai">CLI commands - Antigravity CLI</span><span data-pagefind-meta="surface" class="astro-hkwnjoai">cli</span><span data-pagefind-meta="surfaceLabel" class="astro-hkwnjoai">Antigravity CLI</span><span data-pagefind-meta="section" class="astro-hkwnjoai">Get Started</span><span data-pagefind-weight="6" class="astro-hkwnjoai">CLI commands - Antigravity CLI Antigravity CLI Get Started</span><span data-pagefind-filter="surface:cli" class="astro-hkwnjoai">Antigravity CLI</span></div><div class="breadcrumbs-row astro-hkwnjoai" data-pagefind-ignore><nav aria-label="Breadcrumb" class="starlight-breadcrumbs astro-hkwnjoai"><span class="breadcrumb-item  astro-hkwnjoai" title="Get Started">Get Started</span><svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" class="breadcrumb-separator astro-hkwnjoai" aria-hidden="true"><polyline points="9 18 15 12 9 6" class="astro-hkwnjoai"></polyline></svg><span class="breadcrumb-item  astro-hkwnjoai" title="CLI reference">CLI reference</span><svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" class="breadcrumb-separator astro-hkwnjoai" aria-hidden="true"><polyline points="9 18 15 12 9 6" class="astro-hkwnjoai"></polyline></svg><span class="breadcrumb-item breadcrumb-last astro-hkwnjoai" title="CLI commands">CLI commands</span></nav><div id="md-actions-root" class="markdown-dropdown-wrapper astro-pyacvpuh" data-pathname="/docs/cli/reference/"><button type="button" class="call-to-action--nav button button-primary button-compact markdown-trigger-btn dropdown-nav" aria-haspopup="true" aria-expanded="false" aria-label="Markdown Options" class="astro-pyacvpuh"><span>Markdown</span><span class="dropdown-icon" translate="no"><svg class viewBox="0 0 24 24" width="18" height="18" fill="currentColor" aria-hidden="true" xmlns="http://www.w3.org/2000/svg"><path d="M12 15L7.05 10.05L8.45 8.65L12 12.2L15.55 8.65L16.95 10.05L12 15Z"></path></svg></span></button><div class="markdown-dropdown-menu astro-pyacvpuh" data-md-menu hidden><button type="button" class="markdown-menu-item astro-pyacvpuh" data-copy-md><svg class="icon astro-pyacvpuh" viewBox="0 -960 960 960" width="16px" height="16px" fill="currentColor" aria-hidden="true" xmlns="http://www.w3.org/2000/svg"><path d="M318.46-230.77q-23.53,0-40.61-17.08t-17.08-40.61V-802.31q0-23.53 17.08-40.61T318.46-860H712.31q23.53,0 40.61,17.08T770-802.31v513.84q0,23.53-17.08,40.61t-40.61,17.08H318.46Zm0-45.38H712.31q4.62,0 8.46-3.85t3.85-8.46V-802.31q0-4.62-3.85-8.46t-8.46-3.85H318.46q-4.62,0-8.46,3.85t-3.85,8.46v513.84q0,4.62 3.85,8.46t8.46,3.85ZM207.69-120q-23.53,0-40.61-17.08T150-177.7V-736.92h45.38V-177.7q0,4.62 3.85,8.46t8.46,3.85H646.92V-120H207.69Zm98.46-156.15q0,0 0-3.85t0-8.46V-802.31q0-4.62 0-8.46t0-3.85q0,0 0,3.85t0,8.46v513.84q0,4.62 0,8.46t0,3.85Z"></path></svg><span class="item-label astro-pyacvpuh">Copy Markdown</span></button><a href="#" data-view-md-link target="_blank" rel="noopener noreferrer" class="markdown-menu-item astro-pyacvpuh"><svg class="icon astro-pyacvpuh" viewBox="0 -960 960 960" width="16px" height="16px" fill="currentColor" aria-hidden="true" xmlns="http://www.w3.org/2000/svg"><path d="M197.69-140q-23.53,0-40.61-17.08T140-197.69V-762.31q0-23.53 17.08-40.61T197.69-820H449.38v45.38H197.69q-4.62,0-8.46,3.85t-3.85,8.46v564.61q0,4.62 3.85,8.46t8.46,3.85H762.31q4.62,0 8.46-3.85t3.85-8.46V-449.38H820v251.69q0,23.53-17.08,40.61T762.31-140H197.69ZM384.31-351.69l-32-32.61L742.62-774.61H530.15V-820H820v289.84H774.61V-742L384.31-351.69Z"></path></svg><span class="item-label astro-pyacvpuh">View Markdown</span></a></div></div>\<script\>
  (() => {
    function setupMarkdownDropdown() {
      const wrappers = document.querySelectorAll('.markdown-dropdown-wrapper');
      wrappers.forEach((wrapper) => {
        const currentPath = (
          wrapper.dataset.pathname || window.location.pathname
        ).replace(/\/$/, '');
        const mdUrl = `${currentPath}.md`;

        const viewLink = wrapper.querySelector('[data-view-md-link]');
        if (viewLink) {
          viewLink.href = mdUrl;
        }

        if (wrapper.dataset.mdInitialized === 'true') return;
        wrapper.dataset.mdInitialized = 'true';

        const triggerBtn = wrapper.querySelector(
          '.markdown-trigger-btn, .button'
        );
        const menu = wrapper.querySelector('[data-md-menu]');
        const copyBtn = wrapper.querySelector('[data-copy-md]');

        if (!triggerBtn || !menu) return;

        const toggleMenu = (e) => {
          e.stopPropagation();
          const isHidden = menu.hasAttribute('hidden');
          if (isHidden) {
            menu.removeAttribute('hidden');
            triggerBtn.setAttribute('aria-expanded', 'true');
          } else {
            menu.setAttribute('hidden', '');
            triggerBtn.setAttribute('aria-expanded', 'false');
          }
        };

        triggerBtn.addEventListener('click', toggleMenu);

        if (copyBtn) {
          copyBtn.addEventListener('click', async (e) => {
            e.stopPropagation();
            const label = copyBtn.querySelector('.item-label');
            const originalText = label?.textContent || 'Copy Markdown';
            const activePath = (
              wrapper.dataset.pathname || window.location.pathname
            ).replace(/\/$/, '');
            const activeMdUrl = `${activePath}.md`;

            try {
              if (label) label.textContent = 'Fetching...';
              const response = await fetch(activeMdUrl);
              if (!response.ok) {
                throw new Error(`Failed to fetch markdown: ${response.status}`);
              }
              const htmlText = await response.text();
              const parsedDoc = new DOMParser().parseFromString(
                htmlText,
                'text/html'
              );
              const preElement = parsedDoc.querySelector('pre');
              const markdownText = preElement
                ? preElement.textContent || ''
                : htmlText;
              await navigator.clipboard.writeText(markdownText);

              if (label) {
                label.textContent = 'Copied!';
                setTimeout(() => {
                  label.textContent = originalText;
                  menu.setAttribute('hidden', '');
                  triggerBtn.setAttribute('aria-expanded', 'false');
                }, 1200);
              }
            } catch (err) {
              console.error('Error copying markdown:', err);
              if (label) label.textContent = 'Error copying';
              setTimeout(() => {
                if (label) label.textContent = originalText;
              }, 2000);
            }
          });
        }
      });
    }

    if (document.readyState === 'loading') {
      document.addEventListener('DOMContentLoaded', setupMarkdownDropdown, {
        once: true,
      });
    } else {
      setupMarkdownDropdown();
    }

    if (!window.__agyMarkdownActionsBound) {
      window.__agyMarkdownActionsBound = true;
      document.addEventListener(
        'click',
        (e) => {
          const openMenus = document.querySelectorAll(
            '.markdown-dropdown-wrapper [data-md-menu]:not([hidden])'
          );
          openMenus.forEach((menu) => {
            const wrapper = menu.closest('.markdown-dropdown-wrapper');
            if (wrapper && !wrapper.contains(e.target)) {
              menu.setAttribute('hidden', '');
              const triggerBtn = wrapper.querySelector(
                '.markdown-trigger-btn, .button'
              );
              if (triggerBtn) {
                triggerBtn.setAttribute('aria-expanded', 'false');
              }
            }
          });
        },
        { passive: true }
      );
      document.addEventListener('astro:page-load', setupMarkdownDropdown);
      window.addEventListener('agy:page-ready', setupMarkdownDropdown);
    }
  })();
</script></div><script type="module" src="/_astro/StarlightPageTitleOverride.astro_astro_type_script_index_0_lang.DMV6_9UE.js"></script></div></div><div class="content-panel astro-oil7k7jf"><div class="sl-container astro-oil7k7jf"><div class="sl-markdown-content"><div class="sl-heading-wrapper level-h1"><h1 id="cli-commands">CLI commands</h1><a class="sl-anchor-link" href="#cli-commands"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “CLI commands”</span></a></div>
<p>Review tables listing all TUI slash commands, default keyboard shortcuts, command-line flags, and JSON configuration parameters.</p>
<div class="sl-heading-wrapper level-h2"><h2 id="core-slash-commands">Core slash commands</h2><a class="sl-anchor-link" href="#core-slash-commands"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Core slash commands”</span></a></div>
<p>Type <code dir="auto">/</code> inside the prompt box to open the typeahead command selection menu.</p>













































































































































































































































































<table><thead><tr><th align="left">Command</th><th align="left">Category</th><th align="left">Alias</th><th align="left">Execution Purpose</th></tr></thead>\<tbody\><tr><td align="left"><strong><code dir="auto">/add-dir &#x3C;path></code></strong></td><td align="left">Utilities</td><td align="left">—</td><td align="left">Add a directory path to the active workspace.</td></tr><tr><td align="left"><strong><a href="/docs/cli/commands/agents"><code dir="auto">/agents</code></a></strong></td><td align="left">Tools &#x26; Tasks</td><td align="left">—</td><td align="left">Open the <a href="/docs/cli/commands/agents">Agent Manager panel</a> to switch custom agents and monitor background subagents.</td></tr><tr><td align="left"><strong><code dir="auto">/artifact</code></strong></td><td align="left">Tools &#x26; Tasks</td><td align="left">—</td><td align="left">Open the Artifact Review panel.</td></tr><tr><td align="left"><strong><a href="/docs/boost"><code dir="auto">/boost</code></a></strong> <code dir="auto">&#x3C;task></code></td><td align="left">Reasoning</td><td align="left">—</td><td align="left">Run on-demand multi-agent deep reasoning loops.</td></tr><tr><td align="left"><strong><a href="/docs/commands/browser"><code dir="auto">/browser</code></a></strong> <code dir="auto">&#x3C;task></code></td><td align="left">Tools &#x26; Tasks</td><td align="left">—</td><td align="left">Delegate web browsing, UI verification, and in-browser debugging to the <a href="/docs/commands/browser">browser subagent</a>.</td></tr><tr><td align="left"><strong><a href="/docs/cli/commands/btw"><code dir="auto">/btw</code></a> <code dir="auto">&#x3C;query></code></strong></td><td align="left">Utilities</td><td align="left">—</td><td align="left">Ask a <a href="/docs/cli/commands/btw">side question in the background</a> without interrupting the main conversation.</td></tr><tr><td align="left"><strong><a href="/docs/cli/commands/changelog"><code dir="auto">/changelog</code></a></strong></td><td align="left">Utilities</td><td align="left">—</td><td align="left">Open the <a href="/docs/cli/commands/changelog">interactive release notes viewer</a> or print CLI changes in headless mode.</td></tr><tr><td align="left"><strong><code dir="auto">/clear</code></strong></td><td align="left">Utilities</td><td align="left"><code dir="auto">/new</code></td><td align="left">Clear the terminal and reset active conversation contexts.</td></tr><tr><td align="left"><strong><a href="/docs/cli/commands/codesearch"><code dir="auto">/codesearch</code></a> <code dir="auto">&#x3C;query></code></strong></td><td align="left">Utilities</td><td align="left"><code dir="auto">/cs</code>, <code dir="auto">/search</code></td><td align="left">Open the <a href="/docs/cli/commands/codesearch">interactive Code Search panel</a> to search code across the workspace.</td></tr><tr><td align="left"><strong><code dir="auto">/config</code></strong></td><td align="left">Configurations</td><td align="left"><code dir="auto">/settings</code></td><td align="left">Open the interactive Settings Editor overlay.</td></tr><tr><td align="left"><strong><a href="/docs/cli/commands/context"><code dir="auto">/context</code></a></strong></td><td align="left">Utilities</td><td align="left">—</td><td align="left">Open the <a href="/docs/cli/commands/context">context usage visualization panel</a>.</td></tr><tr><td align="left"><strong><code dir="auto">/copy [n|btw]</code></strong></td><td align="left">Utilities</td><td align="left">—</td><td align="left">Copy the last agent response, the n-th most recent response, or the latest <a href="/docs/cli/commands/btw"><code dir="auto">/btw</code></a> answer.</td></tr><tr><td align="left"><strong><a href="/docs/cli/commands/credits"><code dir="auto">/credits</code></a></strong></td><td align="left">Account</td><td align="left">—</td><td align="left">View remaining G1 credits and purchase links.</td></tr><tr><td align="left"><strong><a href="/docs/cli/commands/diff"><code dir="auto">/diff</code></a></strong></td><td align="left">Utilities</td><td align="left">—</td><td align="left">Open the <a href="/docs/cli/commands/diff">interactive diff viewer</a> to view changes, turns, and commits.</td></tr><tr><td align="left"><strong><a href="/docs/cli/commands/effort"><code dir="auto">/effort</code></a> <code dir="auto">[level]</code></strong></td><td align="left">Configurations</td><td align="left">—</td><td align="left">Set or inspect the active model’s <a href="/docs/cli/commands/effort">reasoning effort level</a> (<code dir="auto">low</code> through <code dir="auto">max</code>, depending on the model).</td></tr><tr><td align="left"><strong><code dir="auto">/exit</code></strong></td><td align="left">Core</td><td align="left"><code dir="auto">/quit</code></td><td align="left">Close the TUI session and restore your host shell.</td></tr><tr><td align="left"><strong><code dir="auto">/feedback</code></strong></td><td align="left">Utilities</td><td align="left">—</td><td align="left">Open the feedback submission panel.</td></tr><tr><td align="left"><strong><code dir="auto">/fork</code></strong></td><td align="left">Conversations</td><td align="left"><code dir="auto">/branch</code></td><td align="left">Clone the current conversation thread into a new parallel session.</td></tr><tr><td align="left"><strong><a href="/docs/commands/goal"><code dir="auto">/goal</code></a></strong> <code dir="auto">&#x3C;objective></code></td><td align="left">Reasoning</td><td align="left">—</td><td align="left">Run autonomously until the <a href="/docs/commands/goal">goal is verified complete</a>, without intermediate pauses.</td></tr><tr><td align="left"><strong><a href="/docs/commands/grill-me"><code dir="auto">/grill-me</code></a></strong> <code dir="auto">[task]</code></td><td align="left">Planning</td><td align="left">—</td><td align="left">Have the agent <a href="/docs/commands/grill-me">interview you</a> one question at a time, with a recommended answer for each, before it starts work.</td></tr><tr><td align="left"><strong><code dir="auto">/help</code></strong></td><td align="left">Utilities</td><td align="left">—</td><td align="left">Open the help panel showing commands and shortcuts.</td></tr><tr><td align="left"><strong><code dir="auto">/hooks</code></strong></td><td align="left">Tools &#x26; Tasks</td><td align="left">—</td><td align="left">Browse active pre-flight/post-format script hooks.</td></tr><tr><td align="left"><strong><code dir="auto">/keybindings</code></strong></td><td align="left">Configurations</td><td align="left">—</td><td align="left">Open the interactive Keyboard Shortcut Editor.</td></tr><tr><td align="left"><strong><a href="/docs/commands/learn"><code dir="auto">/learn</code></a></strong> <code dir="auto">[instruction]</code></td><td align="left">Tools &#x26; Tasks</td><td align="left">—</td><td align="left">Capture session corrections as persistent <a href="/docs/commands/learn">rules or skills</a> through a reviewable proposal.</td></tr><tr><td align="left"><strong><code dir="auto">/logout</code></strong></td><td align="left">Account</td><td align="left">—</td><td align="left">Disconnect your profile and purge authentication tokens from the secure keyring.</td></tr><tr><td align="left"><strong><code dir="auto">/mcp</code></strong></td><td align="left">Tools &#x26; Tasks</td><td align="left">—</td><td align="left">Open the Model Context Protocol (MCP) server manager.</td></tr><tr><td align="left"><strong><code dir="auto">/model</code></strong></td><td align="left">Configurations</td><td align="left">—</td><td align="left">Choose your preferred reasoning model (persists across sessions).</td></tr><tr><td align="left"><strong><code dir="auto">/open &#x3C;path></code></strong></td><td align="left">Utilities</td><td align="left">—</td><td align="left">Force the path to open inside your default system editor.</td></tr><tr><td align="left"><strong><a href="/docs/cli/commands/permissions"><code dir="auto">/permissions</code></a></strong></td><td align="left">Configurations</td><td align="left">—</td><td align="left">Open the interactive tool permissions manager panel.</td></tr><tr><td align="left"><strong><a href="/docs/plan"><code dir="auto">/plan</code></a></strong> <code dir="auto">[task]</code></td><td align="left">Planning</td><td align="left">—</td><td align="left">Explore the codebase, clarify requirements, and draft a reviewable <a href="/docs/implementation-plan">Implementation Plan</a>.</td></tr><tr><td align="left"><strong><a href="/docs/plugins?tab=cli"><code dir="auto">/plugin</code></a></strong></td><td align="left">Tools &#x26; Tasks</td><td align="left"><code dir="auto">/plugins</code></td><td align="left">Open the interactive <a href="/docs/marketplace?tab=cli">Plugins Manager</a> or run inline <code dir="auto">install</code>, <code dir="auto">uninstall</code>, <code dir="auto">enable</code>, <code dir="auto">disable</code>, and <code dir="auto">list</code> commands.</td></tr><tr><td align="left"><strong><code dir="auto">/rename &#x3C;name></code></strong></td><td align="left">Conversations</td><td align="left">—</td><td align="left">Rename the current session thread.</td></tr><tr><td align="left"><strong><a href="/docs/remote-control?tab=cli#interactive-mode"><code dir="auto">/remote-control</code></a> [on/off]</strong></td><td align="left">Utilities</td><td align="left">—</td><td align="left">Toggle Remote Control for the active terminal session.</td></tr><tr><td align="left"><strong><a href="/docs/cli/commands/resume"><code dir="auto">/resume</code></a></strong></td><td align="left">Conversations</td><td align="left"><code dir="auto">/switch</code>, <code dir="auto">/conversation</code></td><td align="left">Open the <a href="/docs/cli/commands/resume">conversation picker overlay</a> to select and load previous threads.</td></tr><tr><td align="left"><strong><code dir="auto">/rewind</code></strong></td><td align="left">Conversations</td><td align="left"><code dir="auto">/undo</code></td><td align="left">Roll back your conversation history to a previous message.</td></tr><tr><td align="left"><strong><a href="/docs/commands/schedule"><code dir="auto">/schedule</code></a></strong> <code dir="auto">&#x3C;when> &#x3C;task></code></td><td align="left">Tools &#x26; Tasks</td><td align="left">—</td><td align="left">Set a <a href="/docs/commands/schedule">one-shot timer or recurring cron schedule</a> that wakes the agent with an instruction.</td></tr><tr><td align="left"><strong><code dir="auto">/skills</code></strong></td><td align="left">Tools &#x26; Tasks</td><td align="left">—</td><td align="left">Browse loaded local and global Agent Skills.</td></tr><tr><td align="left"><strong><a href="/docs/cli/commands/statusline"><code dir="auto">/statusline</code></a></strong></td><td align="left">Configurations</td><td align="left">—</td><td align="left">Open the Status Bar customization overlay.</td></tr><tr><td align="left"><strong><code dir="auto">/tasks</code></strong></td><td align="left">Tools &#x26; Tasks</td><td align="left">—</td><td align="left">Open the Task Manager panel to monitor background shell execution logs.</td></tr><tr><td align="left"><strong><a href="/docs/teamwork"><code dir="auto">/teamwork-preview</code></a></strong> <code dir="auto">&#x3C;task></code></td><td align="left">Reasoning</td><td align="left"><code dir="auto">/teamwork</code></td><td align="left">Launch <a href="/docs/teamwork">collaborative multi-agent teams</a> for long-horizon projects (paid plans).</td></tr><tr><td align="left"><strong><a href="/docs/cli/commands/title"><code dir="auto">/title</code></a> [on/off]</strong></td><td align="left">Configurations</td><td align="left">—</td><td align="left">Toggle or set terminal window title updates.</td></tr><tr><td align="left"><strong><a href="/docs/cli/commands/usage"><code dir="auto">/usage</code></a></strong></td><td align="left">Utilities</td><td align="left"><code dir="auto">/quota</code></td><td align="left">Display model quota usage.</td></tr><tr><td align="left"><strong><a href="/docs/cli/commands/voice"><code dir="auto">/voice</code></a></strong></td><td align="left">Utilities</td><td align="left"><code dir="auto">/record</code></td><td align="left">Dictate a prompt using your microphone.</td></tr></tbody></table>
<div class="sl-heading-wrapper level-h2"><h2 id="default-keybindings">Default keybindings</h2><a class="sl-anchor-link" href="#default-keybindings"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Default keybindings”</span></a></div>
<p>The following tables list the keyboard shortcut commands mapping global, prompt, navigation, and approval operations.</p>
<div class="sl-heading-wrapper level-h3"><h3 id="global-controls">Global controls</h3><a class="sl-anchor-link" href="#global-controls"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Global controls”</span></a></div>
<p>These hotkeys are always active regardless of which panel, overlay, or prompt is currently focused.</p>






























<table><thead><tr><th align="left">Key</th><th align="left">TUI Command</th><th align="left">Action Behavior</th></tr></thead>\<tbody\><tr><td align="left"><strong><code dir="auto">Esc</code></strong></td><td align="left"><code dir="auto">cli.escape</code></td><td align="left">Closes active panels, halts active streams, or clears empty prompts.</td></tr><tr><td align="left"><strong><code dir="auto">Ctrl+C</code></strong></td><td align="left"><code dir="auto">cli.exit</code></td><td align="left">Terminates the CLI session (prompts for confirmation if agent is working).</td></tr><tr><td align="left"><strong><code dir="auto">Ctrl+D</code></strong></td><td align="left"><code dir="auto">cli.exit</code></td><td align="left">Exits the CLI session (only when the prompt box is empty).</td></tr><tr><td align="left"><strong><code dir="auto">Ctrl+L</code></strong></td><td align="left"><code dir="auto">cli.clear_screen</code></td><td align="left">Refreshes and clears the visual terminal buffer.</td></tr></tbody></table>
<div class="sl-heading-wrapper level-h3"><h3 id="prompt-focus-keys">Prompt focus keys</h3><a class="sl-anchor-link" href="#prompt-focus-keys"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Prompt focus keys”</span></a></div>
<p>These keys are active when writing instructions inside the prompt box.</p>





















































































<table><thead><tr><th align="left">Key</th><th align="left">TUI Command</th><th align="left">Action Behavior</th></tr></thead>\<tbody\><tr><td align="left"><strong><code dir="auto">Enter</code></strong></td><td align="left"><code dir="auto">cli.enter</code></td><td align="left">Submits your prompt or active menu selection to the agent.</td></tr><tr><td align="left"><strong><code dir="auto">Shift+Enter</code></strong> / <strong><code dir="auto">Ctrl+J</code></strong></td><td align="left"><code dir="auto">prompt.insert_newline</code></td><td align="left">Inserts a clean newline without submitting.</td></tr><tr><td align="left"><strong><code dir="auto">Shift+Tab</code></strong></td><td align="left"><code dir="auto">cli.cycle_mode</code></td><td align="left">Cycles between execution modes (<code dir="auto">default</code> > <code dir="auto">accept-edits</code> > <code dir="auto">auto</code> > <code dir="auto">plan</code>; <code dir="auto">auto</code> only where available).</td></tr><tr><td align="left"><strong><code dir="auto">Ctrl+V</code></strong></td><td align="left"><code dir="auto">edit.paste</code></td><td align="left">Pastes graphic media files or clipboard blocks into the prompt.</td></tr><tr><td align="left"><strong><code dir="auto">Ctrl+O</code></strong></td><td align="left"><code dir="auto">view.toggle_trajectory</code></td><td align="left">Expands or collapses detailed tool reasoning outputs.</td></tr><tr><td align="left"><strong><code dir="auto">Ctrl+R</code></strong></td><td align="left"><code dir="auto">view.review_artifact</code></td><td align="left">Opens the Artifact Review panel.</td></tr><tr><td align="left"><strong><code dir="auto">Ctrl+G</code></strong></td><td align="left"><code dir="auto">edit.open_editor</code></td><td align="left">Launches your default <code dir="auto">$EDITOR</code> shell to compose your prompt.</td></tr><tr><td align="left"><strong><code dir="auto">Alt+J</code></strong></td><td align="left"><code dir="auto">subagent.jump_to_waiting</code></td><td align="left">Instantly switches focus to the next subagent awaiting confirmation.</td></tr><tr><td align="left"><strong><code dir="auto">Ctrl+K</code></strong></td><td align="left"><code dir="auto">subagent.approve_fast</code></td><td align="left">Instantly approves the pending subagent action listed in the status alert.</td></tr><tr><td align="left"><strong><code dir="auto">Ctrl+A</code></strong></td><td align="left"><code dir="auto">—</code></td><td align="left">Moves the prompt insertion cursor to the beginning of the line.</td></tr><tr><td align="left"><strong><code dir="auto">Ctrl+E</code></strong></td><td align="left"><code dir="auto">—</code></td><td align="left">Moves the prompt insertion cursor to the end of the line.</td></tr><tr><td align="left"><strong><code dir="auto">Ctrl+_</code></strong> / <strong><code dir="auto">Ctrl+Shift+-</code></strong></td><td align="left"><code dir="auto">edit.undo</code></td><td align="left">Reverts the last edit.</td></tr><tr><td align="left"><strong><code dir="auto">Ctrl+Shift+Z</code></strong></td><td align="left"><code dir="auto">edit.redo</code></td><td align="left">Redoes the last undone text operation.</td></tr><tr><td align="left"><strong><code dir="auto">Ctrl+D</code></strong></td><td align="left"><code dir="auto">—</code></td><td align="left">Forward delete (only when the prompt box is non-empty).</td></tr><tr><td align="left"><strong><code dir="auto">F5</code></strong></td><td align="left"><code dir="auto">voice.start_dictation</code></td><td align="left">Starts or stops <a href="/docs/cli/commands/voice">voice dictation</a>.</td></tr></tbody></table>
<div class="sl-heading-wrapper level-h3"><h3 id="navigation-and-scrolling">Navigation and scrolling</h3><a class="sl-anchor-link" href="#navigation-and-scrolling"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Navigation and scrolling”</span></a></div>
<p>These keys are active inside select panels, menus, and scrollable text boxes.</p>



































<table><thead><tr><th align="left">Key</th><th align="left">TUI Command</th><th align="left">Action Behavior</th></tr></thead>\<tbody\><tr><td align="left"><strong><code dir="auto">↑</code></strong> / <strong><code dir="auto">↓</code></strong></td><td align="left"><code dir="auto">navigation.up</code> / <code dir="auto">navigation.down</code></td><td align="left">Scrolls highlighted selections up or down by one item.</td></tr><tr><td align="left"><strong><code dir="auto">PgUp</code></strong> / <strong><code dir="auto">Shift+↑</code></strong></td><td align="left"><code dir="auto">navigation.page_up</code> / <code dir="auto">navigation.half_page_up</code></td><td align="left">Scrolls the active text viewport up by one page block.</td></tr><tr><td align="left"><strong><code dir="auto">PgDn</code></strong> / <strong><code dir="auto">Shift+↓</code></strong></td><td align="left"><code dir="auto">navigation.page_down</code> / <code dir="auto">navigation.half_page_down</code></td><td align="left">Scrolls the active text viewport down by one page block.</td></tr><tr><td align="left"><strong><code dir="auto">←</code></strong> / <strong><code dir="auto">→</code></strong></td><td align="left"><code dir="auto">navigation.left</code> / <code dir="auto">navigation.right</code></td><td align="left">Swaps pages inside multipage structures (like the Session Picker).</td></tr><tr><td align="left"><strong><code dir="auto">Tab</code></strong></td><td align="left"><code dir="auto">navigation.tab</code></td><td align="left">Confirms the highlighted slash-command autofill option.</td></tr></tbody></table>
<div class="sl-heading-wrapper level-h3"><h3 id="tool-confirmations">Tool confirmations</h3><a class="sl-anchor-link" href="#tool-confirmations"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Tool confirmations”</span></a></div>
<p>These keys are active during confirmation prompts.</p>

























<table><thead><tr><th align="left">Key</th><th align="left">TUI Command</th><th align="left">Action Behavior</th></tr></thead>\<tbody\><tr><td align="left"><strong><code dir="auto">y</code></strong></td><td align="left"><code dir="auto">confirm.yes</code></td><td align="left">Authorizes the proposed tool, command, or active artifact.</td></tr><tr><td align="left"><strong><code dir="auto">n</code></strong></td><td align="left"><code dir="auto">confirm.no</code></td><td align="left">Rejects the proposed tool, command, or active artifact.</td></tr><tr><td align="left"><strong><code dir="auto">A</code></strong></td><td align="left"><code dir="auto">—</code></td><td align="left">(Inside Review panel) Approves all generated artifacts in one action (built-in shortcut).</td></tr></tbody></table>
<div class="sl-heading-wrapper level-h2"><h2 id="command-line-flags">Command-line flags</h2><a class="sl-anchor-link" href="#command-line-flags"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Command-line flags”</span></a></div>
<p>Pass startup flags when launching <code dir="auto">agy</code> to configure the session or run non-interactively:</p>














































































































<table><thead><tr><th align="left">Flag</th><th align="left">Default</th><th align="left">Description</th></tr></thead>\<tbody\><tr><td align="left"><strong><code dir="auto">--mode</code></strong></td><td align="left"><code dir="auto">""</code></td><td align="left">Set the agent <a href="/docs/cli/modes">execution mode</a> for this session: <code dir="auto">accept-edits</code>, <code dir="auto">auto</code> (Preview; <code dir="auto">v1.3.5+</code> on macOS and Linux, <code dir="auto">v1.4.4+</code> on Windows; requires the terminal sandbox; not available for business or enterprise accounts), or <code dir="auto">plan</code>.</td></tr><tr><td align="left"><strong><code dir="auto">--model</code></strong></td><td align="left">—</td><td align="left">Select the model for the current CLI session (refer to <code dir="auto">agy models</code>).</td></tr><tr><td align="left"><strong><code dir="auto">--effort</code></strong></td><td align="left">—</td><td align="left">Set the <a href="/docs/cli/commands/effort">reasoning effort</a> for the current CLI session (<code dir="auto">low</code> through <code dir="auto">max</code>, depending on the model).</td></tr><tr><td align="left"><strong><code dir="auto">--agent</code></strong></td><td align="left">—</td><td align="left">Select the agent for the current CLI session (refer to <code dir="auto">agy agents</code>).</td></tr><tr><td align="left"><strong><code dir="auto">--add-dir</code></strong></td><td align="left"><code dir="auto">[]</code></td><td align="left">Add a directory to the workspace (repeatable).</td></tr><tr><td align="left"><strong><code dir="auto">-c</code></strong>, <strong><code dir="auto">--continue</code></strong></td><td align="left"><code dir="auto">false</code></td><td align="left">Continue the most recent conversation.</td></tr><tr><td align="left"><strong><code dir="auto">--conversation</code></strong></td><td align="left">—</td><td align="left">Resume a previous conversation by ID.</td></tr><tr><td align="left"><strong><code dir="auto">-i</code></strong>, <strong><code dir="auto">--prompt-interactive</code></strong></td><td align="left">—</td><td align="left">Run an initial prompt interactively and continue the session.</td></tr><tr><td align="left"><strong><code dir="auto">-p</code></strong>, <strong><code dir="auto">--print</code></strong>, <strong><code dir="auto">--prompt</code></strong></td><td align="left">—</td><td align="left">Run a single prompt non-interactively in <a href="/docs/cli/headless">headless mode</a> and print the response.</td></tr><tr><td align="left"><strong><code dir="auto">--output-format</code></strong></td><td align="left"><code dir="auto">text</code></td><td align="left">Output format for print mode (<code dir="auto">text</code>, <code dir="auto">json</code>, <code dir="auto">stream-json</code>).</td></tr><tr><td align="left"><strong><code dir="auto">--input-format</code></strong></td><td align="left"><code dir="auto">text</code></td><td align="left">Input format for print mode (<code dir="auto">text</code>, <code dir="auto">stream-json</code>).</td></tr><tr><td align="left"><strong><code dir="auto">--json-schema</code></strong></td><td align="left">—</td><td align="left">JSON schema string or path to a schema file to enforce structured output in print mode.</td></tr><tr><td align="left"><strong><code dir="auto">--print-timeout</code></strong></td><td align="left"><code dir="auto">0s</code></td><td align="left">Optional time limit for print mode (<code dir="auto">0</code> waits until the turn completes).</td></tr><tr><td align="left"><strong><code dir="auto">--disable-slash-commands</code></strong></td><td align="left"><code dir="auto">false</code></td><td align="left">Disable slash command and skill expansion in print mode.</td></tr><tr><td align="left"><strong><code dir="auto">--sandbox</code></strong></td><td align="left"><code dir="auto">false</code></td><td align="left">Run in a terminal sandbox with OS containment restrictions enabled (in <code dir="auto">v1.4.4+</code>, no effect if the sandbox isn’t available on your machine).</td></tr><tr><td align="left"><strong><code dir="auto">--dangerously-skip-permissions</code></strong></td><td align="left"><code dir="auto">false</code></td><td align="left">Auto-approve all tool permission requests without prompting.</td></tr><tr><td align="left"><strong><code dir="auto">--remote-control</code></strong></td><td align="left"><code dir="auto">false</code></td><td align="left">Create a <a href="/docs/remote-control?tab=cli">Remote Control</a> connection for the CLI session on startup.</td></tr><tr><td align="left"><strong><code dir="auto">--project</code></strong></td><td align="left">—</td><td align="left">Project ID or project name for the current CLI session.</td></tr><tr><td align="left"><strong><code dir="auto">--new-project</code></strong></td><td align="left"><code dir="auto">false</code></td><td align="left">Create a new project for this session.</td></tr><tr><td align="left"><strong><code dir="auto">--log-file</code></strong></td><td align="left">—</td><td align="left">Override the CLI log file path.</td></tr></tbody></table>
<div class="sl-heading-wrapper level-h2"><h2 id="configuration-keys-settingsjson">Configuration keys (<code dir="auto">settings.json</code>)</h2><a class="sl-anchor-link" href="#configuration-keys-settingsjson"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Configuration keys (settings.json)”</span></a></div>
<p>This section lists every key you can set in <code dir="auto">~/.gemini/antigravity-cli/settings.json</code>, with its type and default. A key you leave out uses its default. For how the CLI reads and saves the file, and for the keys that take an object, refer to <a href="/docs/settings?tab=cli">CLI settings</a>.</p>
<div class="sl-heading-wrapper level-h3"><h3 id="example-settingsjson">Example <code dir="auto">settings.json</code></h3><a class="sl-anchor-link" href="#example-settingsjson"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Example settings.json”</span></a></div>
<pre class="astro-code astro-code-themes github-light github-dark" style="background-color:#fff;--shiki-dark-bg:#24292e;color:#24292e;--shiki-dark:#e1e4e8; overflow-x: auto; white-space: pre-wrap; word-wrap: break-word;" tabindex="0" data-language="json" dir="ltr"><code><span class="line"><span style="color:#24292E;--shiki-dark:#E1E4E8">{</span></span>
<span class="line"><span style="color:#005CC5;--shiki-dark:#79B8FF">    "agentMode"</span><span style="color:#24292E;--shiki-dark:#E1E4E8">: </span><span style="color:#032F62;--shiki-dark:#9ECBFF">"auto"</span><span style="color:#24292E;--shiki-dark:#E1E4E8">,</span></span>
<span class="line"><span style="color:#005CC5;--shiki-dark:#79B8FF">    "colorScheme"</span><span style="color:#24292E;--shiki-dark:#E1E4E8">: </span><span style="color:#032F62;--shiki-dark:#9ECBFF">"tokyo night"</span><span style="color:#24292E;--shiki-dark:#E1E4E8">,</span></span>
<span class="line"><span style="color:#005CC5;--shiki-dark:#79B8FF">    "altScreenMode"</span><span style="color:#24292E;--shiki-dark:#E1E4E8">: </span><span style="color:#032F62;--shiki-dark:#9ECBFF">"always"</span><span style="color:#24292E;--shiki-dark:#E1E4E8">,</span></span>
<span class="line"><span style="color:#005CC5;--shiki-dark:#79B8FF">    "toolPermission"</span><span style="color:#24292E;--shiki-dark:#E1E4E8">: </span><span style="color:#032F62;--shiki-dark:#9ECBFF">"proceed-in-sandbox"</span><span style="color:#24292E;--shiki-dark:#E1E4E8">,</span></span>
<span class="line"><span style="color:#005CC5;--shiki-dark:#79B8FF">    "notifications"</span><span style="color:#24292E;--shiki-dark:#E1E4E8">: </span><span style="color:#005CC5;--shiki-dark:#79B8FF">true</span><span style="color:#24292E;--shiki-dark:#E1E4E8">,</span></span>
<span class="line"><span style="color:#005CC5;--shiki-dark:#79B8FF">    "enableTerminalSandbox"</span><span style="color:#24292E;--shiki-dark:#E1E4E8">: </span><span style="color:#005CC5;--shiki-dark:#79B8FF">true</span></span>
<span class="line"><span style="color:#24292E;--shiki-dark:#E1E4E8">}</span></span></code></pre>
<p>To get autocomplete and validation in editors such as VS Code, add <code dir="auto">"$schema": "https://antigravity.google/schemas/v1/settings.json"</code> at the top of the file. For details, refer to <a href="/docs/settings?tab=cli#editor-autocomplete-and-validation">Editor autocomplete and validation</a>.</p>

















































































































































































































<table><thead><tr><th align="left">Option Key Name</th><th align="left">Value Type</th><th align="left">System Default</th><th align="left">Parameter Characteristics &#x26; Options</th></tr></thead>\<tbody\><tr><td align="left"><strong><code dir="auto">agentMode</code></strong></td><td align="left">string</td><td align="left"><code dir="auto">""</code> (default mode)</td><td align="left">Default startup <a href="/docs/cli/modes">execution mode</a>: <code dir="auto">""</code> (standard <code dir="auto">default</code> mode), <code dir="auto">"accept-edits"</code> (auto-approve workspace file edits), <code dir="auto">"auto"</code> (Preview; <code dir="auto">v1.3.5+</code> on macOS and Linux, <code dir="auto">v1.4.4+</code> on Windows; requires the terminal sandbox; auto-approves edits, runs commands in the terminal sandbox by default, and automatically checks any command that needs to run outside the sandbox so safe, reversible commands run without prompting while irreversible actions still prompt; not available for business or enterprise accounts), or <code dir="auto">"plan"</code> (research and outline steps before writing code). Can be overridden per session with <code dir="auto">--mode</code>.</td></tr><tr><td align="left"><strong><code dir="auto">colorScheme</code></strong></td><td align="left">string</td><td align="left"><code dir="auto">"terminal"</code></td><td align="left">Color theme: <code dir="auto">"light"</code>, <code dir="auto">"solarized light"</code>, <code dir="auto">"colorblind-friendly light"</code>, <code dir="auto">"dark"</code>, <code dir="auto">"solarized dark"</code>, <code dir="auto">"colorblind-friendly dark"</code>, <code dir="auto">"tokyo night"</code>, or <code dir="auto">"terminal"</code> (inherits native shell colors).</td></tr><tr><td align="left"><strong><code dir="auto">altScreenMode</code></strong></td><td align="left">string</td><td align="left"><code dir="auto">"default"</code></td><td align="left">Screen buffer usage: <code dir="auto">"default"</code> (renders the conversation inline and opens full-screen views in the alternate screen), <code dir="auto">"always"</code> (runs the whole TUI in the alternate screen), or <code dir="auto">"never"</code> (never uses the alternate screen; available only in <code dir="auto">settings.json</code>).</td></tr><tr><td align="left"><strong><code dir="auto">toolPermission</code></strong></td><td align="left">string</td><td align="left"><code dir="auto">"request-review"</code></td><td align="left">Global safety presets: <code dir="auto">"request-review"</code> (prompts for write, bash, web, and MCP tools unless an allow rule covers them), <code dir="auto">"proceed-in-sandbox"</code> (runs terminal commands without prompting when the sandbox is on and the command runs inside it; in <code dir="auto">v1.4.4+</code>, behaves like <code dir="auto">"request-review"</code> if the sandbox isn’t available), <code dir="auto">"always-proceed"</code> (runs commands, web requests, and MCP tools without prompting unless an ask or deny rule matches), or <code dir="auto">"strict"</code> (prompts before every tool action, including file reads, even if an allow rule matches). Deny rules apply in every mode.</td></tr><tr><td align="left"><strong><code dir="auto">artifactReviewPolicy</code></strong></td><td align="left">string</td><td align="left"><code dir="auto">"asks-for-review"</code></td><td align="left">Whether the agent waits for your review when it asks for feedback on an artifact, such as an implementation plan: <code dir="auto">"asks-for-review"</code> (always waits), <code dir="auto">"agent-decides"</code> (currently the same as <code dir="auto">"asks-for-review"</code>), or <code dir="auto">"always-proceed"</code> (never waits). Doesn’t affect file edits.</td></tr><tr><td align="left"><strong><code dir="auto">notifications</code></strong></td><td align="left">boolean</td><td align="left"><code dir="auto">false</code></td><td align="left">Rings the terminal bell and shows a desktop notification when the agent needs your approval or an answer, or finishes its turn.</td></tr><tr><td align="left"><strong><code dir="auto">showTips</code></strong></td><td align="left">boolean</td><td align="left"><code dir="auto">true</code></td><td align="left">Displays helpful usage tips while the agent is generating responses.</td></tr><tr><td align="left"><strong><code dir="auto">showFeedbackSurvey</code></strong></td><td align="left">boolean</td><td align="left"><code dir="auto">true</code></td><td align="left">Displays periodic quality feedback surveys upon active task completions.</td></tr><tr><td align="left"><strong><code dir="auto">editor</code></strong></td><td align="left">string</td><td align="left"><code dir="auto">"auto"</code></td><td align="left">External editor for opening artifacts and files and for composing prompts (<code dir="auto">Ctrl+G</code>): <code dir="auto">"auto"</code> (uses <code dir="auto">$EDITOR</code>, then falls back to <code dir="auto">vim</code>, or to <code dir="auto">edit</code> and <code dir="auto">notepad</code> on Windows), <code dir="auto">"vim"</code>, <code dir="auto">"emacs"</code>, or any other editor command.</td></tr><tr><td align="left"><strong><code dir="auto">editorMode</code></strong></td><td align="left">string</td><td align="left"><code dir="auto">"default"</code></td><td align="left">Prompt editing model: <code dir="auto">"default"</code> (flat text editing) or <code dir="auto">"vim"</code> (modal editing). Distinct from <code dir="auto">editor</code>, which selects an external program. Refer to <a href="/docs/cli/vim-editor-mode">Vim editor mode</a>.</td></tr><tr><td align="left"><strong><code dir="auto">vimInsertFirst</code></strong></td><td align="left">boolean</td><td align="left"><code dir="auto">false</code></td><td align="left">Starts Vim editing in Insert mode and makes a bare <code dir="auto">Enter</code> submit. Requires <code dir="auto">editorMode</code> set to <code dir="auto">"vim"</code>.</td></tr><tr><td align="left"><strong><code dir="auto">allowNonWorkspaceAccess</code></strong></td><td align="left">boolean</td><td align="left"><code dir="auto">false</code></td><td align="left">Lets the agent read and edit files outside your workspace folders, subject to <code dir="auto">toolPermission</code>, <code dir="auto">agentMode</code>, and your permission rules.</td></tr><tr><td align="left"><strong><code dir="auto">enableTerminalSandbox</code></strong></td><td align="left">boolean</td><td align="left"><code dir="auto">false</code></td><td align="left">Restricts all local execution commands launched by agents to OS containment rings (in <code dir="auto">v1.4.4+</code>, no effect if the sandbox isn’t available on your machine).</td></tr><tr><td align="left"><strong><code dir="auto">useG1Credits</code></strong></td><td align="left">boolean</td><td align="left"><code dir="auto">false</code></td><td align="left">Uses your AI credits for model requests after your plan’s quota runs out. Not available for business accounts.</td></tr><tr><td align="left"><strong><code dir="auto">verbosity</code></strong></td><td align="left">string</td><td align="left"><code dir="auto">"medium"</code></td><td align="left">Visual verbosity level: <code dir="auto">"high"</code> (renders every tool call, command, and thought in full), <code dir="auto">"medium"</code> (groups related tool calls and thoughts into summaries while keeping commands and responses visible), or <code dir="auto">"low"</code> (renders only responses and collapses tool activity into a token usage summary).</td></tr><tr><td align="left"><strong><code dir="auto">runningLightSpeed</code></strong></td><td align="left">string</td><td align="left"><code dir="auto">"medium"</code></td><td align="left">Visual running light progress animation speed: <code dir="auto">"fast"</code>, <code dir="auto">"medium"</code>, <code dir="auto">"slow"</code>, or <code dir="auto">"off"</code>.</td></tr><tr><td align="left"><strong><code dir="auto">copyOnSelect</code></strong></td><td align="left">boolean</td><td align="left"><code dir="auto">true</code></td><td align="left">In alt-screen mode, copies text to the clipboard when you release a mouse selection. When <code dir="auto">false</code>, the artifact viewer uses your terminal’s own text selection.</td></tr><tr><td align="left"><strong><code dir="auto">hideActiveItems</code></strong></td><td align="left">boolean</td><td align="left"><code dir="auto">false</code></td><td align="left">Hides the list of active subagents and background tasks below the prompt box (<strong>Show Active Tasks</strong> in <code dir="auto">/config</code>).</td></tr><tr><td align="left"><strong><code dir="auto">pickerGrouping</code></strong></td><td align="left">string</td><td align="left"><code dir="auto">"none"</code></td><td align="left">How the <a href="/docs/cli/commands/resume"><code dir="auto">/resume</code></a> picker lists conversations: <code dir="auto">"none"</code> (one list, most recent first) or <code dir="auto">"grouped"</code> (grouped by workspace).</td></tr><tr><td align="left"><strong><code dir="auto">queuedMessages</code></strong></td><td align="left">string</td><td align="left"><code dir="auto">"queue"</code></td><td align="left">What happens to a message you send while the agent is working: <code dir="auto">"queue"</code> (held until the current turn ends) or <code dir="auto">"send-immediately"</code> (interrupts the agent and sends it right away).</td></tr><tr><td align="left"><strong><code dir="auto">clearScrollbackOnResize</code></strong></td><td align="left">boolean</td><td align="left"><code dir="auto">true</code></td><td align="left">In inline rendering, clears your terminal’s scrollback when the CLI redraws the conversation, such as after you resize the window.</td></tr><tr><td align="left"><strong><code dir="auto">disableSlashCommands</code></strong></td><td align="left">boolean</td><td align="left"><code dir="auto">false</code></td><td align="left">In <a href="/docs/cli/headless">headless mode</a> (<code dir="auto">-p</code>), sends prompts that start with <code dir="auto">/</code> as typed instead of running slash commands or skills. Same as <code dir="auto">--disable-slash-commands</code>.</td></tr><tr><td align="left"><strong><code dir="auto">historySize</code></strong></td><td align="left">integer</td><td align="left"><code dir="auto">2000</code></td><td align="left">Prompt history entries kept on disk, trimmed when the CLI exits: <code dir="auto">-1</code> keeps every entry, and <code dir="auto">0</code> clears the history.</td></tr><tr><td align="left"><strong><code dir="auto">model</code></strong></td><td align="left">string</td><td align="left"><code dir="auto">""</code></td><td align="left">The model you chose with <code dir="auto">/model</code>, saved by its label.</td></tr><tr><td align="left"><strong><code dir="auto">modelProvider</code></strong></td><td align="left">string</td><td align="left"><code dir="auto">""</code></td><td align="left"><code dir="auto">"gemini"</code> sends model requests directly to the Gemini API with the key in <code dir="auto">GEMINI_API_KEY</code> (<a href="/docs/install#using-a-gemini-api-key">Using a Gemini API key</a>).</td></tr><tr><td align="left"><strong><code dir="auto">trustedWorkspaces</code></strong></td><td align="left">string[]</td><td align="left"><code dir="auto">[]</code></td><td align="left">Folders you chose to trust when the CLI asked. The CLI adds entries for you.</td></tr><tr><td align="left"><strong><code dir="auto">useTerminalCursor</code></strong></td><td align="left">boolean</td><td align="left"><code dir="auto">false</code></td><td align="left">Uses your terminal’s own cursor shape instead of a block cursor.</td></tr><tr><td align="left"><strong><code dir="auto">permissions</code></strong></td><td align="left">object</td><td align="left">—</td><td align="left">Allow, ask, and deny rules for the agent’s tools. Refer to <a href="/docs/settings?tab=cli#permissions"><code dir="auto">permissions</code></a>.</td></tr><tr><td align="left"><strong><code dir="auto">statusLine</code></strong></td><td align="left">object</td><td align="left">—</td><td align="left">Custom status line script. Refer to <a href="/docs/settings?tab=cli#statusline-and-title"><code dir="auto">statusLine</code> and <code dir="auto">title</code></a>.</td></tr><tr><td align="left"><strong><code dir="auto">title</code></strong></td><td align="left">object</td><td align="left">—</td><td align="left">Custom terminal title script. Refer to <a href="/docs/settings?tab=cli#statusline-and-title"><code dir="auto">statusLine</code> and <code dir="auto">title</code></a>.</td></tr><tr><td align="left"><strong><code dir="auto">sandboxProxy</code></strong></td><td align="left">object</td><td align="left">—</td><td align="left">HTTP proxy for sandboxed terminal commands. Refer to <a href="/docs/settings?tab=cli#sandboxproxy"><code dir="auto">sandboxProxy</code></a>.</td></tr><tr><td align="left"><strong><code dir="auto">customModelsConfig</code></strong></td><td align="left">object</td><td align="left">—</td><td align="left">Extra models for the Gemini API key setup. Refer to <a href="/docs/settings?tab=cli#custommodelsconfig-and-gcp"><code dir="auto">customModelsConfig</code> and <code dir="auto">gcp</code></a>.</td></tr><tr><td align="left"><strong><code dir="auto">gcp</code></strong></td><td align="left">object</td><td align="left">—</td><td align="left">Saved by the <strong>Use a Google Cloud project</strong> sign-in. Refer to <a href="/docs/settings?tab=cli#custommodelsconfig-and-gcp"><code dir="auto">customModelsConfig</code> and <code dir="auto">gcp</code></a>.</td></tr></tbody></table>
<p><strong>Enable Telemetry</strong> in <code dir="auto">/config</code> has no <code dir="auto">settings.json</code> key: the CLI saves that choice with your account.</p>
<div class="sl-heading-wrapper level-h2"><h2 id="next-steps">Next steps</h2><a class="sl-anchor-link" href="#next-steps"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Next steps”</span></a></div>
<p>Learn how to safely deploy permission policies, sandboxes, and customize plugins:</p>
\<ul\>
\<li\><strong><a href="/docs/settings?tab=cli">CLI settings</a></strong>: Use the interactive <code dir="auto">/config</code> panel, choose a rendering mode, and see how the CLI reads and saves <code dir="auto">settings.json</code>.</li>
\<li\><strong><a href="/docs/sandbox?tab=cli">Permissions and sandbox</a></strong>: Enforce command-line containment rules.</li>
\<li\><strong><a href="/docs/plugins?tab=cli">Plugins and skills</a></strong>: Create your own custom skills slash commands.</li>
\<li\><strong><a href="/docs/install">Installation and auth</a></strong>: Update your CLI installation.</li>
</ul></div><footer class="sl-flex docs-footer docs-footer-wrapper astro-4mgpquaz" data-pagefind-ignore><div class="meta sl-flex astro-4mgpquaz"></div><div class="pagination-links print:hidden astro-jgq3vlii" dir="ltr"><a href="/docs/ide/extensions/xcode/" rel="prev" class="astro-jgq3vlii"><svg aria-hidden="true" class="astro-jgq3vlii astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.5rem;"><path d="M17 11H9.41l3.3-3.29a1.004 1.004 0 1 0-1.42-1.42l-5 5a1 1 0 0 0-.21.33 1 1 0 0 0 0 .76 1 1 0 0 0 .21.33l5 5a1.002 1.002 0 0 0 1.639-.325 1 1 0 0 0-.219-1.095L9.41 13H17a1 1 0 0 0 0-2Z"/></svg><span class="astro-jgq3vlii">Previous<br class="astro-jgq3vlii"><span class="link-title astro-jgq3vlii">Xcode</span></span></a><a href="/docs/cli/features/" rel="next" class="astro-jgq3vlii"><svg aria-hidden="true" class="astro-jgq3vlii astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.5rem;"><path d="M17.92 11.62a1.001 1.001 0 0 0-.21-.33l-5-5a1.003 1.003 0 1 0-1.42 1.42l3.3 3.29H7a1 1 0 0 0 0 2h7.59l-3.3 3.29a1.002 1.002 0 0 0 .325 1.639 1 1 0 0 0 1.095-.219l5-5a1 1 0 0 0 .21-.33 1 1 0 0 0 0-.76Z"/></svg><span class="astro-jgq3vlii">Next<br class="astro-jgq3vlii"><span class="link-title astro-jgq3vlii">CLI features</span></span></a></div><nav class="docs-legal-nav print:hidden astro-4mgpquaz" data-analytics-nav="docs_footer" aria-label="Google legal and cookie links"><a class="docs-footer-link astro-4mgpquaz" href="https://about.google/" target="_blank" rel="noopener noreferrer">About Google</a><a class="docs-footer-link astro-4mgpquaz" href="https://about.google/products/" target="_blank" rel="noopener noreferrer">Google Products</a><a class="docs-footer-link astro-4mgpquaz" href="https://policies.google.com/privacy?hl=en-US" target="_blank" rel="noopener noreferrer">Privacy</a><a class="docs-footer-link astro-4mgpquaz" href="/terms">Terms</a><a aria-hidden="true" class="docs-footer-link glue-cookie-notification-bar-control astro-4mgpquaz">Manage cookies</a></nav><template data-download-survey-template><dialog class="dls" data-download-survey aria-labelledby="dls-title" aria-describedby="dls-subtitle" data-nosnippet><div class="dls-card"><button type="button" class="dls-close" data-dls-dismiss="close" aria-label="Close"><svg viewBox="0 0 24 24" aria-hidden="true" focusable="false"><path d="M6.5 6.5l11 11M17.5 6.5l-11 11"></path></svg></button><div class="dls-body"><div class="dls-ask" data-dls-view="ask"><p class="dls-status" data-dls-status><svg class="dls-status-icon" viewBox="0 0 20 20" aria-hidden="true" focusable="false"><circle cx="10" cy="10" r="10"></circle><path d="M6 10.4l2.6 2.6L14 7.6"></path></svg><span data-download-survey-text>Your download has started</span></p><h2 class="dls-title" id="dls-title" data-download-survey-text>How did you hear about us?</h2><p class="dls-subtitle" id="dls-subtitle" data-download-survey-text>Pick one. It helps us reach more people like you.</p><div class="dls-options" role="group" aria-labelledby="dls-title" data-dls-options><button type="button" class="dls-option" data-dls-answer="search"><svg class="dls-option-check" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="M3.2 8.4l3 3 6.6-6.8"></path></svg><span data-download-survey-text>Search engine</span></button><button type="button" class="dls-option" data-dls-answer="ai_assistant"><svg class="dls-option-check" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="M3.2 8.4l3 3 6.6-6.8"></path></svg><span data-download-survey-text>AI assistant</span></button><button type="button" class="dls-option" data-dls-answer="youtube"><svg class="dls-option-check" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="M3.2 8.4l3 3 6.6-6.8"></path></svg><span data-download-survey-text>YouTube</span></button><button type="button" class="dls-option" data-dls-answer="x"><svg class="dls-option-check" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="M3.2 8.4l3 3 6.6-6.8"></path></svg><span data-download-survey-text>X (Twitter)</span></button><button type="button" class="dls-option" data-dls-answer="linkedin"><svg class="dls-option-check" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="M3.2 8.4l3 3 6.6-6.8"></path></svg><span data-download-survey-text>LinkedIn</span></button><button type="button" class="dls-option" data-dls-answer="reddit"><svg class="dls-option-check" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="M3.2 8.4l3 3 6.6-6.8"></path></svg><span data-download-survey-text>Reddit</span></button><button type="button" class="dls-option" data-dls-answer="hacker_news"><svg class="dls-option-check" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="M3.2 8.4l3 3 6.6-6.8"></path></svg><span data-download-survey-text>Hacker News</span></button><button type="button" class="dls-option" data-dls-answer="friend_or_colleague"><svg class="dls-option-check" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="M3.2 8.4l3 3 6.6-6.8"></path></svg><span data-download-survey-text>Friend or colleague</span></button><button type="button" class="dls-option" data-dls-answer="news_or_blog"><svg class="dls-option-check" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="M3.2 8.4l3 3 6.6-6.8"></path></svg><span data-download-survey-text>News or blog</span></button><button type="button" class="dls-option" data-dls-answer="event"><svg class="dls-option-check" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="M3.2 8.4l3 3 6.6-6.8"></path></svg><span data-download-survey-text>Event or conference</span></button><button type="button" class="dls-option" data-dls-answer="google_product"><svg class="dls-option-check" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="M3.2 8.4l3 3 6.6-6.8"></path></svg><span data-download-survey-text>Another Google product</span></button><button type="button" class="dls-option" data-dls-answer="other"><svg class="dls-option-check" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="M3.2 8.4l3 3 6.6-6.8"></path></svg><span data-download-survey-text>Other</span></button></div><div class="dls-actions"><button type="button" class="dls-skip" data-dls-dismiss="skip"><span data-download-survey-text>Skip</span></button></div></div><div class="dls-thanks" data-dls-view="thanks" role="status" aria-live="polite" aria-labelledby="dls-thanks-title" tabindex="-1" hidden><div class="dls-thanks-content" data-dls-thanks-content><svg class="dls-thanks-mark" viewBox="0 0 52 52" aria-hidden="true" focusable="false"><circle cx="26" cy="26" r="24"></circle><path d="M16 27l7 7 14-15"></path></svg><p class="dls-thanks-title" id="dls-thanks-title" data-download-survey-text>Thanks for letting us know!</p></div></div></div></div></dialog></template><script type="module" src="/_astro/DownloadSurvey.astro_astro_type_script_index_0_lang.DWzR8tN5.js"></script></footer></div></div></main></div></div></div></div></body></html>