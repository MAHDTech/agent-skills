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
</script><!-- Client runtime modules: i18n dictionary runtime and adaptive network/transitions manager --><script type="module" src="/_astro/SharedHeadBootstrap.astro_astro_type_script_index_0_lang.DdvTdmaD.js"></script><!-- Google Glue Cookie Notification Bar stylesheet (deferred print->all) --><link href="https://www.gstatic.com/glue/cookienotificationbar/cookienotificationbar.min.css" rel="stylesheet" media="print" onload="this.media='all'" />\<noscript\><link href="https://www.gstatic.com/glue/cookienotificationbar/cookienotificationbar.min.css" rel="stylesheet" /></noscript><meta charset="utf-8"/><meta name="viewport" content="width=device-width, initial-scale=1"/>\<title\>Remote Control | Google Antigravity Docs</title><link rel="canonical" href="https://antigravity.google/docs/remote-control/"/><link rel="sitemap" href="/sitemap-index.xml"/><link rel="icon" type="image/x-icon" href="/favicon.ico" sizes="48x48"/><link rel="icon" type="image/png" href="/apple-touch-icon.png" sizes="180x180"/><link rel="icon" type="image/png" href="/assets/image/antigravity-logo.png" sizes="276x276"/><link rel="apple-touch-icon" href="/apple-touch-icon.png" sizes="180x180"/><link rel="apple-touch-icon-precomposed" href="/apple-touch-icon-precomposed.png" sizes="180x180"/><link rel="preconnect" href="https://fonts.googleapis.com"/><link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/><link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Google+Symbols:opsz,wght,FILL,GRAD,ROND@20..48,100..700,0..1,-50..200,0..100&display=block" media="print" onload="this.media='all'"/><link rel="shortcut icon" href="/favicon.svg" type="image/svg+xml"/><meta name="generator" content="Astro v7.2.9"/><meta name="generator" content="Starlight v0.41.5"/><meta property="og:title" content="Remote Control | Google Antigravity Docs"/><meta property="og:type" content="article"/><meta property="og:url" content="https://antigravity.google/docs/remote-control/"/><meta property="og:locale" content="en"/><meta property="og:site_name" content="Google Antigravity Docs"/><meta property="og:image" content="https://antigravity.google/assets/image/sitecards/sitecard-documentation.png"/><meta property="og:image:secure_url" content="https://antigravity.google/assets/image/sitecards/sitecard-documentation.png"/><meta name="twitter:image" content="https://antigravity.google/assets/image/sitecards/sitecard-documentation.png"/><meta name="twitter:card" content="summary_large_image"/><meta name="description" content="Antigravity Remote Control allows you to securely connect to and drive your Antigravity desktop sessions and CLI instances running across your machines from…"/><meta property="og:description" content="Antigravity Remote Control allows you to securely connect to and drive your Antigravity desktop sessions and CLI instances running across your machines from…"/><meta name="twitter:title" content="Remote Control | Google Antigravity Docs"/><meta name="twitter:description" content="Antigravity Remote Control allows you to securely connect to and drive your Antigravity desktop sessions and CLI instances running across your machines from…"/><script type="application/ld+json">{"@context":"https://schema.org","@graph":[{"@type":"TechArticle","headline":"Remote Control","description":"Antigravity Remote Control allows you to securely connect to and drive your Antigravity desktop sessions and CLI instances running across your machines from…","url":"https://antigravity.google/docs/remote-control/","mainEntityOfPage":{"@type":"WebPage","@id":"https://antigravity.google/docs/remote-control/"},"author":{"@type":"Organization","name":"Google Antigravity","url":"https://antigravity.google/"},"publisher":{"@type":"Organization","name":"Google","url":"https://www.google.com/"}},{"@type":"BreadcrumbList","itemListElement":[{"@type":"ListItem","position":1,"name":"Docs","item":"https://antigravity.google/docs/overview/"},{"@type":"ListItem","position":2,"name":"Remote Control","item":"https://antigravity.google/docs/remote-control/"}]}]}</script>\<script\>
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
</style><script type="module" src="/_astro/page.LAbJoB63.js"></script>\<style\>@layer starlight.components{.card:where(.astro-zts4jgjd){--sl-card-border:var(--sl-color-purple);--sl-card-bg:var(--sl-color-purple-low);border:1px solid var(--sl-color-gray-5);background-color:var(--sl-color-black);flex-direction:column;gap:clamp(.5rem,.125rem + 1vw,1rem);padding:clamp(1rem,.125rem + 3vw,2.5rem)}.card:where(.astro-zts4jgjd):nth-child(4n+1){--sl-card-border:var(--sl-color-orange);--sl-card-bg:var(--sl-color-orange-low)}.card:where(.astro-zts4jgjd):nth-child(4n+3){--sl-card-border:var(--sl-color-green);--sl-card-bg:var(--sl-color-green-low)}.card:where(.astro-zts4jgjd):nth-child(4n+4){--sl-card-border:var(--sl-color-red);--sl-card-bg:var(--sl-color-red-low)}.card:where(.astro-zts4jgjd):nth-child(4n+5){--sl-card-border:var(--sl-color-blue);--sl-card-bg:var(--sl-color-blue-low)}.title:where(.astro-zts4jgjd){font-weight:600;font-size:var(--sl-text-h4);color:var(--sl-color-white);line-height:var(--sl-line-height-headings);align-items:center;gap:1rem}.card:where(.astro-zts4jgjd) .icon:where(.astro-zts4jgjd){border:1px solid var(--sl-card-border);background-color:var(--sl-card-bg);border-radius:.25rem;flex-shrink:0;padding:.2em}.card:where(.astro-zts4jgjd) .body:where(.astro-zts4jgjd){font-size:clamp(var(--sl-text-sm), calc(.5rem + 1vw), var(--sl-text-body));margin:0}}
</style>\<style\>@layer starlight.components{.card-grid:where(.astro-lmor3ela){grid-template-columns:100%;gap:1rem;display:grid}.card-grid:where(.astro-lmor3ela)>*{margin-top:0!important}@media (width>=50rem){.card-grid:where(.astro-lmor3ela){grid-template-columns:1fr 1fr;gap:1.5rem}.stagger:where(.astro-lmor3ela){--stagger-height:5rem;padding-bottom:var(--stagger-height)}.stagger:where(.astro-lmor3ela)>:nth-child(2n){transform:translateY(var(--stagger-height))}}}
</style>\<style\>@layer starlight.components{.sl-link-card:where(.astro-vodiqeol){border:1px solid var(--sl-color-gray-5);box-shadow:var(--sl-shadow-sm);border-radius:.5rem;grid-template-columns:1fr auto;gap:.5rem;padding:1rem;display:grid;position:relative}a:where(.astro-vodiqeol){line-height:var(--sl-line-height-headings);text-decoration:none}a:where(.astro-vodiqeol):before{content:"";position:absolute;inset:0}.stack:where(.astro-vodiqeol){flex-direction:column;gap:.5rem}.title:where(.astro-vodiqeol){color:var(--sl-color-white);font-weight:600;font-size:var(--sl-text-lg)}.description:where(.astro-vodiqeol){color:var(--sl-color-gray-3);line-height:1.5}.icon:where(.astro-vodiqeol){color:var(--sl-color-gray-3)}.sl-link-card:where(.astro-vodiqeol):hover{background:var(--sl-color-gray-7,var(--sl-color-gray-6));border-color:var(--sl-color-gray-2)}.sl-link-card:where(.astro-vodiqeol):hover .icon:where(.astro-vodiqeol){color:var(--sl-color-white)}}
</style>\<style\>@layer starlight.components{.sl-steps{--bullet-size:calc(var(--sl-line-height) * 1rem);--bullet-margin:.375rem;counter-reset:steps-counter var(--sl-steps-start,0);padding-inline-start:0;list-style:none}.sl-steps>li{counter-increment:steps-counter;padding-inline-start:calc(var(--bullet-size) + 1rem);min-height:calc(var(--bullet-size) + var(--bullet-margin));padding-bottom:1px;position:relative}.sl-steps>li+li{margin-top:0}.sl-steps>li:before{content:counter(steps-counter);top:0;width:var(--bullet-size);height:var(--bullet-size);line-height:var(--bullet-size);font-size:var(--sl-text-xs);text-align:center;color:var(--sl-color-white);background-color:var(--sl-color-gray-6);box-shadow:inset 0 0 0 1px var(--sl-color-gray-5);border-radius:99rem;font-weight:600;position:absolute;inset-inline-start:0}.sl-steps>li:after{--guide-width:1px;content:"";top:calc(var(--bullet-size) + var(--bullet-margin));bottom:var(--bullet-margin);width:var(--guide-width);background-color:var(--sl-color-hairline-light);position:absolute;inset-inline-start:calc((var(--bullet-size) - var(--guide-width)) / 2)}}@layer starlight.content{.sl-steps>li>:first-child{--lh:calc(1em * var(--sl-line-height));--shift-y:calc(.5 * (var(--bullet-size) - var(--lh)));transform:translateY(var(--shift-y));margin-bottom:var(--shift-y)}.sl-steps>li>:first-child:where(h1,h2,h3,h4,h5,h6){--lh:calc(1em * var(--sl-line-height-headings))}@supports (--prop:1lh){.sl-steps>li>:first-child{--lh:1lh}}}
</style>\<style\>@layer starlight.components{starlight-tabs:where(.astro-hbt4ddzs){display:block}.tablist-wrapper:where(.astro-hbt4ddzs){overflow-x:auto}:where(.astro-hbt4ddzs)[role=tablist]{border-bottom:2px solid var(--sl-color-gray-5);padding:0;list-style:none;display:flex}.tab:where(.astro-hbt4ddzs){display:flex}.tab:where(.astro-hbt4ddzs)>:where(.astro-hbt4ddzs)[role=tab]{--sl-tab-color-border:var(--sl-color-gray-5);line-height:var(--sl-line-height-headings);box-shadow:0 2px 0 var(--sl-tab-color-border);color:var(--sl-color-gray-3);outline-offset:var(--sl-outline-offset-inside);overflow-wrap:initial;align-items:center;gap:.5rem;padding:.275rem 1.25rem;text-decoration:none;display:flex}.tab:where(.astro-hbt4ddzs) :where(.astro-hbt4ddzs)[role=tab][aria-selected=true]{--sl-tab-color-border:var(--sl-color-text-accent);color:var(--sl-color-white);font-weight:600}.tablist-wrapper:where(.astro-hbt4ddzs)~[role=tabpanel]{margin-top:1rem}}
</style><link rel="stylesheet" href="/_astro/Mp4Viewer.DPgP9jHx.css"></head><body class="astro-vdxcvpsm"><a class="sl-skip-link astro-vxeigawq" href="#_top">Skip to content</a><div class="page sl-flex astro-gqpr5s5a"><header class="header astro-gqpr5s5a"><div class="header-stack astro-mtusqnhg" data-analytics-nav="docs_header"><!-- Row 1: Logo and Search / Controls --><div class="header-main astro-mtusqnhg"><div class="title-wrapper sl-flex astro-mtusqnhg"><a href="/" class="site-title sl-flex astro-clw6ags5"><img class="light:sl-hidden print:hidden astro-clw6ags5" alt src="/_astro/antigravity_product_lockup_dark.YcYY_PSr.svg" width="1687" height="217"><img class="dark:sl-hidden print:block astro-clw6ags5" alt src="/_astro/antigravity_product_lockup_full_color.cUD_Se6a.svg" width="1687" height="217"><span class="sr-only astro-clw6ags5" translate="no">Google Antigravity Docs</span></a></div><div class="sl-flex search-wrapper print:hidden astro-mtusqnhg"><site-search id="doc-search-root" class="agy-hybrid-search" data-i18n-ignore="true" data-index-version="60cca000"><button type="button" data-open-modal id="doc-search-trigger" class="agy-search-trigger" aria-label="Search documentation" aria-keyshortcuts="Control+K Meta+K /"><span class="agy-search-trigger__left"><svg class="agy-search-trigger__icon" aria-hidden="true" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg><span class="agy-search-trigger__text">Search<span class="agy-search-trigger__text-suffix"> docs...</span></span></span><kbd class="agy-search-trigger__kbd" aria-hidden="true">⌘K</kbd></button><dialog id="doc-search-modal" class="agy-search-dialog" aria-label="Search"><div class="dialog-frame sl-flex"><div id="starlight__search" class="agy-search-container"><div class="agy-search-header"><div class="agy-search-input-wrap"><svg class="agy-search-input-icon" aria-hidden="true" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg><input id="doc-search-input" class="pagefind-ui__search-input" type="search" placeholder="Search documentation..." autocomplete="off" autocorrect="off" autocapitalize="off" spellcheck="false" aria-label="Search documentation" aria-controls="doc-search-results-list"><button type="button" class="agy-search-clear" data-clear-search hidden aria-label="Clear search query" title="Clear search"><svg aria-hidden="true" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><line x1="15" y1="9" x2="9" y2="15"></line><line x1="9" y1="9" x2="15" y2="15"></line></svg></button></div><div class="agy-search-header__actions"><button type="button" class="agy-shortcuts-toggle agy-search-help-btn" data-toggle-shortcuts aria-expanded="false" aria-controls="doc-search-shortcuts-panel" aria-label="Keyboard shortcuts" title="Keyboard shortcuts"><span aria-hidden="true">?</span></button><button type="button" data-close-modal class="agy-search-close" aria-label="Close search"><span class="agy-search-close__kbd" aria-hidden="true">ESC</span><span class="agy-search-close__touch" aria-hidden="true">Cancel</span></button></div></div><div class="agy-search-filters" role="toolbar" aria-label="Filter results by product surface"><button type="button" class="agy-surface-pill active" data-surface-filter="all" data-surface-label="All" aria-pressed="true"><span class="agy-surface-pill__label">All</span></button><button type="button" class="agy-surface-pill" data-surface-filter="2-0" data-surface-label="Antigravity App" aria-pressed="false"><span class="agy-surface-pill__label">App</span><span class="agy-surface-pill__mute" data-exclude-surface="2-0" aria-hidden="true" title="Exclude Antigravity App from results">×</span></button><button type="button" class="agy-surface-pill" data-surface-filter="cli" data-surface-label="Antigravity CLI" aria-pressed="false"><span class="agy-surface-pill__label">CLI</span><span class="agy-surface-pill__mute" data-exclude-surface="cli" aria-hidden="true" title="Exclude Antigravity CLI from results">×</span></button><button type="button" class="agy-surface-pill" data-surface-filter="antigravity-ide" data-surface-label="Antigravity IDE" aria-pressed="false"><span class="agy-surface-pill__label">IDE</span><span class="agy-surface-pill__mute" data-exclude-surface="antigravity-ide" aria-hidden="true" title="Exclude Antigravity IDE from results">×</span></button><button type="button" class="agy-surface-pill" data-surface-filter="ide" data-surface-label="IDE Extensions" aria-pressed="false"><span class="agy-surface-pill__label">Extensions</span><span class="agy-surface-pill__mute" data-exclude-surface="ide" aria-hidden="true" title="Exclude IDE Extensions from results">×</span></button><button type="button" class="agy-surface-pill" data-surface-filter="sdk" data-surface-label="Antigravity SDK" aria-pressed="false"><span class="agy-surface-pill__label">SDK</span><span class="agy-surface-pill__mute" data-exclude-surface="sdk" aria-hidden="true" title="Exclude Antigravity SDK from results">×</span></button><button type="button" class="agy-surface-pill" data-surface-filter="blog" data-surface-label="Blog" aria-pressed="false"><span class="agy-surface-pill__label">Blog</span><span class="agy-surface-pill__mute" data-exclude-surface="blog" aria-hidden="true" title="Exclude Blog from results">×</span></button><button type="button" class="agy-surface-pill" data-surface-filter="releases" data-surface-label="Releases" aria-pressed="false"><span class="agy-surface-pill__label">Releases</span><span class="agy-surface-pill__mute" data-exclude-surface="releases" aria-hidden="true" title="Exclude Releases from results">×</span></button></div><div id="doc-search-status" class="agy-search-status" data-search-status data-i18n-ignore="true" role="status" aria-live="polite" aria-atomic="true" hidden></div><div id="doc-search-results" class="pagefind-ui__results-area agy-search-results-area" data-results-container data-i18n-ignore="true"><div class="agy-search-suggestions" data-search-suggestions><p class="agy-search-suggestions__heading" data-discovery-heading>Explore Antigravity</p><div class="agy-discover-grid" data-discovery-grid><a class="agy-discover-card agy-search-card" data-doc-card="getting-started" href="/docs/getting-started/"><span class="agy-discover-card__body"><span class="agy-discover-card__title agy-search-card-title">Getting Started</span><span class="agy-discover-card__desc agy-search-card-desc">Install & launch your first agent</span></span><span class="agy-discover-card__arrow" aria-hidden="true">→</span></a><a class="agy-discover-card agy-search-card" data-doc-card="remote-control" href="/docs/remote-control/"><span class="agy-discover-card__body"><span class="agy-discover-card__title agy-search-card-title">Remote Control</span><span class="agy-discover-card__desc agy-search-card-desc">Drive desktop & CLI sessions from any browser</span></span><span class="agy-discover-card__arrow" aria-hidden="true">→</span></a><a class="agy-discover-card agy-search-card" data-doc-card="skills" href="/docs/skills/"><span class="agy-discover-card__body"><span class="agy-discover-card__title agy-search-card-title">Agent Skills & Rules</span><span class="agy-discover-card__desc agy-search-card-desc">Reusable workflows & workspace instructions</span></span><span class="agy-discover-card__arrow" aria-hidden="true">→</span></a><a class="agy-discover-card agy-search-card" data-doc-card="subagents" href="/docs/subagents/"><span class="agy-discover-card__body"><span class="agy-discover-card__title agy-search-card-title">Subagents & Teamwork</span><span class="agy-discover-card__desc agy-search-card-desc">Parallel multi-agent orchestration</span></span><span class="agy-discover-card__arrow" aria-hidden="true">→</span></a><a class="agy-discover-card agy-search-card" data-doc-card="sdk" href="/docs/sdk/overview/"><span class="agy-discover-card__body"><span class="agy-discover-card__title agy-search-card-title">Antigravity SDK</span><span class="agy-discover-card__desc agy-search-card-desc">Build custom Python agents & hooks</span></span><span class="agy-discover-card__arrow" aria-hidden="true">→</span></a><a class="agy-discover-card agy-search-card" data-doc-card="permissions" href="/docs/permissions/"><span class="agy-discover-card__body"><span class="agy-discover-card__title agy-search-card-title">Permissions & Sandbox</span><span class="agy-discover-card__desc agy-search-card-desc">File access controls & safe execution</span></span><span class="agy-discover-card__arrow" aria-hidden="true">→</span></a></div><div class="agy-try-searching"><span class="agy-try-searching__label" data-try-searching-label>Try searching</span><div class="agy-try-searching__chips" data-suggestion-chips><button type="button" class="agy-suggestion-chip agy-search-chip" data-search-suggestion="Remote Control" data-query="Remote Control">Remote Control</button><button type="button" class="agy-suggestion-chip agy-search-chip" data-search-suggestion="agent skills" data-query="agent skills">Agent skills</button><button type="button" class="agy-suggestion-chip agy-search-chip" data-search-suggestion="custom subagents" data-query="custom subagents">Custom subagents</button><button type="button" class="agy-suggestion-chip agy-search-chip" data-search-suggestion="IDE extensions" data-query="IDE extensions">IDE extensions</button><button type="button" class="agy-suggestion-chip agy-search-chip" data-search-suggestion="sandbox permissions" data-query="sandbox permissions">Sandbox permissions</button></div></div></div><ol id="doc-search-results-list" class="pagefind-ui__results agy-search-results-list" data-results-list data-i18n-ignore="true"></ol></div><div class="agy-related-topics" data-related-topics data-i18n-ignore="true" hidden></div><div id="doc-search-shortcuts-panel" class="agy-search-footer agy-search-help-popover" data-shortcuts-panel hidden><div class="agy-search-footer__hints"><span class="agy-footer-hint">\<kbd\>↑</kbd>\<kbd\>↓</kbd><span data-footer-hint="navigate">Navigate</span></span><span class="agy-footer-hint">\<kbd\>↵</kbd> <span data-footer-hint="open">Open</span></span><span class="agy-footer-hint">\<kbd\>esc</kbd> <span data-footer-hint="close">Close</span></span></div></div></div></div></dialog></site-search><script type="module" src="/_astro/SearchOverride.astro_astro_type_script_index_0_lang.DyJlY0iY.js"></script></div><div class="sl-hidden md:sl-flex print:hidden right-group astro-mtusqnhg"><div class="header-cta-group astro-mtusqnhg"><a class="top-nav-btn remote-control-btn astro-mtusqnhg" href="https://antigravity.google.com" target="_blank" rel="noopener noreferrer" title="Launch Remote Control"><span class="btn-icon astro-mtusqnhg" aria-hidden="true">🚀</span> Remote Control</a><a href="/download" class="call-to-action--nav button button-primary button-compact download-button dropdown-nav" aria-haspopup="false" aria-expanded="false" data-analytics-event="download-cta" data-module-name="header" data-page-position="main nav" class="astro-mtusqnhg"><span>Download</span><span class="dropdown-icon" translate="no"><svg class viewBox="0 0 24 24" width="18" height="18" fill="currentColor" aria-hidden="true" xmlns="http://www.w3.org/2000/svg"><path d="M12 16L7 11L8.4 9.55L11 12.15V4H13V12.15L15.6 9.55L17 11L12 16ZM6 20C5.45 20 4.979 19.804 4.588 19.413C4.196 19.021 4.00067 18.55 4 18V15H6V18H18V15H20V18C20 18.55 19.804 19.021 19.413 19.413C19.021 19.804 18.55 20 18 20H6Z"></path></svg></span></a><script type="module">var e=/^\d+\.\d+\.\d+$/,t=/^\d+$/;function n(n){let r=Array.isArray(n)?n:n&&typeof n==`object`&&Array.isArray(n.versions)?n.versions:null;if(!r||r.length===0)return null;let i=r[0];if(!i||typeof i!=`object`)return null;let{version:a,execution_id:o}=i;return typeof a!=`string`||!e.test(a)||typeof o!=`string`||!t.test(o)?null:{version:a,executionId:o}}function r(e,t,n){let r=`https://storage.googleapis.com/antigravity-public/antigravity-hub/${e}-${t}`;return n===`darwin-arm`||n===`darwin-x64`?`${r}/${n}/Antigravity.dmg`:n===`windows-x64`?`${r}/windows-x64/${e===`2.0.0`?`Antigravity.exe`:`Antigravity-x64.exe`}`:n===`windows-arm64`?`${r}/windows-arm/${e===`2.0.0`?`Antigravity.exe`:`Antigravity-arm64.exe`}`:n===`linux-x64`?`${r}/linux-x64/Antigravity.tar.gz`:n===`linux-arm`?`${r}/linux-arm/Antigravity.tar.gz`:`${r}/${n}/Antigravity.dmg`}var i=null,a=null;async function o(){return i&&Date.now()-i.fetchedAt<3e5?i.build:a||(a=(async()=>{let e=new AbortController,t=setTimeout(()=>e.abort(),8e3);try{let t=await fetch(`https://antigravity-hub-auto-updater-974169037036.us-central1.run.app/releases`,{cache:`no-store`,signal:e.signal});if(!t.ok)return null;let r=n(await t.json());return r&&(i={build:r,fetchedAt:Date.now()}),r}catch{return null}finally{clearTimeout(t),a=null}})(),a)}function s(e,t=document){t.querySelectorAll(`a[data-hub-platform]`).forEach(t=>{let n=t.getAttribute(`data-hub-platform`);n&&(t.href=r(e.version,e.executionId,n),t.setAttribute(`data-hub-version`,e.version))}),t.querySelectorAll(`[data-hub-latest-version]`).forEach(t=>{t.textContent=`v${e.version}`})}async function c(){if(!document.querySelector(`a[data-hub-platform]`))return;let e=await o();e&&s(e)}var l=!1;function u(){document.readyState===`loading`?document.addEventListener(`DOMContentLoaded`,()=>{c()}):c(),l||(l=!0,document.addEventListener(`astro:page-load`,()=>{c()}),window.addEventListener(`agy:page-ready`,()=>{c()}))}u();</script></div><div class="sl-flex social-icons astro-mtusqnhg"></div><starlight-theme-select><label style="--sl-select-width: 6.25em" class="astro-r2p65idu"><span class="sr-only astro-r2p65idu">Select theme</span><svg aria-hidden="true" class="icon label-icon astro-r2p65idu astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1em;"><path d="M21 14h-1V7a3 3 0 0 0-3-3H7a3 3 0 0 0-3 3v7H3a1 1 0 0 0-1 1v2a3 3 0 0 0 3 3h14a3 3 0 0 0 3-3v-2a1 1 0 0 0-1-1ZM6 7a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v7H6V7Zm14 10a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1v-1h16v1Z"/></svg><select autocomplete="off" class="astro-r2p65idu"><option value="dark" class="astro-r2p65idu">Dark</option><option value="light" class="astro-r2p65idu">Light</option><option value="auto" selected class="astro-r2p65idu">Auto</option></select><svg aria-hidden="true" class="icon caret astro-r2p65idu astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1em;"><path d="M17 9.17a1 1 0 0 0-1.41 0L12 12.71 8.46 9.17a1 1 0 1 0-1.41 1.42l4.24 4.24a1.002 1.002 0 0 0 1.42 0L17 10.59a1.002 1.002 0 0 0 0-1.42Z"/></svg></label></starlight-theme-select>\<script\>
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
</script><script type="module" src="/_astro/LanguageSelector.astro_astro_type_script_index_0_lang.B8alFdNs.js"></script></div></div><!-- Row 2: Subnav Bar below logo and controls (Get Started, Features, etc.) --><nav class="subnav-bar print:hidden astro-mtusqnhg" aria-label="Documentation sections"><div class="subnav-container astro-mtusqnhg"><div class="subnav-left astro-mtusqnhg"><a href="/docs/overview/" class="subnav-pill astro-mtusqnhg">Get Started</a><a href="/docs/features/" class="subnav-pill active astro-mtusqnhg" aria-current="page">Features</a><a href="/docs/sdk/overview/" class="subnav-pill astro-mtusqnhg">SDK</a></div><div class="subnav-right astro-mtusqnhg"><a href="/docs/changelog/" class="subnav-pill astro-mtusqnhg">Changelog</a><a href="/blog/" class="subnav-pill external-link blog-link astro-mtusqnhg">Blog <span class="arrow astro-mtusqnhg" aria-hidden="true">↗</span></a></div></div></nav><!-- Row 3: Dismissible Top Banners under Docs Menu --><div class="docs-top-banner-slot print:hidden astro-mtusqnhg" data-docs-announcement-slot="top-banner"><script type="module" src="/_astro/AnnouncementPill.astro_astro_type_script_index_0_lang.BbFE-USi.js"></script><div class="announcement-pill announcement-pill--surface-docs announcement-pill--host-docs announcement-pill--placement-top-banner announcement-pill--dark is-interactive astro-mtusqnhg astro-pyiv6lj6" role="region" aria-label="New announcement" data-announcement-pill data-pill-surface="docs" data-pill-host-surface="docs" data-pill-placement="top-banner" data-pill-variant="dark" data-pill-icon-name="spark" data-pill-badge="New" data-cta-text="Learn more" data-destination="/blog/gemini-4-argon-in-google-antigravity" data-pill-storage-key="agy_announcement_dismissed_docs" data-pill-fingerprint="New|Gemini 4 Argon available for Google AI Pro and Ultra in Google Antigravity|/blog/gemini-4-argon-in-google-antigravity" data-pill-ttl-ms="604800000" data-analytics-event="announcement-pill" data-module-name="docs_top_banner" data-page-position="top-banner"><a href="/blog/gemini-4-argon-in-google-antigravity" class="pill-main-link astro-pyiv6lj6" data-pill-link><span class="pill-badge astro-pyiv6lj6" data-pill-badge><span class="pill-icon astro-pyiv6lj6" data-pill-icon aria-hidden="true"><svg class="pill-spark-icon astro-pyiv6lj6" viewBox="0 0 16 16" width="12" height="12" aria-hidden="true"><path d="M8 0.5C8.5 4.6 11.4 7.5 15.5 8C11.4 8.5 8.5 11.4 8 15.5C7.5 11.4 4.6 8.5 0.5 8C4.6 7.5 7.5 4.6 8 0.5Z" fill="currentColor" class="astro-pyiv6lj6"></path></svg></span><span class="pill-badge-label astro-pyiv6lj6" data-pill-badge-label>New</span></span><span class="pill-text astro-pyiv6lj6" data-pill-text>Gemini 4 Argon available for Google AI Pro and Ultra in Google Antigravity</span><span class="pill-cta astro-pyiv6lj6" data-pill-cta><span class="pill-cta-label astro-pyiv6lj6" data-pill-cta-label>Learn more</span><svg class="pill-arrow astro-pyiv6lj6" viewBox="0 0 16 16" width="13" height="13" fill="none" aria-hidden="true"><path d="M3.5 8H12.5M12.5 8L8.5 4M12.5 8L8.5 12" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" class="astro-pyiv6lj6"></path></svg></span></a><button type="button" class="pill-dismiss astro-pyiv6lj6" data-pill-dismiss data-analytics-event="" aria-label="Dismiss New announcement" title="Dismiss announcement"><svg viewBox="0 0 16 16" width="14" height="14" fill="none" aria-hidden="true" class="astro-pyiv6lj6"><path d="M4 4L12 12M12 4L4 12" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" class="astro-pyiv6lj6"></path></svg></button></div>\<script\>
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
      </script></div></div></header><nav class="sidebar print:hidden astro-gqpr5s5a" aria-label="Main"><starlight-menu-button class="print:hidden astro-vvx7mffu"><button aria-expanded="false" aria-label="Menu" aria-controls="starlight__sidebar" class="sl-flex md:sl-hidden astro-vvx7mffu"><svg aria-hidden="true" class="open-menu astro-vvx7mffu astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1em;"><path d="M3 8h18a1 1 0 1 0 0-2H3a1 1 0 0 0 0 2Zm18 8H3a1 1 0 0 0 0 2h18a1 1 0 0 0 0-2Zm0-5H3a1 1 0 0 0 0 2h18a1 1 0 0 0 0-2Z"/></svg><svg aria-hidden="true" class="close-menu astro-vvx7mffu astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1em;"><path d="m13.41 12 6.3-6.29a1.004 1.004 0 1 0-1.42-1.42L12 10.59l-6.29-6.3a1.004 1.004 0 0 0-1.42 1.42l6.3 6.29-6.3 6.29a1 1 0 0 0 0 1.42.998.998 0 0 0 1.42 0l6.29-6.3 6.29 6.3a.999.999 0 0 0 1.42 0 1 1 0 0 0 0-1.42L13.41 12Z"/></svg></button></starlight-menu-button><script type="module">var e=class extends HTMLElement{btn=this.querySelector(`button`);constructor(){super(),this.btn.addEventListener(`click`,()=>this.toggleExpanded());let e=this.closest(`nav`);e&&e.addEventListener(`keyup`,e=>this.closeOnEscape(e)),matchMedia(`(min-width: 50em)`).addEventListener(`change`,()=>this.setExpanded(!1))}setExpanded(e){this.setAttribute(`aria-expanded`,String(e)),document.body.toggleAttribute(`data-mobile-menu-expanded`,e),document.querySelectorAll(`.main-frame, .sl-skip-link`).forEach(t=>{t.toggleAttribute(`inert`,e)})}toggleExpanded(){this.setExpanded(this.getAttribute(`aria-expanded`)!==`true`)}closeOnEscape(e){e.code===`Escape`&&(this.setExpanded(!1),this.btn.focus())}};customElements.define(`starlight-menu-button`,e);</script><div id="starlight__sidebar" class="sidebar-pane astro-gqpr5s5a"><div class="sidebar-content sl-flex astro-gqpr5s5a"><div class="sidebar-wrapper astro-s3sbtxwf" data-default-section="features" data-analytics-nav="docs_sidebar"><sl-sidebar-state-persist data-hash="1m4v1wn" class="astro-f5yvicm7"><script aria-hidden="true">
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
	</script><div class="sidebar-sections astro-s3sbtxwf" data-sidebar-section="get-started"><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">Get Started</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/overview/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/getting-started/" class="astro-ojxuodky"><span class="astro-ojxuodky">Quickstart</span></a></li><li class="astro-ojxuodky"><a href="/docs/install/" class="astro-ojxuodky"><span class="astro-ojxuodky">Authenticate</span></a></li><li class="astro-ojxuodky"><a href="/docs/models/" class="astro-ojxuodky"><span class="astro-ojxuodky">Choose a model</span></a></li><li class="astro-ojxuodky"><a href="/docs/build-with-google/" class="astro-ojxuodky"><span class="astro-ojxuodky">Build with Google</span></a></li></ul></section><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">IDE Extensions</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/ide/extensions/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/extensions/vscode/" class="astro-ojxuodky"><span class="astro-ojxuodky">Visual Studio Code</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/extensions/visual-studio/" class="astro-ojxuodky"><span class="astro-ojxuodky">Visual Studio</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/extensions/jetbrains/" class="astro-ojxuodky"><span class="astro-ojxuodky">JetBrains IDEs</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/extensions/zed/" class="astro-ojxuodky"><span class="astro-ojxuodky">Zed</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/extensions/xcode/" class="astro-ojxuodky"><span class="astro-ojxuodky">Xcode</span></a></li></ul></section><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">CLI reference</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/cli/reference/" class="astro-ojxuodky"><span class="astro-ojxuodky">CLI commands</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/features/" class="astro-ojxuodky"><span class="astro-ojxuodky">CLI features</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/using/" class="astro-ojxuodky"><span class="astro-ojxuodky">CLI settings and tips</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/troubleshooting/" class="astro-ojxuodky"><span class="astro-ojxuodky">CLI troubleshooting</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/best-practices/" class="astro-ojxuodky"><span class="astro-ojxuodky">CLI best practices</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/credits/" class="astro-ojxuodky"><span class="astro-ojxuodky">CLI AI credits</span></a></li></ul></section><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">Migrations</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/cli/gcli-migration/" class="astro-ojxuodky"><span class="astro-ojxuodky">Gemini CLI</span></a></li><li class="astro-ojxuodky"><a href="/docs/firebase-studio-migration/" class="astro-ojxuodky"><span class="astro-ojxuodky">Firebase Studio</span></a></li><li class="astro-ojxuodky"><a href="/docs/migration/workflows-to-skills/" class="astro-ojxuodky"><span class="astro-ojxuodky">Workflows to Skills</span></a></li></ul></section><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">Resources</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/plans/" class="astro-ojxuodky"><span class="astro-ojxuodky">Plans</span></a></li><li class="astro-ojxuodky"><a href="/docs/enterprise/" class="astro-ojxuodky"><span class="astro-ojxuodky">Enterprise Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/faq/" class="astro-ojxuodky"><span class="astro-ojxuodky">FAQ</span></a></li></ul></section></div><div class="sidebar-sections is-active astro-s3sbtxwf" data-sidebar-section="features"><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">LEARN</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/features/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/remote-control/" aria-current="page" class="astro-ojxuodky"><span class="astro-ojxuodky">Remote Control</span></a></li><li class="astro-ojxuodky"><a href="/docs/marketplace/" class="astro-ojxuodky"><span class="astro-ojxuodky">Marketplace</span></a></li><li class="astro-ojxuodky"><a href="/docs/plugins/" class="astro-ojxuodky"><span class="astro-ojxuodky">Plugins</span></a></li><li class="astro-ojxuodky"><a href="/docs/sandbox/" class="astro-ojxuodky"><span class="astro-ojxuodky">Terminal sandbox</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/review-changes-editor/" class="astro-ojxuodky"><span class="astro-ojxuodky">Review changes</span></a></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Terminal</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="0"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/cli/modes/" class="astro-ojxuodky"><span class="astro-ojxuodky">Execution modes</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/headless/" class="astro-ojxuodky"><span class="astro-ojxuodky">Headless mode</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/statusline/" class="astro-ojxuodky"><span class="astro-ojxuodky">Status line customization</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/title/" class="astro-ojxuodky"><span class="astro-ojxuodky">Terminal title customization</span></a></li></ul></details></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Browser</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="1"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/ide/browser/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/browser-recordings/" class="astro-ojxuodky"><span class="astro-ojxuodky">Browser recordings</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/allowlist-denylist/" class="astro-ojxuodky"><span class="astro-ojxuodky">Allowlist and denylist</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/separate-chrome-profile/" class="astro-ojxuodky"><span class="astro-ojxuodky">Separate Chrome profile</span></a></li></ul></details></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Artifacts</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="2"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/artifacts/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/implementation-plan/" class="astro-ojxuodky"><span class="astro-ojxuodky">Implementation plan</span></a></li><li class="astro-ojxuodky"><a href="/docs/walkthrough/" class="astro-ojxuodky"><span class="astro-ojxuodky">Walkthrough</span></a></li><li class="astro-ojxuodky"><a href="/docs/screenshots/" class="astro-ojxuodky"><span class="astro-ojxuodky">Visual screenshots</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/artifacts/" class="astro-ojxuodky"><span class="astro-ojxuodky">Reviewing artifacts (CLI)</span></a></li></ul></details></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Projects</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="3"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/projects/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/prompting/" class="astro-ojxuodky"><span class="astro-ojxuodky">Prompting and interaction</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/conversations/" class="astro-ojxuodky"><span class="astro-ojxuodky">Managing conversations</span></a></li></ul></details></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Permissions</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="4"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/permissions/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li></ul></details></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Editor features</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="5"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/ide/tab/" class="astro-ojxuodky"><span class="astro-ojxuodky">Tab completion</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/agent-side-panel/" class="astro-ojxuodky"><span class="astro-ojxuodky">Agent side panel</span></a></li><li class="astro-ojxuodky"><a href="/docs/agent/" class="astro-ojxuodky"><span class="astro-ojxuodky">Agent overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/tools/" class="astro-ojxuodky"><span class="astro-ojxuodky">Task groups</span></a></li></ul></details></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Settings</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="6"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/settings/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/agent-settings/" class="astro-ojxuodky"><span class="astro-ojxuodky">Agent settings</span></a></li><li class="astro-ojxuodky"><a href="/docs/artifact-review/" class="astro-ojxuodky"><span class="astro-ojxuodky">Artifact review</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/vim-editor-mode/" class="astro-ojxuodky"><span class="astro-ojxuodky">Vim editor mode</span></a></li></ul></details></li></ul></section><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">SLASH COMMANDS</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/slash-commands/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/agents/" class="astro-ojxuodky"><span class="astro-ojxuodky">/agents</span></a></li><li class="astro-ojxuodky"><a href="/docs/boost/" class="astro-ojxuodky"><span class="astro-ojxuodky">/boost</span></a></li><li class="astro-ojxuodky"><a href="/docs/commands/browser/" class="astro-ojxuodky"><span class="astro-ojxuodky">/browser</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/btw/" class="astro-ojxuodky"><span class="astro-ojxuodky">/btw</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/changelog/" class="astro-ojxuodky"><span class="astro-ojxuodky">/changelog</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/codesearch/" class="astro-ojxuodky"><span class="astro-ojxuodky">/codesearch</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/context/" class="astro-ojxuodky"><span class="astro-ojxuodky">/context</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/credits/" class="astro-ojxuodky"><span class="astro-ojxuodky">/credits</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/diff/" class="astro-ojxuodky"><span class="astro-ojxuodky">/diff</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/effort/" class="astro-ojxuodky"><span class="astro-ojxuodky">/effort</span></a></li><li class="astro-ojxuodky"><a href="/docs/commands/goal/" class="astro-ojxuodky"><span class="astro-ojxuodky">/goal</span></a></li><li class="astro-ojxuodky"><a href="/docs/commands/grill-me/" class="astro-ojxuodky"><span class="astro-ojxuodky">/grill-me</span></a></li><li class="astro-ojxuodky"><a href="/docs/commands/learn/" class="astro-ojxuodky"><span class="astro-ojxuodky">/learn</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/permissions/" class="astro-ojxuodky"><span class="astro-ojxuodky">/permissions</span></a></li><li class="astro-ojxuodky"><a href="/docs/plan/" class="astro-ojxuodky"><span class="astro-ojxuodky">/plan</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/resume/" class="astro-ojxuodky"><span class="astro-ojxuodky">/resume</span></a></li><li class="astro-ojxuodky"><a href="/docs/commands/schedule/" class="astro-ojxuodky"><span class="astro-ojxuodky">/schedule</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/statusline/" class="astro-ojxuodky"><span class="astro-ojxuodky">/statusline</span></a></li><li class="astro-ojxuodky"><a href="/docs/teamwork/" class="astro-ojxuodky"><span class="astro-ojxuodky">/teamwork-preview</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/title/" class="astro-ojxuodky"><span class="astro-ojxuodky">/title</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/usage/" class="astro-ojxuodky"><span class="astro-ojxuodky">/usage</span></a></li><li class="astro-ojxuodky"><a href="/docs/cli/commands/voice/" class="astro-ojxuodky"><span class="astro-ojxuodky">/voice</span></a></li></ul></section><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">EXTEND</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/mcp/" class="astro-ojxuodky"><span class="astro-ojxuodky">MCP</span></a></li><li class="astro-ojxuodky"><details class="astro-ojxuodky"><summary class="astro-ojxuodky"><span class="group-label astro-ojxuodky"><span class="large astro-ojxuodky">Skills</span></span><svg aria-hidden="true" class="caret astro-ojxuodky astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.25rem;"><path d="m14.83 11.29-4.24-4.24a1 1 0 1 0-1.42 1.41L12.71 12l-3.54 3.54a1 1 0 0 0 0 1.41 1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.24-4.24a1.002 1.002 0 0 0 0-1.42Z"/></svg></summary><sl-sidebar-restore data-index="7"></sl-sidebar-restore><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/skills/" class="astro-ojxuodky"><span class="astro-ojxuodky">Agent skills</span></a></li><li class="astro-ojxuodky"><a href="/docs/rules/" class="astro-ojxuodky"><span class="astro-ojxuodky">Rules</span></a></li><li class="astro-ojxuodky"><a href="/docs/ide/workflows/" class="astro-ojxuodky"><span class="astro-ojxuodky">IDE workflows</span></a></li></ul></details></li><li class="astro-ojxuodky"><a href="/docs/subagents/" class="astro-ojxuodky"><span class="astro-ojxuodky">Custom subagents</span></a></li><li class="astro-ojxuodky"><a href="/docs/hooks/" class="astro-ojxuodky"><span class="astro-ojxuodky">Lifecycle hooks</span></a></li><li class="astro-ojxuodky"><a href="/docs/sidecars/" class="astro-ojxuodky"><span class="astro-ojxuodky">Sidecars</span></a></li></ul></section></div><div class="sidebar-sections astro-s3sbtxwf" data-sidebar-section="sdk"><section class="sidebar-group-section astro-s3sbtxwf"><h2 class="sidebar-section-heading astro-s3sbtxwf">ANTIGRAVITY SDK</h2><ul class="astro-ojxuodky"><li class="astro-ojxuodky"><a href="/docs/sdk/overview/" class="astro-ojxuodky"><span class="astro-ojxuodky">Overview and quickstart</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/local-models/" class="astro-ojxuodky"><span class="astro-ojxuodky">Local models</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/personas/" class="astro-ojxuodky"><span class="astro-ojxuodky">Personas</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/tools/" class="astro-ojxuodky"><span class="astro-ojxuodky">Tools and skills</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/mcp/" class="astro-ojxuodky"><span class="astro-ojxuodky">MCP</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/policies/" class="astro-ojxuodky"><span class="astro-ojxuodky">Policies</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/subagents/" class="astro-ojxuodky"><span class="astro-ojxuodky">Subagents</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/structured-output/" class="astro-ojxuodky"><span class="astro-ojxuodky">Structured output</span></a></li><li class="astro-ojxuodky"><a href="/docs/sdk/lifecycle/" class="astro-ojxuodky"><span class="astro-ojxuodky">Lifecycle and hooks</span></a></li></ul></section></div><div class="sidebar-sections astro-s3sbtxwf" data-sidebar-section="changelog"></div><script aria-hidden="true">
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
            " class="astro-sgnutmzh"></path></svg></span><span class="display-current astro-sgnutmzh"></span></summary><div class="dropdown astro-sgnutmzh"><ul class="isMobile astro-sgnutmzh"><li class="astro-sgnutmzh"><a href="#_top" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Overview</span></a></li><li class="astro-sgnutmzh"><a href="#enabling-remote-control-in-the-antigravity-app" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Enabling Remote Control in the Antigravity App</span></a><ul class="astro-sgnutmzh"><li class="astro-sgnutmzh"><a href="#connecting-from-a-web-browser" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Connecting from a web browser</span></a></li></ul></li><li class="astro-sgnutmzh"><a href="#desktop-troubleshooting" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Desktop troubleshooting</span></a><ul class="astro-sgnutmzh"><li class="astro-sgnutmzh"><a href="#antigravity-app-desktop" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Antigravity App desktop</span></a></li></ul></li><li class="astro-sgnutmzh"><a href="#overview" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Overview</span></a></li><li class="astro-sgnutmzh"><a href="#interactive-mode" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Interactive mode</span></a><ul class="astro-sgnutmzh"><li class="astro-sgnutmzh"><a href="#enable-remote-control-in-an-active-session-remote-control" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Enable Remote Control in an active session (/remote-control)</span></a></li><li class="astro-sgnutmzh"><a href="#launch-directly-with-remote-control---remote-control" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Launch directly with Remote Control (--remote-control)</span></a></li><li class="astro-sgnutmzh"><a href="#how-interactive-remote-control-works" class="astro-sgnutmzh"><span class="astro-sgnutmzh">How interactive Remote Control works</span></a></li></ul></li><li class="astro-sgnutmzh"><a href="#remote-control-headless-daemon" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Remote Control headless daemon</span></a><ul class="astro-sgnutmzh"><li class="astro-sgnutmzh"><a href="#managing-the-daemon" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Managing the daemon</span></a></li><li class="astro-sgnutmzh"><a href="#windows-notes" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Windows notes</span></a></li><li class="astro-sgnutmzh"><a href="#sign-in" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Sign-in</span></a></li><li class="astro-sgnutmzh"><a href="#naming-your-machine-headless-daemon" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Naming your machine (headless daemon)</span></a></li><li class="astro-sgnutmzh"><a href="#how-the-daemon-runs" class="astro-sgnutmzh"><span class="astro-sgnutmzh">How the daemon runs</span></a></li></ul></li><li class="astro-sgnutmzh"><a href="#interactive-mode-vs-headless-background-daemon" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Interactive mode vs. headless background daemon</span></a></li><li class="astro-sgnutmzh"><a href="#cli-troubleshooting" class="astro-sgnutmzh"><span class="astro-sgnutmzh">CLI troubleshooting</span></a><ul class="astro-sgnutmzh"><li class="astro-sgnutmzh"><a href="#interactive-mode-issues" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Interactive mode issues</span></a></li><li class="astro-sgnutmzh"><a href="#headless-daemon-issues" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Headless daemon issues</span></a></li></ul></li><li class="astro-sgnutmzh"><a href="#next-steps" class="astro-sgnutmzh"><span class="astro-sgnutmzh">Next steps</span></a></li></ul></div></details></nav></mobile-starlight-toc><script type="module" src="/_astro/MobileTableOfContentsOverride.astro_astro_type_script_index_0_lang.BhmNkhRh.js"></script></div><div class="right-sidebar-panel sl-hidden lg:sl-block astro-q5mffni3"><div class="sl-container astro-q5mffni3"><starlight-toc data-min-h="2" data-max-h="3"><nav aria-labelledby="starlight__on-this-page" data-analytics-nav="docs_toc"><h2 id="starlight__on-this-page">On this page</h2><ul class="starlight-toc-list">\<li\><a href="#_top"><span>Overview</span></a></li>\<li\><a href="#enabling-remote-control-in-the-antigravity-app"><span>Enabling Remote Control in the Antigravity App</span></a>\<ul\>\<li\><a href="#connecting-from-a-web-browser"><span>Connecting from a web browser</span></a></li></ul></li>\<li\><a href="#desktop-troubleshooting"><span>Desktop troubleshooting</span></a>\<ul\>\<li\><a href="#antigravity-app-desktop"><span>Antigravity App desktop</span></a></li></ul></li>\<li\><a href="#overview"><span>Overview</span></a></li>\<li\><a href="#interactive-mode"><span>Interactive mode</span></a>\<ul\>\<li\><a href="#enable-remote-control-in-an-active-session-remote-control"><span>Enable Remote Control in an active session (/remote-control)</span></a></li>\<li\><a href="#launch-directly-with-remote-control---remote-control"><span>Launch directly with Remote Control (--remote-control)</span></a></li>\<li\><a href="#how-interactive-remote-control-works"><span>How interactive Remote Control works</span></a></li></ul></li>\<li\><a href="#remote-control-headless-daemon"><span>Remote Control headless daemon</span></a>\<ul\>\<li\><a href="#managing-the-daemon"><span>Managing the daemon</span></a></li>\<li\><a href="#windows-notes"><span>Windows notes</span></a></li>\<li\><a href="#sign-in"><span>Sign-in</span></a></li>\<li\><a href="#naming-your-machine-headless-daemon"><span>Naming your machine (headless daemon)</span></a></li>\<li\><a href="#how-the-daemon-runs"><span>How the daemon runs</span></a></li></ul></li>\<li\><a href="#interactive-mode-vs-headless-background-daemon"><span>Interactive mode vs. headless background daemon</span></a></li>\<li\><a href="#cli-troubleshooting"><span>CLI troubleshooting</span></a>\<ul\>\<li\><a href="#interactive-mode-issues"><span>Interactive mode issues</span></a></li>\<li\><a href="#headless-daemon-issues"><span>Headless daemon issues</span></a></li></ul></li>\<li\><a href="#next-steps"><span>Next steps</span></a></li></ul></nav></starlight-toc><script type="module">var e=class extends HTMLElement{_current=null;_onScroll=null;_observer=null;_cleanup=[];set current(e){e!==this._current&&(this._current&&this._current.removeAttribute(`aria-current`),e.setAttribute(`aria-current`,`true`),this._current=e)}connectedCallback(){this.init(),window.addEventListener(`agy:page-ready`,this.reinit)}disconnectedCallback(){window.removeEventListener(`agy:page-ready`,this.reinit),this.cleanup()}cleanup(){this._current=null,this._onScroll&&=(window.removeEventListener(`scroll`,this._onScroll),null),this._observer&&=(this._observer.disconnect(),null),this._cleanup.forEach(e=>e()),this._cleanup=[]}reinit=()=>{this.cleanup(),this.init()};init=()=>{let e=Array.from(this.querySelectorAll(`a`));if(!e.length)return;let t=[],n=new Map,r=document.getElementById(`_top`);if(r){let i=e.find(e=>e.hash===`#_top`);if(i){n.set(`_top`,i);let e=i.closest(`li`);e&&t.push({link:i,li:e,heading:r,tabpanel:null,surfacePane:null})}}e.forEach(e=>{let r=decodeURIComponent(e.hash.slice(1));if(!r||r===`_top`)return;let i=document.getElementById(r),a=e.closest(`li`);if(i&&a){n.set(r,e);let o=i.closest(`[role="tabpanel"]`),s=i.closest(`.surface-pane`);t.push({link:e,li:a,heading:i,tabpanel:o,surfacePane:s})}});let i=e=>e?e.hidden||e.hasAttribute(`hidden`)||e.style.display===`none`:!1,a=e=>{if(!e)return!1;if(e.hidden||e.hasAttribute(`hidden`))return!0;if(e.style.display)return e.style.display===`none`;let t=e.getAttribute(`data-surface-pane`);return t?t!==`hub`:getComputedStyle(e).display===`none`},o=!1,s=()=>{o||(o=!0,requestAnimationFrame(c))},c=()=>{if(o=!1,window.innerWidth<800||this.offsetParent===null)return;let e=t.filter(e=>e.heading.id===`_top`||!e.heading.hidden&&e.heading.style.display!==`none`&&!i(e.tabpanel)&&!a(e.surfacePane));if(!e.length)return;let r=e[0];if(window.scrollY>10)for(let t of e)if(t.heading.getBoundingClientRect().top<=130)r=t;else break;let s=n.get(r.heading.id);s&&(this.current=s)},l=(e=!1)=>{window.innerWidth<800||(t.forEach(e=>{let t=i(e.tabpanel)||a(e.surfacePane),n=t?`none`:``;e.li.style.display!==n&&(e.li.style.display=n),t?e.li.getAttribute(`data-tab-hidden`)!==`true`&&e.li.setAttribute(`data-tab-hidden`,`true`):e.li.hasAttribute(`data-tab-hidden`)&&e.li.removeAttribute(`data-tab-hidden`)}),e?c():s())},u=e=>{let t=e.target?.closest(`a`);if(!t||!this.contains(t))return;let n=decodeURIComponent(t.hash.slice(1));if(!n)return;let r=document.getElementById(n);if(r){let e=r.closest(`.surface-pane[data-surface-pane]`);if(e&&a(e)){let t=e.getAttribute(`data-surface-pane`);t&&(document.querySelector(`.surface-card-btn[data-surface-id="${t}"]`)?.click(),requestAnimationFrame(()=>{l()}))}let t=r.closest(`starlight-tabs [role="tabpanel"]`);if(t){let e=t.closest(`starlight-tabs`);if(e){let n=Array.from(e.querySelectorAll(`:scope > [role="tabpanel"]`)).indexOf(t);n!==-1&&(e.querySelectorAll(`[role="tab"]`)[n]?.click(),requestAnimationFrame(()=>{l()}))}}}};this.addEventListener(`click`,u),this._cleanup.push(()=>{this.removeEventListener(`click`,u)});let d=document.querySelectorAll(`[role="tabpanel"], .surface-pane`);if(d.length>0){let e=0,t,n=()=>{e||=requestAnimationFrame(()=>{e=0,clearTimeout(t),t=setTimeout(()=>{l()},0)})};this._observer=new MutationObserver(n),d.forEach(e=>{this._observer?.observe(e,{attributes:!0,attributeFilter:[`hidden`,`style`,`class`]})}),document.addEventListener(`ag-surface-changed`,n),this._cleanup.push(()=>{document.removeEventListener(`ag-surface-changed`,n)})}this._onScroll=s,window.addEventListener(`scroll`,this._onScroll,{passive:!0}),l(!0)}};customElements.get(`starlight-toc`)||customElements.define(`starlight-toc`,e);</script></div></div></div></aside><div class="main-pane astro-abfeecmu"><main data-pagefind-body class="astro-vdxcvpsm" lang="en" dir="ltr"><div class="content-panel astro-oil7k7jf"><div class="sl-container astro-oil7k7jf"><div class="pagefind-search-meta sr-only astro-hkwnjoai" data-i18n-ignore="true"><span data-pagefind-meta="title" class="astro-hkwnjoai">Remote Control</span><span data-pagefind-meta="surface" class="astro-hkwnjoai">2-0</span><span data-pagefind-meta="surfaceLabel" class="astro-hkwnjoai">Antigravity App</span><span data-pagefind-meta="section" class="astro-hkwnjoai">Features</span><span data-pagefind-weight="6" class="astro-hkwnjoai">Remote Control Antigravity App Features</span><span data-pagefind-filter="surface:2-0" class="astro-hkwnjoai">Antigravity App</span><span data-pagefind-filter="surface:cli" class="astro-hkwnjoai">Antigravity CLI</span></div><div class="breadcrumbs-row astro-hkwnjoai" data-pagefind-ignore><nav aria-label="Breadcrumb" class="starlight-breadcrumbs astro-hkwnjoai"><span class="breadcrumb-item  astro-hkwnjoai" title="Features">Features</span><svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" class="breadcrumb-separator astro-hkwnjoai" aria-hidden="true"><polyline points="9 18 15 12 9 6" class="astro-hkwnjoai"></polyline></svg><span class="breadcrumb-item  astro-hkwnjoai" title="Learn">Learn</span><svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" class="breadcrumb-separator astro-hkwnjoai" aria-hidden="true"><polyline points="9 18 15 12 9 6" class="astro-hkwnjoai"></polyline></svg><span class="breadcrumb-item breadcrumb-last astro-hkwnjoai" title="Remote Control">Remote Control</span></nav><div id="md-actions-root" class="markdown-dropdown-wrapper astro-pyacvpuh" data-pathname="/docs/remote-control/"><button type="button" class="call-to-action--nav button button-primary button-compact markdown-trigger-btn dropdown-nav" aria-haspopup="true" aria-expanded="false" aria-label="Markdown Options" class="astro-pyacvpuh"><span>Markdown</span><span class="dropdown-icon" translate="no"><svg class viewBox="0 0 24 24" width="18" height="18" fill="currentColor" aria-hidden="true" xmlns="http://www.w3.org/2000/svg"><path d="M12 15L7.05 10.05L8.45 8.65L12 12.2L15.55 8.65L16.95 10.05L12 15Z"></path></svg></span></button><div class="markdown-dropdown-menu astro-pyacvpuh" data-md-menu hidden><button type="button" class="markdown-menu-item astro-pyacvpuh" data-copy-md><svg class="icon astro-pyacvpuh" viewBox="0 -960 960 960" width="16px" height="16px" fill="currentColor" aria-hidden="true" xmlns="http://www.w3.org/2000/svg"><path d="M318.46-230.77q-23.53,0-40.61-17.08t-17.08-40.61V-802.31q0-23.53 17.08-40.61T318.46-860H712.31q23.53,0 40.61,17.08T770-802.31v513.84q0,23.53-17.08,40.61t-40.61,17.08H318.46Zm0-45.38H712.31q4.62,0 8.46-3.85t3.85-8.46V-802.31q0-4.62-3.85-8.46t-8.46-3.85H318.46q-4.62,0-8.46,3.85t-3.85,8.46v513.84q0,4.62 3.85,8.46t8.46,3.85ZM207.69-120q-23.53,0-40.61-17.08T150-177.7V-736.92h45.38V-177.7q0,4.62 3.85,8.46t8.46,3.85H646.92V-120H207.69Zm98.46-156.15q0,0 0-3.85t0-8.46V-802.31q0-4.62 0-8.46t0-3.85q0,0 0,3.85t0,8.46v513.84q0,4.62 0,8.46t0,3.85Z"></path></svg><span class="item-label astro-pyacvpuh">Copy Markdown</span></button><a href="#" data-view-md-link target="_blank" rel="noopener noreferrer" class="markdown-menu-item astro-pyacvpuh"><svg class="icon astro-pyacvpuh" viewBox="0 -960 960 960" width="16px" height="16px" fill="currentColor" aria-hidden="true" xmlns="http://www.w3.org/2000/svg"><path d="M197.69-140q-23.53,0-40.61-17.08T140-197.69V-762.31q0-23.53 17.08-40.61T197.69-820H449.38v45.38H197.69q-4.62,0-8.46,3.85t-3.85,8.46v564.61q0,4.62 3.85,8.46t8.46,3.85H762.31q4.62,0 8.46-3.85t3.85-8.46V-449.38H820v251.69q0,23.53-17.08,40.61T762.31-140H197.69ZM384.31-351.69l-32-32.61L742.62-774.61H530.15V-820H820v289.84H774.61V-742L384.31-351.69Z"></path></svg><span class="item-label astro-pyacvpuh">View Markdown</span></a></div></div>\<script\>
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
</script></div><div class="surface-chips-row not-content astro-hkwnjoai" data-pagefind-ignore><span class="surface-chips-label astro-hkwnjoai">Available on:</span><div class="surface-chips-list astro-hkwnjoai"><span class="surface-chip surface-chip-2-0 astro-hkwnjoai"><svg class="surface-chip-icon astro-hkwnjoai" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><rect x="2" y="3" width="20" height="14" rx="2" ry="2" class="astro-hkwnjoai"></rect><line x1="8" y1="21" x2="16" y2="21" class="astro-hkwnjoai"></line><line x1="12" y1="17" x2="12" y2="21" class="astro-hkwnjoai"></line></svg><span class="astro-hkwnjoai">Antigravity App</span></span><span class="surface-chip surface-chip-cli astro-hkwnjoai"><svg class="surface-chip-icon astro-hkwnjoai" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><polyline points="4 17 10 11 4 5" class="astro-hkwnjoai"></polyline><line x1="12" y1="19" x2="20" y2="19" class="astro-hkwnjoai"></line></svg><span class="astro-hkwnjoai">Antigravity CLI</span></span></div></div><script type="module" src="/_astro/StarlightPageTitleOverride.astro_astro_type_script_index_0_lang.DMV6_9UE.js"></script></div></div><div class="content-panel astro-oil7k7jf"><div class="sl-container astro-oil7k7jf"><div class="sl-markdown-content"><div class="sl-heading-wrapper level-h1"><h1 id="remote-control">Remote Control</h1><a class="sl-anchor-link" href="#remote-control"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Remote Control”</span></a></div>
<p>Antigravity Remote Control allows you to securely connect to and drive your
Antigravity desktop sessions and CLI instances running across your machines
from any web browser.</p>
<p>As AI agents take on larger-scope tasks—such as full-subsystem refactorings, extensive test suite runs, and complex dependency migrations—operations can run for extended periods. Remote Control untethers you from your physical desk while preserving your entire local development environment.</p>
\<script\>
(() => {
	class StarlightTabsRestore extends HTMLElement {
		connectedCallback() {
			const starlightTabs = this.closest('starlight-tabs');
			if (!(starlightTabs instanceof HTMLElement) || typeof localStorage === 'undefined') return;
			const syncKey = starlightTabs.dataset.syncKey;
			if (!syncKey) return;
			const label = localStorage.getItem(`starlight-synced-tabs__${syncKey}`);
			if (!label) return;
			const tabs = [...starlightTabs?.querySelectorAll('[role="tab"]')];
			const tabIndexToRestore = tabs.findIndex(
				(tab) => tab instanceof HTMLAnchorElement && tab.textContent?.trim() === label
			);
			const panels = starlightTabs?.querySelectorAll(':scope > [role="tabpanel"]');
			const newTab = tabs[tabIndexToRestore];
			const newPanel = panels[tabIndexToRestore];
			if (tabIndexToRestore < 1 || !newTab || !newPanel) return;
			tabs[0]?.setAttribute('aria-selected', 'false');
			tabs[0]?.setAttribute('tabindex', '-1');
			panels?.[0]?.setAttribute('hidden', 'true');
			newTab.removeAttribute('tabindex');
			newTab.setAttribute('aria-selected', 'true');
			newPanel.removeAttribute('hidden');
		}
	}
	customElements.define('starlight-tabs-restore', StarlightTabsRestore);
})()
</script><starlight-tabs data-sync-key="surface" class="astro-hbt4ddzs"><div class="tablist-wrapper not-content astro-hbt4ddzs"><ul role="tablist" class="astro-hbt4ddzs"><li role="presentation" class="tab astro-hbt4ddzs"><a role="tab" href="#tab-panel-111" id="tab-111" aria-selected="true" tabindex="0" class="astro-hbt4ddzs">Antigravity App</a></li><li role="presentation" class="tab astro-hbt4ddzs"><a role="tab" href="#tab-panel-112" id="tab-112" aria-selected="false" tabindex="-1" class="astro-hbt4ddzs">Antigravity CLI</a></li></ul></div><div id="tab-panel-111" aria-labelledby="tab-111" role="tabpanel"><div class="sl-heading-wrapper level-h2"><h2 id="sr-tab-1" class="sr-only" data-pagefind-ignore="" data-sr-tab-heading="true">Antigravity App</h2><a class="sl-anchor-link" href="#sr-tab-1"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore="">Section titled “Antigravity App”</span></a></div><div class="sl-heading-wrapper level-h3"><h3 id="enabling-remote-control-in-the-antigravity-app">Enabling Remote Control in the Antigravity App</h3><a class="sl-anchor-link" href="#enabling-remote-control-in-the-antigravity-app"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore="">Section titled “Enabling Remote Control in the Antigravity App”</span></a></div><p>You can enable Remote Control directly from the Antigravity App settings:</p>\<ol\>
\<li\>Open the <strong>Settings</strong> panel by pressing <code dir="auto">Cmd + ,</code> (or <code dir="auto">Ctrl + ,</code> on Linux and Windows), or click <strong>Settings</strong> at the bottom of the left sidebar.</li>
\<li\>Navigate to the <strong>App</strong> section.</li>
\<li\>Toggle <strong>Enable Remote Control</strong> to <strong>On</strong>.</li>
\<li\><em>(Optional)</em> Set a custom <strong>Nickname</strong> (for example, <code dir="auto">workstation-primary</code> or <code dir="auto">server-machine</code>) to easily identify this machine in your instance list.</li>
</ol><div class="sl-heading-wrapper level-h4"><h4 id="connecting-from-a-web-browser">Connecting from a web browser</h4><a class="sl-anchor-link" href="#connecting-from-a-web-browser"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore="">Section titled “Connecting from a web browser”</span></a></div><p>To access your remote Antigravity instance:</p>\<ol\>
\<li\>Open your web browser on any device and navigate to the
<a href="https://antigravity.google.com">Antigravity Remote Control Dashboard</a>. (Tip: On mobile devices, you can optionally install the web app to your home screen to receive push notifications when agents complete tasks or request input.)</li>
\<li\>Sign in with the same Google Account that you used on your desktop application.</li>
\<li\>In the instance switcher, select the machine you want to control.</li>
\<li\>You now have full access to view active conversations, start new agent tasks, review implementation plans, and inspect artifacts.</li>
</ol><div class="sl-heading-wrapper level-h3"><h3 id="desktop-troubleshooting">Desktop troubleshooting</h3><a class="sl-anchor-link" href="#desktop-troubleshooting"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore="">Section titled “Desktop troubleshooting”</span></a></div><div class="sl-heading-wrapper level-h4"><h4 id="antigravity-app-desktop">Antigravity App desktop</h4><a class="sl-anchor-link" href="#antigravity-app-desktop"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore="">Section titled “Antigravity App desktop”</span></a></div><p>If you encounter issues connecting to your desktop instance, review the following troubleshooting steps:</p>\<ul\>
\<li\><strong>Machine does not appear in the web UI</strong>:
\<ul\>
\<li\>Ensure that <strong>Enable Remote Control</strong> is toggled on in the Antigravity App settings on your host machine.</li>
\<li\>Verify that your host machine has an active internet connection and is not asleep or suspended.</li>
\<li\>Check that you are signed in with the same Google Account in both the desktop app and the web browser.</li>
</ul>
</li>
\<li\><strong>Reconnection and disconnects</strong>:
\<ul\>
\<li\>If your local network connectivity drops temporarily, the web interface automatically tries to reconnect. Any running background agent tasks and shell commands continue executing on your host machine uninterrupted as long as the host maintains an internet connection.</li>
</ul>
</li>
</ul></div><div id="tab-panel-112" aria-labelledby="tab-112" role="tabpanel" hidden><div class="sl-heading-wrapper level-h2"><h2 id="sr-tab-2" class="sr-only" data-pagefind-ignore="" data-sr-tab-heading="true">Antigravity CLI</h2><a class="sl-anchor-link" href="#sr-tab-2"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore="true">Section titled “Antigravity CLI”</span></a></div><div class="mp4-viewer-container astro-g2dj7gvy" data-mp4-container="" data-analytics-module="mp4_viewer"><div class="mp4-video-wrapper astro-g2dj7gvy" data-mp4-wrapper=""><video data-src="/assets/video/cli-remote-control-demo.mp4" data-video-src="/assets/video/cli-remote-control-demo.mp4" data-video-provider="mp4" data-module-name="mp4_viewer" data-autoplay="true" preload="none" loop muted playsinline controls width="1280" height="720" aria-label="Video player" class="mp4-viewer-video astro-g2dj7gvy"></video></div></div><script type="module" src="/_astro/Mp4Viewer.astro_astro_type_script_index_0_lang.qhaK0xW1.js"></script><div class="sl-heading-wrapper level-h3"><h3 id="overview">Overview</h3><a class="sl-anchor-link" href="#overview"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore="">Section titled “Overview”</span></a></div><p>Remote Control lets you interact with your CLI instances remotely using the Antigravity desktop UI. Antigravity CLI supports two ways to use Remote Control:</p>\<ol\>
\<li\><strong>Interactive mode</strong>: Start a Remote Control connection in an active CLI instance using <code dir="auto">/remote-control</code> or start it in a new instance by launching the CLI with the <code dir="auto">--remote-control</code> flag. The tunnel stays open only for the lifetime of that CLI instance.</li>
\<li\><strong>Headless background daemon</strong>: Start a connection to an always-on headless CLI instance with <code dir="auto">agy remote-control start</code>. You can interact with this instance remotely as long as the machine running it is on.</li>
</ol>\<hr\><div class="sl-heading-wrapper level-h3"><h3 id="interactive-mode">Interactive mode</h3><a class="sl-anchor-link" href="#interactive-mode"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore="">Section titled “Interactive mode”</span></a></div><div class="sl-heading-wrapper level-h4"><h4 id="enable-remote-control-in-an-active-session-remote-control">Enable Remote Control in an active session (<code dir="auto">/remote-control</code>)</h4><a class="sl-anchor-link" href="#enable-remote-control-in-an-active-session-remote-control"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore="">Section titled “Enable Remote Control in an active session (/remote-control)”</span></a></div><p>You can turn on Remote Control at any point during an interactive <code dir="auto">agy</code> terminal session without restarting the CLI.</p><p>Inside the prompt box, run <code dir="auto">/remote-control</code> (or <code dir="auto">/remote-control on</code>):</p><pre class="astro-code astro-code-themes github-light github-dark" style="background-color:#fff;--shiki-dark-bg:#24292e;color:#24292e;--shiki-dark:#e1e4e8; overflow-x: auto; white-space: pre-wrap; word-wrap: break-word;" tabindex="0" data-language="text" dir="ltr"><code><span class="line"><span>/remote-control</span></span></code></pre><p>The CLI establishes a reverse tunnel for the current session and prints the session’s hostname along with a link to open the active conversation on another device:</p><pre class="astro-code astro-code-themes github-light github-dark" style="background-color:#fff;--shiki-dark-bg:#24292e;color:#24292e;--shiki-dark:#e1e4e8; overflow-x: auto; white-space: pre-wrap; word-wrap: break-word;" tabindex="0" data-language="text" dir="ltr"><code><span class="line"><span>Remote control on for this session.</span></span>
<span class="line"><span>Hostname: devbox-swift-falcon</span></span>
<span class="line"><span>Open https://antigravity.google.com/r/&#x3C;instance-id>?p=c%2F&#x3C;conversation-id> on another device to continue this conversation.</span></span></code></pre><p>Open the printed URL in any web browser (on your laptop, tablet, or phone) signed in to the same Google Account to interact with the session using the Antigravity desktop UI.</p><p>To disconnect the remote tunnel while keeping your local terminal session running, run:</p><pre class="astro-code astro-code-themes github-light github-dark" style="background-color:#fff;--shiki-dark-bg:#24292e;color:#24292e;--shiki-dark:#e1e4e8; overflow-x: auto; white-space: pre-wrap; word-wrap: break-word;" tabindex="0" data-language="text" dir="ltr"><code><span class="line"><span>/remote-control off</span></span></code></pre><pre class="astro-code astro-code-themes github-light github-dark" style="background-color:#fff;--shiki-dark-bg:#24292e;color:#24292e;--shiki-dark:#e1e4e8; overflow-x: auto; white-space: pre-wrap; word-wrap: break-word;" tabindex="0" data-language="text" dir="ltr"><code><span class="line"><span>Remote control off. This machine is no longer reachable from other devices.</span></span>
<span class="line"><span>Hostname: devbox-swift-falcon</span></span></code></pre><div class="sl-heading-wrapper level-h4"><h4 id="launch-directly-with-remote-control---remote-control">Launch directly with Remote Control (<code dir="auto">--remote-control</code>)</h4><a class="sl-anchor-link" href="#launch-directly-with-remote-control---remote-control"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore="">Section titled “Launch directly with Remote Control (--remote-control)”</span></a></div><p>To start a remote connection immediately upon launching an interactive CLI session, use the <code dir="auto">--remote-control</code> flag:</p><pre class="astro-code astro-code-themes github-light github-dark" style="background-color:#fff;--shiki-dark-bg:#24292e;color:#24292e;--shiki-dark:#e1e4e8; overflow-x: auto; white-space: pre-wrap; word-wrap: break-word;" tabindex="0" data-language="bash" dir="ltr"><code><span class="line"><span style="color:#6F42C1;--shiki-dark:#B392F0">agy</span><span style="color:#005CC5;--shiki-dark:#79B8FF"> --remote-control</span></span></code></pre><div class="sl-heading-wrapper level-h4"><h4 id="how-interactive-remote-control-works">How interactive Remote Control works</h4><a class="sl-anchor-link" href="#how-interactive-remote-control-works"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore="">Section titled “How interactive Remote Control works”</span></a></div><div class="sl-heading-wrapper level-h5"><h5 id="session-lifecycle">Session lifecycle</h5><a class="sl-anchor-link" href="#session-lifecycle"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore="">Section titled “Session lifecycle”</span></a></div><p>Interactive Remote Control is strictly tied to the running <code dir="auto">agy</code> terminal process:</p>\<ul\>
\<li\><strong>Automatic cleanup on exit</strong>: When you exit the interactive CLI (<code dir="auto">/exit</code>, <code dir="auto">Ctrl+C</code>, or <code dir="auto">Ctrl+D</code>) or close the terminal window, the Remote Control tunnel automatically terminates and the instance disappears from the dashboard.</li>
\<li\><strong>Concurrent terminal sessions</strong>: Because each interactive CLI process generates its own session instance ID and session hostname, you can run <code dir="auto">agy --remote-control</code> across multiple terminal tabs or project directories simultaneously.</li>
</ul><div class="sl-heading-wrapper level-h5"><h5 id="live-bidirectional-synchronization">Live bidirectional synchronization</h5><a class="sl-anchor-link" href="#live-bidirectional-synchronization"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore="">Section titled “Live bidirectional synchronization”</span></a></div><p>When you connect to your interactive CLI session using the Antigravity desktop UI in your browser, both surfaces stay synchronized in real time:</p>\<ul\>
\<li\><strong>Shared live stream</strong>: Prompts submitted from your remote device stream live into your local terminal TUI as they happen, and thoughts, tool executions, and diffs generated in the terminal appear live in the remote UI.</li>
\<li\><strong>Cross-device tool approvals and questions</strong>: When the agent pauses for a tool confirmation (<code dir="auto">y</code>/<code dir="auto">n</code>) or asks an interactive clarifying question, you can respond from either your local terminal or the remote UI.</li>
\<li\><strong>CLI-controlled settings</strong>: You cannot toggle settings in the settings panel from the remote web interface; to change settings, use the CLI.</li>
</ul>\<hr\><div class="sl-heading-wrapper level-h3"><h3 id="remote-control-headless-daemon">Remote Control headless daemon</h3><a class="sl-anchor-link" href="#remote-control-headless-daemon"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore="">Section titled “Remote Control headless daemon”</span></a></div><p>Use the <code dir="auto">remote-control</code> subcommand to register an always-on headless CLI instance that starts automatically in the background and stays connected whenever your machine is powered on:</p><pre class="astro-code astro-code-themes github-light github-dark" style="background-color:#fff;--shiki-dark-bg:#24292e;color:#24292e;--shiki-dark:#e1e4e8; overflow-x: auto; white-space: pre-wrap; word-wrap: break-word;" tabindex="0" data-language="bash" dir="ltr"><code><span class="line"><span style="color:#6F42C1;--shiki-dark:#B392F0">agy</span><span style="color:#032F62;--shiki-dark:#9ECBFF"> remote-control</span><span style="color:#032F62;--shiki-dark:#9ECBFF"> start</span></span></code></pre><p>The daemon registers as an OS service (a systemd user service on Linux, a LaunchAgent on macOS, or a Scheduled Task on Windows) and starts immediately without requiring an open terminal window.</p><p>Run <code dir="auto">agy remote-control status</code> to view this machine’s instance name, then select it from the <a href="https://antigravity.google.com">Antigravity Remote Control Dashboard</a> to interact with it using the Antigravity desktop UI.</p><div class="sl-heading-wrapper level-h4"><h4 id="managing-the-daemon">Managing the daemon</h4><a class="sl-anchor-link" href="#managing-the-daemon"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore="">Section titled “Managing the daemon”</span></a></div><p>Use the following commands to manage the headless daemon:</p><pre class="astro-code astro-code-themes github-light github-dark" style="background-color:#fff;--shiki-dark-bg:#24292e;color:#24292e;--shiki-dark:#e1e4e8; overflow-x: auto; white-space: pre-wrap; word-wrap: break-word;" tabindex="0" data-language="text" dir="ltr"><code><span class="line"><span>agy remote-control start     Register and start the daemon</span></span>
<span class="line"><span>agy remote-control status    Show the daemon's status and instance name</span></span>
<span class="line"><span>agy remote-control stop      Stop the daemon and unregister it</span></span></code></pre><div class="sl-heading-wrapper level-h4"><h4 id="windows-notes">Windows notes</h4><a class="sl-anchor-link" href="#windows-notes"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore="">Section titled “Windows notes”</span></a></div><p>Keep the following behavior in mind when running the daemon on Windows:</p>\<ul\>
\<li\>Registering the daemon to <strong>start at boot</strong> requires an Administrator prompt. PowerShell and Command Prompt both work.</li>
\<li\>From a normal prompt, <code dir="auto">start</code> still works, but if your machine gets logged out or restarted, the daemon resumes only after you log back in; it cannot start on boot.</li>
\<li\><code dir="auto">status</code> and <code dir="auto">stop</code> work from a normal prompt.</li>
</ul><div class="sl-heading-wrapper level-h4"><h4 id="sign-in">Sign-in</h4><a class="sl-anchor-link" href="#sign-in"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore="">Section titled “Sign-in”</span></a></div><p>The daemon uses the credentials you used to sign in to the CLI for authentication. This sign-in is separate from the Antigravity editor on the same machine.</p><div class="sl-heading-wrapper level-h4"><h4 id="naming-your-machine-headless-daemon">Naming your machine (headless daemon)</h4><a class="sl-anchor-link" href="#naming-your-machine-headless-daemon"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore="">Section titled “Naming your machine (headless daemon)”</span></a></div><p>For headless daemon instances, the instance name is how the machine appears in Remote Control:</p>\<ul\>
\<li\>
<p>Set it with the <code dir="auto">--name</code> flag:</p>
<pre class="astro-code astro-code-themes github-light github-dark" style="background-color:#fff;--shiki-dark-bg:#24292e;color:#24292e;--shiki-dark:#e1e4e8; overflow-x: auto; white-space: pre-wrap; word-wrap: break-word;" tabindex="0" data-language="bash" dir="ltr"><code><span class="line"><span style="color:#6F42C1;--shiki-dark:#B392F0">agy</span><span style="color:#032F62;--shiki-dark:#9ECBFF"> remote-control</span><span style="color:#032F62;--shiki-dark:#9ECBFF"> start</span><span style="color:#005CC5;--shiki-dark:#79B8FF"> --name</span><span style="color:#032F62;--shiki-dark:#9ECBFF"> "my-workstation"</span></span></code></pre>
<p>Re-running <code dir="auto">start --name</code> with a new value renames the machine.</p>
</li>
\<li\>
<p>If you never set a name, the daemon generates a friendly one (such as <code dir="auto">my-machine-distant-plume</code>) on its first start. <code dir="auto">agy remote-control status</code> shows the current name.</p>
</li>
\<li\>
<p>To rename by editing the settings file instead, open <code dir="auto">~/.gemini/config/config.json</code> on Linux or macOS or <code dir="auto">%USERPROFILE%\.gemini\config\config.json</code> on Windows, change the value of <code dir="auto">cliRemoteControlHostname</code>, save, and run <code dir="auto">agy remote-control start</code> again to restart the daemon. The new name appears in the Antigravity Remote Control dashboard once the service restarts; edits do nothing while it’s running.</p>
</li>
</ul><aside aria-label="Note" class="starlight-aside starlight-aside--note"><p class="starlight-aside__title" aria-hidden="true"><svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor" class="starlight-aside__icon"><path d="M12 11a1 1 0 0 0-1 1v4a1 1 0 0 0 2 0v-4a1 1 0 0 0-1-1Zm.38-3.92a1 1 0 0 0-.76 0 1 1 0 0 0-.33.21 1.15 1.15 0 0 0-.21.33 1 1 0 0 0 .21 1.09c.097.088.209.16.33.21A1 1 0 0 0 13 8a1.05 1.05 0 0 0-.29-.71 1 1 0 0 0-.33-.21ZM12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20Zm0 18a8 8 0 1 1 0-16.001A8 8 0 0 1 12 20Z"></path></svg>Note</p><div class="starlight-aside__content"><p><strong>Important:</strong> The file has two similar-looking names in it. <code dir="auto">cliRemoteControlHostname</code> is this service—the one you want. <code dir="auto">remoteControlHostname</code> is the Antigravity editor on the same machine; in the Antigravity App, you can edit this directly in <strong>Settings</strong> > <strong>App</strong>.</p></div></aside><div class="sl-heading-wrapper level-h4"><h4 id="how-the-daemon-runs">How the daemon runs</h4><a class="sl-anchor-link" href="#how-the-daemon-runs"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore="">Section titled “How the daemon runs”</span></a></div>




























