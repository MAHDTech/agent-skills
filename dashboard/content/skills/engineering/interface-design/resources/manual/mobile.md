+++
title = "mobile"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "interface-design"
+++

# Mobile interfaces

## Design and behavior

- Choose navigation from destination relationships and target platform conventions. Keep system back behavior and dismissal predictable; separate switching destinations from performing an action.
- Respect safe areas, system bars, display cutouts, and the software keyboard. Keep the active field, validation, and next action reachable when the keyboard appears.
- Design touch targets and separation using the platform's current guidance and units. A large icon is not proof of an adequate hit area; inspect the interactive bounds.
- Use semantic text roles and support system text scaling. Allow labels and controls to reflow; distinguish decorative text from essential task information.
- Adapt to the supported phone, tablet, foldable, and orientation configurations. Expanded layouts can expose navigation or detail panes rather than stretching the phone composition.
- Provide alternatives for gesture-only actions. Keep destructive gestures recoverable and show the result of a gesture before committing when the platform permits it.
- Use semantic theme roles and test supported appearances. Keep brand expression compatible with native controls, reading, motion preferences, and input behavior.
- Handle interruption, denied permissions, connectivity loss, and resumed sessions without silently losing entered work.

## Verify

Capture the installed app in its emulator, simulator, or device runtime. Exercise navigation, back, dismissal, keyboard entry, and affected permission or offline states. Check small supported screens, expanded layouts when in scope, large text, supported orientations, and themes.

Use platform accessibility inspection and a screen reader when available. Check reading order, names, state announcements, and focus after navigation. Label emulator evidence clearly; reach, gesture comfort, haptics, and performance may still need hardware testing. Restore any test changes to device preferences.

## Official references

- [Apple Human Interface Guidelines](https://developer.apple.com/design/human-interface-guidelines/) for iOS and iPadOS navigation, typography, input, and accessibility.
- [Material Design](https://m3.material.io/) for Android component, layout, and theme guidance.

Read the current target-specific requirements before choosing numeric touch sizes, type limits, or navigation counts. A shared framework does not make the operating systems' behavior identical.

