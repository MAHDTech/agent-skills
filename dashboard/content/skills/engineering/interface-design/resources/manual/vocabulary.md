+++
title = "vocabulary"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "interface-design"
+++

# Design vocabulary

Use the term that explains an observable relationship. Replace "make it pop" with the intended change: increase contrast at the primary action, reduce competing headings, or separate navigation from content.

## Structure and perception

| Term                        | Design decision                                                           | Evidence to inspect                                                |
| --------------------------- | ------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| Information architecture    | Organize concepts around the user's tasks and vocabulary                  | Can someone predict where an item belongs?                         |
| Hierarchy                   | Rank what must be noticed, understood, and acted on                       | Does the primary task compete with secondary material?             |
| Affordance and signifier    | Distinguish what an element permits from the cues that reveal it          | Does the control look and behave as users expect?                  |
| Mapping                     | Relate a control's position or movement to its effect                     | Is the relationship between a control and its target clear?        |
| Proximity and common region | Group related material with space or a shared container                   | Are unrelated items closer than related items?                     |
| Alignment and grid          | Establish shared edges, tracks, and reading paths                         | Do labels, values, and controls follow a repeatable structure?     |
| Rhythm                      | Repeat spacing and type relationships with deliberate exceptions          | Are gaps tied to meaning or arbitrary per element?                 |
| Density                     | Balance information visible at once against scanning and targeting effort | Can the intended user compare or act without excessive navigation? |
| Progressive disclosure      | Reveal advanced or infrequent choices after the essentials                | Are important actions hidden behind unnecessary steps?             |
| Recognition over recall     | Expose useful choices and context where they are needed                   | Must users remember an identifier or command from elsewhere?       |
| Visual salience             | Direct attention through contrast, position, size, or motion              | Does the strongest signal deserve the attention it receives?       |

Choose a layout from the content relationships: a table for comparison, a list for scanning, a split view for selection and detail, a canvas for spatial work, or a sequence for dependent steps. Cards help when items are distinct units; a grid of cards is not a default information architecture.

## Typography and composition

- Define type roles for titles, section headings, body, labels, captions, code, and data. Reuse a small scale and weights that survive the smallest supported surface.
- Judge measure (line length), leading (line spacing), and paragraph spacing together using real copy. Prose, compact controls, and data tables have different reading needs.
- Consider x-height, numeral clarity, language coverage, and available weights when choosing a face. A familiar system family can be the right choice for working UI; a display face needs a reason tied to the brief.
- Use tabular numerals when comparing changing values. Align numbers and units consistently; retain enough precision for the decision being made.
- Use whitespace to separate groups and expose structure. Increasing every margin can make a tool slower without making it clearer.
- Establish a focal point and reading path. On a dashboard, the focal point may be an exception needing action rather than a decorative headline.
- Treat optical alignment as a correction to perceived balance after the underlying grid is consistent, not an excuse for unrelated offsets.

## Color, depth, and shape

- Name colors by role: surface, foreground, muted text, separator, action, selection, focus, error, warning, and success. Map roles to each supported appearance.
- Reserve strong contrast for important content and interaction. Pair state colors with text, shape, position, or icons so meaning survives color loss.
- Distinguish elevation (layer ordering) from decoration. Use overlays, borders, tone, or shadow to explain what is above what and where interaction belongs.
- Choose corner, border, icon, and spacing conventions as a system. An isolated variation should communicate a different role.
- Test real assets and icon families at their rendered size. Preserve a common visual weight and provide accessible names for icon-only actions.
- Use gradients, texture, translucency, and illustration when they serve the approved direction and remain legible. Their presence or absence alone is not a quality verdict.

## Motion and feedback

- Explain what changes, where attention should go, and whether an action succeeded. Transition direction should support the spatial or navigation model.
- Choose duration from distance, task frequency, and platform conventions; repeated commands should not wait on decorative choreography.
- Keep content available without entrance effects. Reduced-motion behavior should preserve the same information and outcome.
- Define interruption behavior: repeated activation, cancellation, reversal, and arrival of new data while a transition is running.

## Coherence test

Describe the direction in one short paragraph with decisions a builder can apply. For example: "A compact desktop workspace with aligned numeric columns, a quiet navigation rail, one action color, and a persistent detail pane. Selection changes immediately; longer operations expose progress and cancellation."

If the rationale could describe any unrelated product, inspect the task and content again. Distinctiveness can come from information structure, domain-specific visualization, or precise interaction as well as imagery and typography. Preserve a user's specified aesthetic while checking usability independently.