<table><thead><tr><th align="left">Behavior</th><th align="left">Linux</th><th align="left">macOS</th><th align="left">Windows</th></tr></thead>\<tbody\><tr><td align="left">Starts</td><td align="left">At boot, nobody needs to log in</td><td align="left">When you log in</td><td align="left">At boot (Administrator install); at login otherwise</td></tr><tr><td align="left">Keeps running after you sign out</td><td align="left">Yes</td><td align="left">No, back at next login.</td><td align="left">Yes, with a boot install</td></tr><tr><td align="left">Comes back by itself after a crash</td><td align="left">Yes</td><td align="left">Yes</td><td align="left">Yes, up to three restart attempts, then at the next boot or by running <code dir="auto">agy remote-control start</code> again</td></tr></tbody></table>\<hr\><div class="sl-heading-wrapper level-h3"><h3 id="interactive-mode-vs-headless-background-daemon">Interactive mode vs. headless background daemon</h3><a class="sl-anchor-link" href="#interactive-mode-vs-headless-background-daemon"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore="">Section titled “Interactive mode vs. headless background daemon”</span></a></div><p>Use the table below to choose between interactive Remote Control and the persistent background daemon:</p>







































<table><thead><tr><th align="left">Feature</th><th align="left">Interactive mode (<code dir="auto">/remote-control</code> / <code dir="auto">agy --remote-control</code>)</th><th align="left">Headless background daemon (<code dir="auto">agy remote-control start</code>)</th></tr></thead>\<tbody\><tr><td align="left"><strong>Scope</strong></td><td align="left">Single interactive terminal session</td><td align="left">Entire machine (background OS service)</td></tr><tr><td align="left"><strong>Lifecycle</strong></td><td align="left">Active only while the terminal session is open and toggled on</td><td align="left">Persistent across logouts and system reboots</td></tr><tr><td align="left"><strong>Instance naming</strong></td><td align="left">Fresh per-session name (for example, <code dir="auto">host-swift-falcon</code>)</td><td align="left">Single persistent machine name (<code dir="auto">--name</code> or <code dir="auto">cliRemoteControlHostname</code>)</td></tr><tr><td align="left"><strong>Concurrency</strong></td><td align="left">Multiple concurrent terminal sessions supported per machine</td><td align="left">One daemon service per machine</td></tr><tr><td align="left"><strong>Direct handoff link</strong></td><td align="left">Prints a link directly into the active conversation thread</td><td align="left">Accessed using the instance switcher on <code dir="auto">antigravity.google.com</code></td></tr><tr><td align="left"><strong>Best for</strong></td><td align="left">Stepping away from an active terminal task and continuing on mobile or web</td><td align="left">Always-on remote access to a workstation or server anytime</td></tr></tbody></table>\<hr\><div class="sl-heading-wrapper level-h3"><h3 id="cli-troubleshooting">CLI troubleshooting</h3><a class="sl-anchor-link" href="#cli-troubleshooting"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore="">Section titled “CLI troubleshooting”</span></a></div><div class="sl-heading-wrapper level-h4"><h4 id="interactive-mode-issues">Interactive mode issues</h4><a class="sl-anchor-link" href="#interactive-mode-issues"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore="">Section titled “Interactive mode issues”</span></a></div><p>Review the following solutions for common interactive mode issues:</p>\<ul\>
\<li\><strong><code dir="auto">Remote control is not enabled for your account.</code></strong>: Account or organization policy has not enabled the Remote Control feature flag. Contact your workspace administrator or verify that your account plan supports Remote Control.</li>
\<li\><strong>Instance disappears from the web dashboard</strong>: <code dir="auto">/remote-control</code> and <code dir="auto">agy --remote-control</code> create tunnels that automatically disconnect when the interactive terminal session exits (<code dir="auto">/exit</code> or <code dir="auto">Ctrl+C</code>), or when <code dir="auto">/remote-control off</code> is run. To keep a machine reachable when no terminal is open, run <code dir="auto">agy remote-control start</code> to install the headless daemon.</li>
\<li\><strong>Multiple CLI instances listed for the same machine</strong>: Each interactive CLI session uses a distinct session hostname so you can control multiple terminal windows simultaneously.</li>
</ul><div class="sl-heading-wrapper level-h4"><h4 id="headless-daemon-issues">Headless daemon issues</h4><a class="sl-anchor-link" href="#headless-daemon-issues"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore="">Section titled “Headless daemon issues”</span></a></div><p>Review the following solutions for common headless daemon issues:</p>\<ul\>
\<li\><strong>Machine doesn’t show up in the Antigravity Remote Control dashboard</strong>: Run <code dir="auto">agy remote-control status</code>. If it reports sign-in problems, run <code dir="auto">agy</code>, sign in, and then run <code dir="auto">agy remote-control start</code> again.</li>
\<li\><strong>Checking the logs</strong>: Inspect the daemon logs for your operating system:
\<ul\>
\<li\><strong>Linux</strong>: <code dir="auto">journalctl --user -u antigravity-cli-daemon -n 50</code></li>
\<li\><strong>macOS</strong>: <code dir="auto">~/Library/Logs/antigravity-cli-daemon.log</code></li>
\<li\><strong>Windows</strong>: The newest <code dir="auto">cli-*.log</code> under <code dir="auto">%USERPROFILE%\.gemini\antigravity-cli\log</code></li>
</ul>
</li>
\<li\><strong>Rename didn’t take effect</strong>: The name is read when the daemon starts; run <code dir="auto">agy remote-control start</code> again.</li>
\<li\><strong>Windows daemon only starts at login, not at boot</strong>: <code dir="auto">start</code> was run from a non-Administrator prompt. Re-run <code dir="auto">agy remote-control start</code> from an elevated prompt for start-at-boot.</li>
\<li\><strong>Connection keeps dropping and reconnecting</strong>: An old script-installed daemon may still be registered and competing with this one for the connection. Run <code dir="auto">agy remote-control start</code> again; it removes the old service automatically.</li>
\<li\><strong>Two similar entries in the Antigravity Remote Control dashboard</strong>: One is the editor, and one is this daemon. They’re separate on purpose; rename whichever one you want to distinguish.</li>
</ul></div><starlight-tabs-restore class="astro-hbt4ddzs"></starlight-tabs-restore></starlight-tabs><script type="module">var e=class e extends HTMLElement{static#e=new Map;tabs;panels;#t;#n=`starlight-synced-tabs__`;constructor(){super();let t=this.querySelector(`[role="tablist"]`);if(this.tabs=[...t.querySelectorAll(`[role="tab"]`)],this.panels=[...this.querySelectorAll(`:scope > [role="tabpanel"]`)],this.#t=this.dataset.syncKey,this.#t){let t=e.#e.get(this.#t)??[];t.push(this),e.#e.set(this.#t,t)}this.tabs.forEach((e,n)=>{e.addEventListener(`click`,e=>{e.preventDefault();let r=t.querySelector(`[aria-selected="true"]`);e.currentTarget!==r&&this.switchTab(e.currentTarget,n)}),e.addEventListener(`keydown`,e=>{let t=this.tabs.indexOf(e.currentTarget),n=e.key===`ArrowLeft`?t-1:e.key===`ArrowRight`?t+1:e.key===`Home`?0:e.key===`End`?this.tabs.length-1:null;n!==null&&this.tabs[n]&&(e.preventDefault(),this.switchTab(this.tabs[n],n))})})}switchTab(t,n,r=!0){if(!t)return;let i=r?this.getBoundingClientRect().top:0;this.tabs.forEach(e=>{e.setAttribute(`aria-selected`,`false`),e.setAttribute(`tabindex`,`-1`)}),this.panels.forEach(e=>{e.hidden=!0});let a=this.panels[n];a&&(a.hidden=!1),t.removeAttribute(`tabindex`),t.setAttribute(`aria-selected`,`true`),r&&(t.focus(),e.#i(this,t),window.scrollTo({top:window.scrollY+(this.getBoundingClientRect().top-i),behavior:`instant`}))}#r(e){!this.#t||typeof localStorage>`u`||localStorage.setItem(this.#n+this.#t,e)}static#i(t,n){let r=t.#t,i=e.#a(n);if(!r||!i)return;let a=e.#e.get(r);if(a){for(let n of a){if(n===t)continue;let r=n.tabs.findIndex(t=>e.#a(t)===i);r!==-1&&n.switchTab(n.tabs[r],r,!1)}t.#r(i)}}static#a(e){return e.textContent?.trim()}};customElements.define(`starlight-tabs`,e);</script>
\<hr\>
<div class="sl-heading-wrapper level-h2"><h2 id="next-steps">Next steps</h2><a class="sl-anchor-link" href="#next-steps"><span aria-hidden="true" class="sl-anchor-icon"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="m12.11 15.39-3.88 3.88a2.52 2.52 0 0 1-3.5 0 2.47 2.47 0 0 1 0-3.5l3.88-3.88a1 1 0 1 0-1.42-1.42l-3.88 3.89a4.48 4.48 0 0 0 6.33 6.33l3.89-3.88a1 1 0 0 0-1.42-1.42m8.58-12.08a4.49 4.49 0 0 0-6.33 0l-3.89 3.88a1 1 0 1 0 1.42 1.42l3.88-3.88a2.52 2.52 0 0 1 3.5 0 2.47 2.47 0 0 1 0 3.5l-3.88 3.88a1 1 0 0 0 0 1.42 1 1 0 0 0 1.42 0l3.88-3.89a4.49 4.49 0 0 0 0-6.33M8.83 15.17a1 1 0 0 0 .71.29 1 1 0 0 0 .71-.29l4.92-4.92a1 1 0 1 0-1.42-1.42l-4.92 4.92a1 1 0 0 0 0 1.42"></path></svg></span><span class="sr-only" data-pagefind-ignore>Section titled “Next steps”</span></a></div>
<p>Explore related documentation and guides:</p>
\<ul\>
\<li\><a href="/docs/settings">Settings overview</a>: Explore configuration options across Antigravity.</li>
\<li\><a href="/docs/permissions">Permissions and security</a>: Configure permission presets and tool access rules.</li>
</ul></div><footer class="sl-flex docs-footer docs-footer-wrapper astro-4mgpquaz" data-pagefind-ignore><div class="meta sl-flex astro-4mgpquaz"></div><div class="pagination-links print:hidden astro-jgq3vlii" dir="ltr"><a href="/docs/features/" rel="prev" class="astro-jgq3vlii"><svg aria-hidden="true" class="astro-jgq3vlii astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.5rem;"><path d="M17 11H9.41l3.3-3.29a1.004 1.004 0 1 0-1.42-1.42l-5 5a1 1 0 0 0-.21.33 1 1 0 0 0 0 .76 1 1 0 0 0 .21.33l5 5a1.002 1.002 0 0 0 1.639-.325 1 1 0 0 0-.219-1.095L9.41 13H17a1 1 0 0 0 0-2Z"/></svg><span class="astro-jgq3vlii">Previous<br class="astro-jgq3vlii"><span class="link-title astro-jgq3vlii">Overview</span></span></a><a href="/docs/marketplace/" rel="next" class="astro-jgq3vlii"><svg aria-hidden="true" class="astro-jgq3vlii astro-n5k4ipy7" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="--sl-icon-size: 1.5rem;"><path d="M17.92 11.62a1.001 1.001 0 0 0-.21-.33l-5-5a1.003 1.003 0 1 0-1.42 1.42l3.3 3.29H7a1 1 0 0 0 0 2h7.59l-3.3 3.29a1.002 1.002 0 0 0 .325 1.639 1 1 0 0 0 1.095-.219l5-5a1 1 0 0 0 .21-.33 1 1 0 0 0 0-.76Z"/></svg><span class="astro-jgq3vlii">Next<br class="astro-jgq3vlii"><span class="link-title astro-jgq3vlii">Marketplace</span></span></a></div><nav class="docs-legal-nav print:hidden astro-4mgpquaz" data-analytics-nav="docs_footer" aria-label="Google legal and cookie links"><a class="docs-footer-link astro-4mgpquaz" href="https://about.google/" target="_blank" rel="noopener noreferrer">About Google</a><a class="docs-footer-link astro-4mgpquaz" href="https://about.google/products/" target="_blank" rel="noopener noreferrer">Google Products</a><a class="docs-footer-link astro-4mgpquaz" href="https://policies.google.com/privacy?hl=en-US" target="_blank" rel="noopener noreferrer">Privacy</a><a class="docs-footer-link astro-4mgpquaz" href="/terms">Terms</a><a aria-hidden="true" class="docs-footer-link glue-cookie-notification-bar-control astro-4mgpquaz">Manage cookies</a></nav><template data-download-survey-template><dialog class="dls" data-download-survey aria-labelledby="dls-title" aria-describedby="dls-subtitle" data-nosnippet><div class="dls-card"><button type="button" class="dls-close" data-dls-dismiss="close" aria-label="Close"><svg viewBox="0 0 24 24" aria-hidden="true" focusable="false"><path d="M6.5 6.5l11 11M17.5 6.5l-11 11"></path></svg></button><div class="dls-body"><div class="dls-ask" data-dls-view="ask"><p class="dls-status" data-dls-status><svg class="dls-status-icon" viewBox="0 0 20 20" aria-hidden="true" focusable="false"><circle cx="10" cy="10" r="10"></circle><path d="M6 10.4l2.6 2.6L14 7.6"></path></svg><span data-download-survey-text>Your download has started</span></p><h2 class="dls-title" id="dls-title" data-download-survey-text>How did you hear about us?</h2><p class="dls-subtitle" id="dls-subtitle" data-download-survey-text>Pick one. It helps us reach more people like you.</p><div class="dls-options" role="group" aria-labelledby="dls-title" data-dls-options><button type="button" class="dls-option" data-dls-answer="search"><svg class="dls-option-check" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="M3.2 8.4l3 3 6.6-6.8"></path></svg><span data-download-survey-text>Search engine</span></button><button type="button" class="dls-option" data-dls-answer="ai_assistant"><svg class="dls-option-check" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="M3.2 8.4l3 3 6.6-6.8"></path></svg><span data-download-survey-text>AI assistant</span></button><button type="button" class="dls-option" data-dls-answer="youtube"><svg class="dls-option-check" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="M3.2 8.4l3 3 6.6-6.8"></path></svg><span data-download-survey-text>YouTube</span></button><button type="button" class="dls-option" data-dls-answer="x"><svg class="dls-option-check" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="M3.2 8.4l3 3 6.6-6.8"></path></svg><span data-download-survey-text>X (Twitter)</span></button><button type="button" class="dls-option" data-dls-answer="linkedin"><svg class="dls-option-check" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="M3.2 8.4l3 3 6.6-6.8"></path></svg><span data-download-survey-text>LinkedIn</span></button><button type="button" class="dls-option" data-dls-answer="reddit"><svg class="dls-option-check" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="M3.2 8.4l3 3 6.6-6.8"></path></svg><span data-download-survey-text>Reddit</span></button><button type="button" class="dls-option" data-dls-answer="hacker_news"><svg class="dls-option-check" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="M3.2 8.4l3 3 6.6-6.8"></path></svg><span data-download-survey-text>Hacker News</span></button><button type="button" class="dls-option" data-dls-answer="friend_or_colleague"><svg class="dls-option-check" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="M3.2 8.4l3 3 6.6-6.8"></path></svg><span data-download-survey-text>Friend or colleague</span></button><button type="button" class="dls-option" data-dls-answer="news_or_blog"><svg class="dls-option-check" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="M3.2 8.4l3 3 6.6-6.8"></path></svg><span data-download-survey-text>News or blog</span></button><button type="button" class="dls-option" data-dls-answer="event"><svg class="dls-option-check" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="M3.2 8.4l3 3 6.6-6.8"></path></svg><span data-download-survey-text>Event or conference</span></button><button type="button" class="dls-option" data-dls-answer="google_product"><svg class="dls-option-check" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="M3.2 8.4l3 3 6.6-6.8"></path></svg><span data-download-survey-text>Another Google product</span></button><button type="button" class="dls-option" data-dls-answer="other"><svg class="dls-option-check" viewBox="0 0 16 16" aria-hidden="true" focusable="false"><path d="M3.2 8.4l3 3 6.6-6.8"></path></svg><span data-download-survey-text>Other</span></button></div><div class="dls-actions"><button type="button" class="dls-skip" data-dls-dismiss="skip"><span data-download-survey-text>Skip</span></button></div></div><div class="dls-thanks" data-dls-view="thanks" role="status" aria-live="polite" aria-labelledby="dls-thanks-title" tabindex="-1" hidden><div class="dls-thanks-content" data-dls-thanks-content><svg class="dls-thanks-mark" viewBox="0 0 52 52" aria-hidden="true" focusable="false"><circle cx="26" cy="26" r="24"></circle><path d="M16 27l7 7 14-15"></path></svg><p class="dls-thanks-title" id="dls-thanks-title" data-download-survey-text>Thanks for letting us know!</p></div></div></div></div></dialog></template><script type="module" src="/_astro/DownloadSurvey.astro_astro_type_script_index_0_lang.DWzR8tN5.js"></script></footer></div></div></main></div></div></div></div></body></html>