# Desktop interfaces

## Design and behavior

- Organize workspaces around objects and tasks: navigation, working area, contextual detail, and status. Use density that supports comparison and repeated use, with enough separation for reliable targeting.
- Design for resizable windows, minimum usable dimensions, display scaling, and multiple monitors where supported. Let panes resize or collapse before hiding required actions.
- Make selection, keyboard focus, active pane, and active window visually distinct. Preserve selection and scroll context through background updates.
- Follow the target OS conventions for menus, shortcuts, window controls, dialogs, file selection, and undo. Reserve familiar commands for their familiar purpose; show platform-specific key labels.
- For frequent commands, provide discoverable shortcuts alongside visible controls. Keep text editing shortcuts working inside fields and scope global commands to the correct window or document.
- Define ownership and modality for auxiliary windows. Opening or closing an inspector should not lose the editing position or strand keyboard focus.
- Use the toolkit's accessibility bridge for custom-drawn controls. Painting a readable label does not expose a usable control to assistive technology.

## Verify

Use the actual desktop runtime. Resize from the minimum supported window to a large workspace; check display scaling, theme, long labels, and pane boundaries. Exercise the task with keyboard and pointer, including focus transfer between panes, menu activation, dialog dismissal, and focus return.

Test save, cancel, undo, and unsaved-change behavior when affected. Inspect accessibility names and control roles through the OS tooling or supported bridge. Identify whether captures came from a desktop build, remote session, or existing fixture; a web preview alone does not verify native behavior.

## Official references

- [Apple Human Interface Guidelines](https://developer.apple.com/design/human-interface-guidelines/) for macOS conventions and accessibility.
- [Design Windows apps](https://learn.microsoft.com/en-us/windows/apps/design/) for Windows structure, commanding, and visual guidance.

For other desktop environments, consult their human interface guidance and the installed toolkit documentation. Cross-platform code still needs an explicit decision about which conventions follow the host OS.
