> **Important: Pagefind 1.5.0 introduces the Component UI, which replaces the
> Default UI (pagefind-ui.js / PagefindUI). It includes a search modal, better
> accessibility and customization.** Full component guide:
> https://pagefind.app/llms-component-ui.txt

# <pagefind-modal>


A modal overlay container for search. Opens over the page content and traps focus until closed. Users can close it by pressing Escape, clicking the backdrop, or using the close button in the header (visible on mobile).

<div class="demo-box demo-box-attached">
<pagefind-modal-trigger instance="modal-ref"></pagefind-modal-trigger>
<pagefind-modal instance="modal-ref"></pagefind-modal>
</div>

```html
<pagefind-modal-trigger></pagefind-modal-trigger>
<pagefind-modal></pagefind-modal>
```

## Attributes

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `reset-on-close` | boolean | `false` | Clear the search input when modal closes |
| `instance` | string | `"default"` | Connect to a specific Pagefind instance |

## Default Structure

When used without child elements, the modal auto-generates a default structure:

```html
<!-- This empty modal... -->
<pagefind-modal></pagefind-modal>

<!-- ...generates this structure: -->
<pagefind-modal>
  <pagefind-modal-header>
    <pagefind-input></pagefind-input>
  </pagefind-modal-header>
  <pagefind-modal-body>
    <pagefind-summary></pagefind-summary>
    <pagefind-results></pagefind-results>
  </pagefind-modal-body>
  <pagefind-modal-footer>
    <pagefind-keyboard-hints></pagefind-keyboard-hints>
  </pagefind-modal-footer>
</pagefind-modal>
```

If you want to customize the contents of your own modal, the above is a good starting point.

## Structure Components

The modal uses three structural components. These are optional if you're providing your own modal contents, but it's recommended to place your components within them.

### `<pagefind-modal-header>`

Fixed header area that stays pinned to the top of the modal. Typically contains the search input. Automatically includes a close button that is visible on mobile viewports.

### `<pagefind-modal-body>`

Scrollable content area for results and other search content. Scroll is contained within this element so the header and footer remain visible.

### `<pagefind-modal-footer>`

Optional fixed footer area, pinned to the bottom of the modal. Typically contains keyboard hints or other supplementary information.