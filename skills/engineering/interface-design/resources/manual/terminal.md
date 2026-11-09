# Terminal interfaces

## Design and behavior

- Compose in rows and display cells. Measure displayed width rather than bytes or character count; wide characters, combining marks, and terminal-dependent glyphs can break alignment.
- Define a minimum usable size and resizing behavior. Collapse secondary panes or offer a readable small-window state rather than drawing over required actions.
- Show active pane, focus, selection, and available commands. Keep help reachable and distinguish navigation keys from text entry; editing a field should not unexpectedly invoke a global action.
- Preserve visible context during refresh. Avoid repainting unchanged regions or emitting updates so quickly that the user cannot read status or assistive technology cannot follow it.
- Work with the user's palette and capabilities. Keep status understandable without color; provide a plain glyph alternative where decorative symbols or line drawing fail.
- Make mouse support supplementary when keyboard operation is promised. Define activation, cancel, back, quit, and interruption behavior explicitly.
- Treat screen-reader compatibility as a behavior to test. Where a full-screen interface cannot provide usable access, offer a linear or non-interactive path for the core task when within scope, and report remaining limitations.
- When the product also exposes a CLI, preserve scriptable output, exit status, and diagnostics. Keep interactive rendering out of redirected output; respect existing plain-output and color preferences.
- Restore cursor visibility, input mode, and screen state on normal exit and handled errors. Cancellation should leave the surrounding shell usable.

## Verify

Exercise the running interface in a terminal or pseudo-terminal, including a narrow size, resize during a task, long and non-ASCII content, missing data, and operation failure. Use keyboard-only navigation and verify focus after opening and closing transient panes.

Check light and dark palettes, reduced color capability or monochrome, and plain glyph fallback where supported. Test terminal cleanup after quit, cancellation, and handled errors. For an accompanying CLI, capture redirected output separately. Record terminal and input assumptions; a screenshot does not demonstrate compatibility with other emulators or screen readers.

## Primary reference

- [XTerm control sequences](https://invisible-island.net/xterm/ctlseqs/ctlseqs.html) documents terminal protocols. Consult the chosen terminal UI library's versioned documentation for capability detection, width handling, input, and cleanup.

Terminal capabilities vary. Protocol support must be checked rather than inferred from how one emulator renders the interface.
