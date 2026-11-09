# Interaction and accessibility

## Task and navigation

Map entry, action, feedback, recovery, and exit for the primary task. Keep the user's location and current object visible. Match labels to domain language; use action names such as "Save draft" when the result would otherwise be ambiguous.

Use navigation for destinations and controls for actions. Keep back, cancel, undo, and close distinct: going back changes location, cancel abandons a pending operation, undo reverses a completed action, and close dismisses a surface. Make unsaved-change behavior explicit.

Choose inline editing for work that benefits from its surrounding context. Use a modal when a decision requires protected focus or interruption, then define entry focus, containment, dismissal, and focus return. Consider an undo path for reversible changes; irreversible changes need a clear consequence and deliberate activation.

## State coverage

Apply states to the controls and flows that actually need them. Hover is relevant for pointers; a permission error belongs to the operation, not every label.

| State                      | Required design decision                                                            |
| -------------------------- | ----------------------------------------------------------------------------------- |
| Initial or empty           | Explain what belongs here and expose a useful first action                          |
| Loading or pending         | Preserve context, indicate progress, and handle repeat activation or cancellation   |
| Ready                      | Make the current object, available actions, and defaults understandable             |
| Focused or selected        | Distinguish keyboard destination from selected data or active navigation            |
| Disabled or unavailable    | Explain the prerequisite where users can discover it without activating the control |
| Invalid or failed          | Identify the problem near its cause, retain valid input, and offer recovery         |
| Success                    | Confirm the outcome without unnecessarily interrupting the next task                |
| Partial, offline, or stale | Identify which data or actions are affected and how to retry or reconcile           |
| Restricted                 | Explain access limitations without exposing protected information                   |

For multi-step flows, cover transitions and repeated actions as well as isolated screens. State whether optimistic changes can roll back and what happens to selection, scroll position, and drafts after refresh or failure.

## Input and feedback

- Keep focus visible, ordered by task, and stable when content updates. Restore it after a transient surface closes; choose a meaningful successor when an item is removed.
- Provide keyboard access to supported actions and show shortcuts where discoverable. For controllers, define directional neighbors and a consistent confirm/cancel convention.
- Give drag, hover, swipe, and timed interactions an alternative where users cannot perform the gesture. Avoid making hidden hover content essential on touch surfaces.
- Keep labels visible while users enter values. Put format hints before submission; identify errors in text and connect them to the relevant field.
- Show units, ranges, defaults, and consequences at the point of decision. Preserve entered values across recoverable errors.
- Expose progress and completion through the platform's accessibility mechanism as well as visible feedback. Avoid announcements on every rapidly changing value.

## Perception and access

Check readable contrast, scalable text, target size and separation, reading order, and names, roles, values, and states exposed to assistive technology. Provide non-color signals for status and non-audio alternatives for essential sound. Respect supported motion and appearance preferences.

Use platform accessibility guidance for native and game surfaces. For web work, use the [WCAG quick reference](https://www.w3.org/WAI/WCAG22/quickref/) to select the applicable criteria and level, and [ARIA Authoring Practices](https://www.w3.org/WAI/ARIA/apg/) when custom widgets require semantics and keyboard behavior. A visual checklist or automated scanner alone does not establish accessibility conformance.

Test realistic long labels, missing data, large values, alternate scripts, and right-to-left layout when supported. Use locale-aware display for dates, numbers, and units; ensure truncation retains a way to access essential content.

## Critique and acceptance

Write findings against evidence, for example: "In the save-error state, the dialog discards the entered title. Retain the draft and place retry beside the error so the user can recover without re-entry."

Classify a blocked task or inaccessible required control as high priority, extra effort or ambiguous feedback as a usability problem, and minor alignment issues as craft. Explain the affected user and state rather than assigning a taste score. Separate observed evidence from predictions requiring usability research.

For an implemented flow, demonstrate the primary task, one material failure and recovery, input navigation, and the applicable state rows. Record which accessibility checks were automated, manually inspected, or exercised with assistive technology, and which remain unverified.
