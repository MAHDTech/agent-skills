+++
title = "game-ui"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "interface-design"
+++

# Game UI

## Design and behavior

- Separate information needed during play from menus, inventory, settings, and overlays. Give urgent gameplay signals prominence without covering the scene or competing with aiming and navigation.
- Design HUD legibility for the intended viewing distance, screen sizes, contrast of changing backgrounds, and supported UI scales. Essential state needs more than a color change or brief animation.
- Define a focus graph for controller navigation, including irregular grids, disabled controls, scroll boundaries, and focus return. Every required action needs a reachable path and consistent confirm/cancel behavior.
- Change button prompts to match the active input device and the user's bindings. Switching devices should preserve selection and task progress.
- Route gameplay and UI input deliberately. Menus, text fields, and overlays must consume the relevant inputs so a selection does not also fire a weapon or move the character. Define whether the game pauses and what continues underneath.
- Keep subtitles, notifications, tutorials, and interaction prompts from obscuring each other. Provide readable text, meaningful speaker identification, and non-audio alternatives for essential cues.
- Respect supported reduced motion and camera-effect preferences. Avoid requiring repeated rapid inputs or timed reading where an alternative fits the game; expose relevant accessibility settings early.
- Preserve progress and explain recovery for disconnected controllers, interrupted sessions, unavailable services, and failed saves when affected by the task.

## Verify

Run the game build and complete the changed flow with each supported input class. Exercise controller focus across every required control and back out of nested menus. Confirm that UI input does not leak into gameplay and that input switching updates prompts without losing state.

Capture HUD and overlay states against both busy and quiet gameplay backgrounds. Check supported aspect ratios, viewing distance, safe regions, UI scale, subtitles, and long localized labels. Verify pause and resume semantics. Label missing hardware and accessibility checks rather than treating editor captures as proof of shipped behavior.

## Official reference

- [Xbox Accessibility Guidelines](https://learn.microsoft.com/en-us/xbox/accessibility/guidelines) for game-specific guidance on text, input, navigation, captions, and feedback.

Use the engine and target platform documentation for focus, input routing, and accessibility APIs. Accessibility guidance informs design; platform certification requires its own current criteria and verification.

