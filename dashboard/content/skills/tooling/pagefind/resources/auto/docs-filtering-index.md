+++
title = "docs-filtering-index"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "pagefind"
+++

> **Important: Pagefind 1.5.0 introduces the Component UI, which replaces the
> Default UI (pagefind-ui.js / PagefindUI). It includes a search modal, better
> accessibility and customization.** Full component guide:
> https://pagefind.app/llms-component-ui.txt

# Setting up filters


To configure filters in Pagefind, pages are associated to filter keys and values using data attributes.

## Capturing a filter value from an element

```html
<h1>My Blog Post</h1>
<p>
    Author:
+    <span data-pagefind-filter="author">bglw</span>
</p>
```

An element tagged with `data-pagefind-filter` will associate that page with the filter name, and capture the contents of the element as the filter value. In the above example, the page would be tagged as `author: ["bglw"]`.

Filters can have multiple values per page, so the following is also valid:

```html
<h1>Hello World</h1>
<p>
    Authors:
+    <span data-pagefind-filter="author">Pagefind</span>
    and
+    <span data-pagefind-filter="author">Liam Bigelow</span>
</p>
```

Which produces: `author: ["Pagefind", "Liam Bigelow"]`.

## Capturing a filter value from an attribute

If the data you want to filter on exists as an attribute, you can use the syntax `filter_name[html_attribute]`

```html
<head>
    <meta
+        data-pagefind-filter="author[content]"
        content="Pagefind"
        property="og:site_name">
</head>
```

This will capture the filter value from the attribute specified, in this case producing `author: ["Pagefind"]`.

## Specifying a filter inline

If your value doesn't already exist on the page, you can use the syntax `filter_name:value`:

```html
<h1 data-pagefind-filter="author:bglw">Hello World</h1>
```

This will tag this page as `author: ["bglw"]`. The element this is set on does not matter, meaning this attribute can be located anywhere that is convenient in your site templating.

## Specifying multiple filters on a single element

Filter captures may be comma separated and all will apply. The exception is specifying a filter inline, which may only be the last item in a list.

For example:

```html
<h1
    data-section="Documentation"
    data-category="Article"
+    data-pagefind-filter="heading, tag[data-section], tag[data-category], author:Freeform text, captured to the end">
        Hello World
</h1>
```

This will produce the filter values for the page:

```json
{
    "heading": ["Hello World"],
    "tag": ["Documentation", "Article"],
    "author": ["Freeform text, captured to the end"]
}
```

## Notes

> The `data-pagefind-filter` attribute does not need to be within the `<body>`, or the `data-pagefind-body` tag.

> The `data-pagefind-filter` attribute will still apply if set on or within a `data-pagefind-ignore` element.

> The keys `any`, `all`, `none`, and `not` are reserved and can't be used as filter keys.


