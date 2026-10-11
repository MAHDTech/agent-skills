<!DOCTYPE html><html lang="en" data-palette="catppuccin" data-mode="ink"> \<head\><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1">\<title\>Herdr plugins: community plugin marketplace</title><meta name="description" content="Discover community Herdr plugins published on GitHub. An automatic index of public repositories tagged with the herdr-plugin topic."><meta name="theme-color" content="#17171a"><link rel="canonical" href="https://herdr.dev/plugins/"><meta property="og:type" content="website"><meta property="og:site_name" content="Herdr"><meta property="og:title" content="Herdr plugins: community plugin marketplace"><meta property="og:description" content="Discover community Herdr plugins published on GitHub. An automatic index of public repositories tagged with the herdr-plugin topic."><meta property="og:url" content="https://herdr.dev/plugins/"><meta property="og:image" content="https://herdr.dev/assets/og-card-v9.png"><meta property="og:image:width" content="1200"><meta property="og:image:height" content="630"><meta property="og:image:alt" content="Herdr — run them anywhere, leave them running."><meta name="twitter:card" content="summary_large_image"><meta name="twitter:title" content="Herdr plugins: community plugin marketplace"><meta name="twitter:description" content="Discover community Herdr plugins published on GitHub. An automatic index of public repositories tagged with the herdr-plugin topic."><meta name="twitter:image" content="https://herdr.dev/assets/og-card-v9.png"><meta name="twitter:image:alt" content="Herdr — run them anywhere, leave them running."><link rel="icon" href="/favicon.ico" sizes="48x48"><link rel="icon" type="image/png" sizes="192x192" href="/assets/favicon.png"><link rel="apple-touch-icon" href="/assets/logo.png"><link rel="preload" href="/assets/jbm.woff2?v=2" as="font" type="font/woff2" crossorigin><link rel="preconnect" href="https://fonts.googleapis.com"><link rel="preconnect" href="https://fonts.gstatic.com" crossorigin><link href="https://fonts.googleapis.com/css2?family=Archivo:wght@600;800;900&family=Inter:wght@400;500;600&display=swap" rel="stylesheet">\<script\>
      /* Ground selection, shared with the docs — see index.html for the
         reasoning. Starlight's `starlight-theme` key is the source of truth;
         `auto` is treated as dark so the default never follows the OS. */
      window.herdrGround = {
        read: function () {
          try {
            return localStorage.getItem('starlight-theme') === 'light' ? 'paper' : 'ink';
          } catch (e) {
            return 'ink';
          }
        },
        apply: function (mode) {
          var root = document.documentElement;
          root.setAttribute('data-mode', mode);
          root.setAttribute('data-palette', mode === 'paper' ? 'solarized-light' : 'catppuccin');
          return mode;
        },
        save: function (mode) {
          try {
            localStorage.setItem('starlight-theme', mode === 'paper' ? 'light' : 'dark');
          } catch (e) {}
        },
      };
      window.herdrGround.apply(window.herdrGround.read());
    </script><link rel="stylesheet" href="/css/style.css?v=59"><link rel="stylesheet" href="/css/site.css?v=64"><link rel="stylesheet" href="/_astro/plugins.lNVOORFy.css"><script type="module" src="/_astro/page.B1D-nYk3.js"></script></head> <body class="site"> <div class="hd-frame"> <nav class="hd-nav"> <a class="lock" href="/"><span class="ram"></span><b>herdr</b></a> <div class="sp"></div> <a class="lnk" href="/docs/">Docs</a> <a class="lnk" href="/plugins/">Plugins</a> <a class="lnk" href="/blog/">Blog</a> <a class="lnk" href="/compare/">Compare</a> <div class="social-links"> <a class="ico wide" href="https://github.com/herdrdev/herdr" aria-label="Herdr on GitHub — star count"> <svg viewBox="0 0 16 16" aria-hidden="true"><path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82A7.64 7.64 0 0 1 8 3.87c.68 0 1.36.09 2 .26 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.01 8.01 0 0 0 16 8c0-4.42-3.58-8-8-8Z"></path></svg> <b data-stars>42,441</b> </a> <a class="ico" href="https://x.com/herdrdev" aria-label="Herdr on X"> <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231Zm-1.161 17.52h1.833L7.084 4.126H5.117Z"></path></svg> </a> <a class="ico" href="https://discord.gg/PsgRfzmUDg" aria-label="Herdr on Discord"> <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M19.73 4.87a18.2 18.2 0 0 0-4.51-1.4l-.56 1.14a16.8 16.8 0 0 0-5.32 0l-.56-1.14a18.2 18.2 0 0 0-4.51 1.4C1.42 9.09.65 13.2 1.03 17.25a18.3 18.3 0 0 0 5.53 2.79c.45-.61.85-1.26 1.19-1.94-.65-.24-1.28-.54-1.88-.9l.46-.36a13.1 13.1 0 0 0 11.34 0l.46.36c-.6.36-1.23.66-1.88.9.34.68.74 1.33 1.19 1.94a18.3 18.3 0 0 0 5.53-2.79c.45-4.69-.77-8.76-3.24-12.38ZM8.35 14.77c-1.1 0-2-1.01-2-2.25s.88-2.25 2-2.25 2.02 1.02 2 2.25c0 1.24-.88 2.25-2 2.25Zm7.3 0c-1.1 0-2-1.01-2-2.25s.88-2.25 2-2.25 2.02 1.02 2 2.25c0 1.24-.88 2.25-2 2.25Z"></path></svg> </a> </div> <div class="mode"> <button type="button" data-ground="ink" aria-pressed="true">ink</button> <button type="button" data-ground="paper" aria-pressed="false">paper</button> </div> <a class="btn" href="/docs/install/">Install</a> <button type="button" class="menu" id="nav-menu" aria-expanded="false" aria-controls="nav-sheet" aria-label="Menu"> <span></span><span></span><span></span> </button> </nav> <div class="sheet" id="nav-sheet"> <a href="/docs/">Docs</a> <a href="/plugins/">Plugins</a> <a href="/blog/">Blog</a> <a href="/compare/">Compare</a> <a href="https://x.com/herdrdev">@herdrdev</a> <a href="https://discord.gg/PsgRfzmUDg">Discord</a> <div class="sheet-ground"> <span>Ground</span> <div class="mode"> <button type="button" data-ground="ink" aria-pressed="true">ink</button> <button type="button" data-ground="paper" aria-pressed="false">paper</button> </div> </div> </div>  <main class="plugins-page"> <header class="ph ph-split"> <div> <div class="hd-eyebrow">\<s\></s>the marketplace</div> \<h1\>Built by the herd.</h1> <p>
