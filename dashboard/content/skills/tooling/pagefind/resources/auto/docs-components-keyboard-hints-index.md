+++
title = "docs-components-keyboard-hints-index"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "pagefind"
+++

{% raw %}
> **Important: Pagefind 1.5.0 introduces the Component UI, which replaces the
> Default UI (pagefind-ui.js / PagefindUI). It includes a search modal, better
> accessibility and customization.** Full component guide:
> https://pagefind.app/llms-component-ui.txt

# \<pagefind-keyboard-hints\>


Displays contextual keyboard shortcuts based on which component currently has focus.

<div class="demo-box demo-box-attached" style="flex-direction: column; gap: 1rem;">
<pagefind-input placeholder="Click me!" instance="hints-demo"></pagefind-input>
<pagefind-keyboard-hints instance="hints-demo"></pagefind-keyboard-hints>
</div>

```html
<pagefind-keyboard-hints></pagefind-keyboard-hints>
```

## Attributes

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `instance` | string | `"default"` | Connect to a specific Pagefind instance |

## How It Works

Keyboard hints are contextual. Different components register their shortcuts when focused:

| Component Focus | Shortcuts Shown |
|----------------|-----------------|
| Input | `↓ navigate`, `esc clear` |
| Results | `↑↓ navigate`, `↵ select`, `/ search` |
| Modal | `esc close` |

When no component has focus, the keyboard hints area is empty.

## Accessibility

This component is marked `aria-hidden="true"` because it's a visual aid for sighted keyboard users.

The Component UI shortcuts are either the expected controls for WAI-ARIA patterns, or they are optional alternatives to standard tab navigation patterns.

### Touch Devices

Keyboard hints are automatically hidden on touch-only devices.
{% endraw %}
