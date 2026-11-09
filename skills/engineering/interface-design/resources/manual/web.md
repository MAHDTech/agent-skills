# Web interfaces

## Design and behavior

- Choose semantic HTML controls and document structure before custom styling. Preserve link behavior, browser history, direct URLs, and expected keyboard interaction.
- Adapt composition to content and available width: collapse navigation, change columns, or expose table scrolling with context. A smaller viewport may require a different structure rather than a scaled desktop layout.
- Keep DOM reading and focus order coherent with the visual order. Fixed headers, footers, and overlays must leave focused controls and validation messages visible.
- Use a content model that survives slow loading, absent imagery, and real text. Reserve layout space where late content would move the user's target.
- Support touch and keyboard alongside pointer interactions. Keep essential information available without hover, and preserve visible focus when changing styles.
- Reuse the existing component system. For a new custom widget, define its semantics, accessible name, state, and keyboard model before rendering it.

## Verify

Run the primary flow in the browser at representative narrow and wide sizes and at supported zoom or text settings. Check overflow, navigation collapse, focused controls under sticky regions, long content, and overlays near viewport edges. Test supported appearances and reduced motion.

Inspect the accessibility tree and walk the flow using the keyboard. Check announcements and reading order with a screen reader when available. Exercise browser back/forward and reload on nested routes where relevant. Capture the rendered changed states and distinguish automated checks from manual evidence.

For performance work, measure the changed loading or interaction path before assigning a cause or claiming an improvement; visual smoothness in one capture is insufficient.

## Official references

- [WCAG quick reference](https://www.w3.org/WAI/WCAG22/quickref/) for applicable accessibility criteria and techniques.
- [ARIA Authoring Practices](https://www.w3.org/WAI/ARIA/apg/) for custom widget semantics and keyboard patterns.

These references supplement the project's framework documentation. Verify current APIs against the installed version before implementing custom controls.