Community plugins published on GitHub, discovered automatically from the
<code>herdr-plugin</code> topic. Install any of them with
<code>herdr plugin install</code>, or publish your own and get listed.
</p> <p class="plugins-trust"> <span class="status-dot dot-idle"></span>
Listings aren't reviewed by Herdr, so install at your own discretion.
<a href="/docs/plugins/#trust-and-security">Trust guidance →</a> </p> </div> <aside class="ph-side"> <b id="plugins-hero-count">—</b> <span id="plugins-hero-summary">Loading plugins…</span> <a href="/docs/plugins/">Publish yours →</a> </aside> </header> <div id="plugins-shelves" hidden> <section class="section plugins-shelf" id="shelf-trending" hidden> <div class="shelf-head"> <div> <h2 class="shelf-title">Trending this week</h2> <p class="shelf-sub">fastest-growing repositories, relative to their own size</p> </div> <button class="shelf-all" type="button" data-sort="trending">see all →</button> </div> <div id="trend-grid" class="trend-grid" role="list"></div> </section> <section class="section plugins-shelf" id="shelf-new" hidden> <div class="shelf-head"> <div> <h2 class="shelf-title">New to the herd</h2> <p class="shelf-sub">fresh projects that joined recently</p> </div> <button class="shelf-all" type="button" data-sort="newest">see all →</button> </div> <div class="new-rows" role="list"></div> </section> </div> <section class="section plugins-results"> <div class="shelf-head plugins-browse-head" id="plugins-browse-head"> <div> <h2 class="shelf-title">Browse everything</h2> <p class="shelf-sub">the full index — search or pick a sort</p> </div> </div> <div class="plugins-controls"> <div class="plugins-search-wrap"> <span class="plugins-search-icon" aria-hidden="true">/</span> <input id="plugins-search" type="search" class="plugins-search" placeholder="search plugins, repositories, owners, languages…" autocomplete="off" spellcheck="false" aria-label="Search plugins"> </div> <label class="plugins-sort-wrap"> <span class="plugins-sort-label">sort</span> <select id="plugins-sort" class="plugins-sort" aria-label="Sort plugins"> <option value="popular">Popular</option> <option value="trending">Trending</option> <option value="recent">Recently updated</option> <option value="newest">Newest</option> <option value="name">Name</option> </select> </label> </div> <p id="plugins-meta" class="plugins-meta" aria-live="polite"></p> <div id="plugins-grid" class="plugins-grid" role="list"></div> <div id="plugins-status" class="plugins-status">Loading plugins…</div> <button id="plugins-more" class="button button-secondary plugins-more" hidden></button> </section> <section class="section plugins-publish"> <div class="plugins-publish-card"> <p class="section-kicker"># publish</p> \<h3\>Want your plugin listed?</h3> <p>
Add the GitHub topic <code>herdr-plugin</code> to a public repository
          with a <code>herdr-plugin.toml</code> manifest. The index picks it up
          on the next refresh. No submission, no review queue.
</p> <div class="plugins-publish-actions"> <a class="button button-primary" href="/docs/marketplace/">How publishing works</a> <a class="button button-secondary" href="/docs/plugins/">Author a plugin</a> </div> </div> </section> </main> \<script\>
    const grid = document.getElementById("plugins-grid");
    const statusEl = document.getElementById("plugins-status");
    const searchInput = document.getElementById("plugins-search");
    const sortSelect = document.getElementById("plugins-sort");
    const metaEl = document.getElementById("plugins-meta");
    const moreButton = document.getElementById("plugins-more");
    const heroCountEl = document.getElementById("plugins-hero-count");
    const heroSummaryEl = document.getElementById("plugins-hero-summary");
    const shelvesWrap = document.getElementById("plugins-shelves");
    const trendingShelf = document.getElementById("shelf-trending");
    const trendGrid = document.getElementById("trend-grid");
    const newShelf = document.getElementById("shelf-new");

    // 24 fills complete rows in both the 3-column and 2-column grid layouts.
    const PAGE_SIZE = 24;
    let visibleLimit = PAGE_SIZE;

    // A shelf earns its place: percent growth only counts once a repository
    // gained real stars this week, so the long tail (median: 1 star) can't
    // flood trending with 1-to-3-star noise, while big repositories must grow
    // at their own scale to appear.
    const TRENDING_MIN_GAIN = 10;
    // "New" means new to everyone: recently indexed AND a recently created
    // repository. Established projects that just adopted the topic surface
    // through trending instead of burying actual newcomers.
    const NEW_SEEN_MAX_DAYS = 14;
    const NEW_CREATED_MAX_DAYS = 60;
    // Six cards fill complete shelf rows in the 3, 2, and 1 column layouts.
    const SHELF_SIZE = 6;
    // A shelf with one or two cards reads as a struggling marketplace, which
    // is worse than no shelf at all.
    const SHELF_MIN_CARDS = 3;

    const LANG_COLORS = {
      TypeScript: "#3178c6",
      JavaScript: "#f1e05a",
      Rust: "#dea584",
      Python: "#3572A5",
      Go: "#00ADD8",
      Lua: "#000080",
      Shell: "#89e051",
      "C++": "#f34b7d",
      Ruby: "#701516",
    };

    // One entry per repository. Its `manifests` are the plugins it ships.
    let repos = [];

    function timeAgo(iso) {
      if (!iso) return null;
      const then = new Date(iso).getTime();
      if (Number.isNaN(then)) return null;
      const secs = Math.max(0, Math.round((Date.now() - then) / 1000));
      const units = [
        ["y", 31536000],
        ["mo", 2592000],
        ["w", 604800],
        ["d", 86400],
        ["h", 3600],
        ["m", 60],
      ];
      for (const [label, size] of units) {
        const v = Math.floor(secs / size);
        if (v >= 1) return v + label + " ago";
      }
      return "just now";
    }

    function isGithubUrl(url) {
      return typeof url === "string" && /^https:\/\/github\.com\/[^/]+\/[^/]+/.test(url);
    }

    function daysSince(iso) {
      if (!iso) return null;
      const then = new Date(iso).getTime();
      if (Number.isNaN(then)) return null;
      return (Date.now() - then) / 86400000;
    }

    function trendingScore(p) {
      const delta = p.starsDelta7d;
      if (typeof delta !== "number" || delta < TRENDING_MIN_GAIN) return null;
      return delta / Math.max((p.stars ?? 0) - delta, 1);
    }

    function isNewToHerd(p) {
      const seen = daysSince(p.firstSeenAt);
      const created = daysSince(p.createdAt);
      return (
        seen !== null && seen <= NEW_SEEN_MAX_DAYS &&
        created !== null && created <= NEW_CREATED_MAX_DAYS
      );
    }

    function trendingBadge(p) {
      const delta = p.starsDelta7d ?? 0;
      const base = Math.max((p.stars ?? 0) - delta, 0);
      return "▲ " + base + " → " + (p.stars ?? 0) + " ★ this week";
    }

    function joinedBadge(p) {
      const ago = timeAgo(p.firstSeenAt);
      return ago === "just now" ? "✦ just joined" : ago ? "✦ joined " + ago : null;
    }

    function avatar(p, size) {
      const img = el("img", "plugin-avatar");
      img.src = "https://github.com/" + encodeURIComponent(p.owner || "") + ".png?size=" + size;
      img.alt = "";
      img.loading = "lazy";
      img.width = size / 2;
      img.height = size / 2;
      return img;
    }

    function cardShell(p, className) {
      const url = isGithubUrl(p.url) ? p.url : null;
      const card = el(url ? "a" : "div", className);
      card.setAttribute("role", "listitem");
      if (url) {
        card.href = url;
        card.target = "_blank";
        card.rel = "noopener noreferrer";
      }
      return card;
    }

    function titleOf(p) {
      const title = el("div", "plugin-title");
      title.appendChild(el("span", "plugin-owner", (p.owner || "?") + "/"));
      title.appendChild(el("span", "plugin-name", p.name || "unknown"));
      return title;
    }

    function buildTrendCard(p) {
      const card = cardShell(p, "plugin-card plugin-trend-card");
      const head = el("div", "plugin-card-head");
      const lead = el("div", "trend-lead");
      lead.appendChild(avatar(p, 64));
      lead.appendChild(titleOf(p));
      head.appendChild(lead);
      head.appendChild(el("span", "plugin-go", "↗"));
      card.appendChild(head);
      card.appendChild(el("div", "plugin-badge", trendingBadge(p)));
      const desc = el(
        "p",
        "plugin-desc" + (p.description ? "" : " is-empty"),
        p.description || "No description provided.",
      );
      card.appendChild(desc);
      return card;
    }

    function buildRow(p, note) {
      const row = cardShell(p, "plugin-row");
      row.appendChild(avatar(p, 64));
      const body = el("div", "plugin-row-body");
      body.appendChild(titleOf(p));
      const desc = el(
        "span",
        "plugin-row-desc" + (p.description ? "" : " is-empty"),
        p.description || "No description provided.",
      );
      body.appendChild(desc);
      row.appendChild(body);
      const side = el("div", "plugin-row-side");
      if (note) side.appendChild(el("span", "plugin-row-note", note));
      const stars = el("span", "plugin-stat");
      stars.appendChild(el("span", "plugin-star", "★"));
      stars.appendChild(el("span", null, String(p.stars ?? 0)));
      side.appendChild(stars);
      row.appendChild(side);
      return row;
    }

    function el(tag, className, text) {
      const node = document.createElement(tag);
      if (className) node.className = className;
      if (text != null) node.textContent = text;
      return node;
    }

    function buildCard(p, badge) {
      const url = isGithubUrl(p.url) ? p.url : null;
      const card = el(url ? "a" : "div", "plugin-card");
      card.setAttribute("role", "listitem");
      if (url) {
        card.href = url;
        card.target = "_blank";
        card.rel = "noopener noreferrer";
      }

      const head = el("div", "plugin-card-head");
      const lead = el("div", "trend-lead");
      lead.appendChild(avatar(p, 64));
      lead.appendChild(titleOf(p));
      head.appendChild(lead);
      head.appendChild(el("span", "plugin-go", "↗"));
      card.appendChild(head);
      if (badge) card.appendChild(el("div", "plugin-badge", badge));

      const desc = el(
        "p",
        "plugin-desc" + (p.description ? "" : " is-empty"),
        p.description || "No description provided.",
      );
      card.appendChild(desc);

      const meta = el("div", "plugin-meta-row");

      const stars = el("span", "plugin-stat");
      stars.appendChild(el("span", "plugin-star", "★"));
      stars.appendChild(el("span", null, String(p.stars ?? 0)));
      meta.appendChild(stars);

      if (p.language) {
        const lang = el("span", "plugin-stat");
        const dot = el("span", "plugin-lang-dot");
        dot.style.background = LANG_COLORS[p.language] || "var(--muted)";
        lang.appendChild(dot);
        lang.appendChild(el("span", null, p.language));
        meta.appendChild(lang);
      }

      // Nearly every repository ships exactly one plugin whose name restates the
      // repository name, so only its version earns a place in the meta row.
      const manifests = manifestsOf(p);
      if (manifests.length === 1 && manifests[0].version) {
        meta.appendChild(el("span", "plugin-stat plugin-version", "v" + manifests[0].version));
      }

      const updated = timeAgo(p.pushedAt);
      if (updated) meta.appendChild(el("span", "plugin-stat plugin-updated", "updated " + updated));

      card.appendChild(meta);

      // A monorepo is the case the card title can't answer on its own, so it
      // gets the extra row: how many plugins, and which.
      if (manifests.length > 1) {
        const ships = el("div", "plugin-ships");
        ships.appendChild(el("span", "plugin-ship-count", manifests.length + " plugins"));
        ships.appendChild(
          el("span", "plugin-ship-list", manifests.map(manifestLabel).join(" · ")),
        );
        card.appendChild(ships);
      }

      return card;
    }

    function manifestsOf(p) {
      return Array.isArray(p.manifests) ? p.manifests : [];
    }

    function manifestLabel(m) {
      return m.name || m.id || m.path || "plugin";
    }

    function matches(p, q) {
      if (!q) return true;
      const hay = [
        p.fullName,
        p.owner,
        p.name,
        p.description,
        p.language,
        ...(p.topics || []),
        ...manifestsOf(p).flatMap((m) => [m.name, m.id, m.description]),
      ]
        .filter(Boolean)
        .join(" ")
        .toLowerCase();
      return q.split(/\s+/).every((term) => hay.includes(term));
    }

    function sortRepos(list, mode) {
      const by = [...list];
      const t = (v) => new Date(v || 0).getTime() || 0;
      if (mode === "recent") by.sort((a, b) => t(b.pushedAt) - t(a.pushedAt));
      // "Newest" means newest to the marketplace, matching the shelf's order;
      // GitHub's createdAt is the fallback for snapshots without firstSeenAt.
      else if (mode === "newest")
        by.sort(
          (a, b) => t(b.firstSeenAt ?? b.createdAt) - t(a.firstSeenAt ?? a.createdAt),
        );
      else if (mode === "name") by.sort((a, b) => (a.fullName || "").localeCompare(b.fullName || ""));
      else if (mode === "trending")
        // Qualified movers first by growth rate, then everything else by raw
        // weekly gain so the list stays fully ordered past the shelf criteria.
        by.sort(
          (a, b) =>
            (trendingScore(b) ?? -1) - (trendingScore(a) ?? -1) ||
            (b.starsDelta7d ?? 0) - (a.starsDelta7d ?? 0) ||
            (b.stars ?? 0) - (a.stars ?? 0),
        );
      else
        by.sort(
          (a, b) => (b.stars ?? 0) - (a.stars ?? 0) || t(b.pushedAt) - t(a.pushedAt),
        );
      return by;
    }

    function renderShelves() {
      const trending = repos
        .map((p) => [p, trendingScore(p)])
        .filter(([, score]) => score !== null)
        .sort((a, b) => b[1] - a[1])
        .slice(0, SHELF_SIZE)
        .map(([p]) => p);
      trendGrid.replaceChildren();
      if (trending.length >= SHELF_MIN_CARDS) {
        for (const p of trending) trendGrid.appendChild(buildTrendCard(p));
      }
      trendingShelf.hidden = trending.length < SHELF_MIN_CARDS;

      const fresh = repos
        .filter(isNewToHerd)
        .sort((a, b) => new Date(b.firstSeenAt).getTime() - new Date(a.firstSeenAt).getTime())
        .slice(0, SHELF_SIZE);
      const newRows = newShelf.querySelector(".new-rows");
      newRows.replaceChildren();
      for (const p of fresh) newRows.appendChild(buildRow(p, joinedBadge(p)));
      newShelf.hidden = fresh.length < SHELF_MIN_CARDS;

      const anyShelf = !trendingShelf.hidden || !newShelf.hidden;
      shelvesWrap.dataset.available = anyShelf ? "1" : "";
      updateShelfVisibility();
    }

    function updateShelfVisibility() {
      shelvesWrap.hidden = shelvesWrap.dataset.available !== "1";
    }

    function render() {
      const q = searchInput.value.trim().toLowerCase();
      const visible = sortRepos(repos.filter((p) => matches(p, q)), sortSelect.value);
      const shown = visible.slice(0, visibleLimit);

      grid.replaceChildren();
      for (const p of shown) grid.appendChild(buildCard(p));

      // Grow a page at a time, and name the real remainder on the last step so
      // the button never promises 24 and delivers 5.
      const remaining = visible.length - shown.length;
      moreButton.hidden = remaining === 0;
      if (remaining > 0) {
        moreButton.textContent = "show " + Math.min(PAGE_SIZE, remaining) + " more";
      }

      if (visible.length === 0) {
        statusEl.textContent = q
          ? "No plugins match your search."
          : "No plugins are listed yet.";
        statusEl.hidden = false;
      } else {
        statusEl.hidden = true;
      }
    }

    function setMeta(data) {
      const parts = [];
      const repoCount = (data && data.repositoryCount) ?? repos.length;
      const total =
        (data && data.pluginCount) ??
        repos.reduce((n, repo) => n + (manifestsOf(repo).length || 1), 0);
      parts.push(total + (total === 1 ? " plugin" : " plugins"));
      parts.push(repoCount + (repoCount === 1 ? " repository" : " repositories"));
      heroCountEl.textContent = total.toLocaleString("en-US");
      heroSummaryEl.textContent =
        (total === 1 ? "plugin" : "plugins") +
        " across " +
        repoCount.toLocaleString("en-US") +
        (repoCount === 1 ? " repository" : " repositories");
      const starsGained = repos.reduce(
        (n, repo) => n + Math.max(repo.starsDelta7d ?? 0, 0),
        0,
      );
      if (starsGained > 0) parts.push("+" + starsGained.toLocaleString("en-US") + " ★ this week");
      const joined = repos.filter((repo) => {
        const seen = daysSince(repo.firstSeenAt);
        return seen !== null && seen <= 7;
      }).length;
      if (joined > 0) parts.push(joined + " joined this week");
      const ago = timeAgo(data && data.generatedAt);
      if (ago) parts.push("updated " + ago);
      metaEl.textContent = parts.join(" · ");
    }

    function apply(data) {
      repos = Array.isArray(data && data.plugins) ? data.plugins : [];
      setMeta(data);
      renderShelves();
      render();
    }

    function showError() {
      repos = [];
      metaEl.textContent = "";
      heroSummaryEl.textContent = "Plugin count unavailable";
      grid.replaceChildren();
      moreButton.hidden = true;
      statusEl.textContent =
        "The plugin marketplace is warming up. Check back in a few minutes.";
      statusEl.hidden = false;
    }

    async function load() {
      try {
        const res = await fetch("https://assets.herdr.dev/plugins/index.json", {
          headers: { accept: "application/json" },
        });
        if (!res.ok) throw new Error("status " + res.status);
        apply(await res.json());
      } catch {
        showError();
      }
    }

    // A new query or order is a new list, so paging starts over.
    function resetAndRender() {
      visibleLimit = PAGE_SIZE;
      render();
    }

    searchInput.addEventListener("input", resetAndRender);
    sortSelect.addEventListener("change", resetAndRender);
    moreButton.addEventListener("click", () => {
      visibleLimit += PAGE_SIZE;
      render();
    });
    for (const button of document.querySelectorAll(".shelf-all")) {
      button.addEventListener("click", () => {
        sortSelect.value = button.dataset.sort;
        resetAndRender();
        grid.scrollIntoView({ behavior: "smooth", block: "start" });
      });
    }
    load();
  </script>   <footer class="foot"> <span class="ram"></span> <a href="/docs/">docs</a> <a href="/plugins/">plugins</a> <a href="/blog/">blog</a> <a href="/compare/">compare</a> <span>built in the terminal · © 2026 Herdr, Inc.</span> <span class="fact">v0.9.3 · Apache 2.0</span> <span class="out"> <a class="ico" href="https://github.com/herdrdev/herdr" aria-label="Herdr on GitHub"> <svg viewBox="0 0 16 16" aria-hidden="true"><path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82A7.64 7.64 0 0 1 8 3.87c.68 0 1.36.09 2 .26 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.01 8.01 0 0 0 16 8c0-4.42-3.58-8-8-8Z"></path></svg> </a> <a class="ico" href="https://x.com/herdrdev" aria-label="Herdr on X"> <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231Zm-1.161 17.52h1.833L7.084 4.126H5.117Z"></path></svg> </a> <a class="ico" href="https://discord.gg/PsgRfzmUDg" aria-label="Herdr on Discord"> <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M19.73 4.87a18.2 18.2 0 0 0-4.51-1.4l-.56 1.14a16.8 16.8 0 0 0-5.32 0l-.56-1.14a18.2 18.2 0 0 0-4.51 1.4C1.42 9.09.65 13.2 1.03 17.25a18.3 18.3 0 0 0 5.53 2.79c.45-.61.85-1.26 1.19-1.94-.65-.24-1.28-.54-1.88-.9l.46-.36a13.1 13.1 0 0 0 11.34 0l.46.36c-.6.36-1.23.66-1.88.9.34.68.74 1.33 1.19 1.94a18.3 18.3 0 0 0 5.53-2.79c.45-4.69-.77-8.76-3.24-12.38ZM8.35 14.77c-1.1 0-2-1.01-2-2.25s.88-2.25 2-2.25 2.02 1.02 2 2.25c0 1.24-.88 2.25-2 2.25Zm7.3 0c-1.1 0-2-1.01-2-2.25s.88-2.25 2-2.25 2.02 1.02 2 2.25c0 1.24-.88 2.25-2 2.25Z"></path></svg> </a> <a href="/cdn-cgi/l/email-protection#620a071b220a071006104c060714"><span class="__cf_email__" data-cfemail="87efe2fec7efe2f5e3f5a9e3e2f1">[email&#160;protected]</span></a> </span> </footer> </div> <script data-cfasync="false" src="/cdn-cgi/scripts/5c5dd728/cloudflare-static/email-decode.min.js"></script>\<script\>
      (function () {
        var buttons = document.querySelectorAll('[data-ground]');
        function paint(mode) {
          window.herdrGround.apply(mode);
          buttons.forEach(function (b) {
            b.setAttribute('aria-pressed', String(b.dataset.ground === mode));
          });
        }
        buttons.forEach(function (b) {
          b.addEventListener('click', function () {
            window.herdrGround.save(b.dataset.ground);
            paint(b.dataset.ground);
          });
        });
        paint(window.herdrGround.read());
      })();

      /* nav sheet — the links, the account and the ground toggle that don't
         fit the bar on a phone */
      (function () {
        var btn = document.getElementById('nav-menu');
        var sheet = document.getElementById('nav-sheet');
        if (!btn || !sheet) return;
        function set(open) {
          sheet.classList.toggle('open', open);
          btn.setAttribute('aria-expanded', String(open));
        }
        btn.addEventListener('click', function (event) {
          event.stopPropagation();
          set(!sheet.classList.contains('open'));
        });
        /* the ground buttons live in the sheet, so a tap inside must not close
           it — only taps on the page behind do */
        document.addEventListener('click', function (event) {
          if (!sheet.contains(event.target)) set(false);
        });
        document.addEventListener('keydown', function (event) {
          if (event.key === 'Escape') set(false);
        });
      })();

      /* The nav count is baked at build, which can be a deploy behind. Refresh
         it from the same snapshot so the number is current on every page. No
         count-up here — this is chrome, not the argument the homepage makes. */
      fetch('https://assets.herdr.dev/stats/stats.json', { headers: { accept: 'application/json' } })
        .then(function (r) {
          return r.ok ? r.json() : null;
        })
        .then(function (d) {
          if (!d || !d.stars || typeof d.stars.total !== 'number') return;
          var text = d.stars.total.toLocaleString('en-US');
          document.querySelectorAll('[data-stars]').forEach(function (el) {
            el.textContent = text;
          });
        })
        .catch(function () {});
    </script> <script type="module" src="https://static.cloudflareinsights.com/beacon.min.js/v4bc70e2c01a94c73b74392e4234840661791215815920" integrity="sha512-L0ha0OXavK/8okipN9F8BtP84dg9DUhPERbBXzwI6dgTA55d2+yweo3pn5CSFYs45/r8md2+xvUPtTdvTNRfjA==" data-cf-beacon='{"version":"2024.11.0","token":"8cf9da0ee686416f891e33febc7703c8","r":1,"spa":2}' crossorigin="anonymous"></script>
</body> </html>