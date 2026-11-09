---
name: interface-design
description: Design, build, refine, or critique user interfaces across web, desktop, mobile, terminals, and games. Use for UI/UX decisions, visual systems, interaction flows, and rendered design reviews.
metadata:
  source: pbakaus/impeccable
  license: Apache-2.0
---

# Interface design

Make the user's next action clear and the visual language fit the product, platform, and use scene. Apply this workflow within the requested scope; selecting the skill does not authorize a redesign or implementation beyond that request. Framework APIs and build tooling belong to the project's implementation references.

## 1. Establish the brief

Read the target implementation, nearby components, real content, and existing product or design documentation. Inspect the running interface or current captures when available; a missing `DESIGN.md` does not mean the project lacks a visual identity.

Identify:

- The people, task, use scene, primary outcome, and supported platforms and input methods.
- Whether the request is planning, building, refinement, redesign, or critique, and which screens or flows it covers.
- The existing components, tokens, assets, accessibility needs, localization, and delivery constraints.
- The surface's purpose: completing a task, understanding content, deciding to act, or experiencing the work. A product can contain several purposes; choose per surface.

For working tools, make ambition visible in clear hierarchy, efficient interaction, and precise detail. For persuasive or expressive surfaces, choose a distinctive direction supported by the content. Preserve familiar platform behavior in both.

Resolve facts by inspection. Ask one focused question with a recommendation when an unresolved decision changes the outcome. Reuse answers already given; a small, specified refinement needs no discovery interview. State low-impact assumptions and proceed.

Done when the target, intended outcome, operation, and constraints are explicit enough to evaluate the result.

## 2. Load the relevant guidance

Read [interaction and accessibility](resources/manual/interaction.md) before changing flows or controls, and for every critique. Read [design vocabulary](resources/manual/vocabulary.md) when choosing or evaluating visual direction, layout, type, color, or motion. Use its terms to explain concrete decisions.

Read each applicable surface reference before proposing platform behavior or editing UI. Hybrid applications may need more than one:

| Surface  | Reference                                | Main constraints                                            |
| -------- | ---------------------------------------- | ----------------------------------------------------------- |
| Web      | [Web](resources/manual/web.md)           | Semantic structure, reflow, browser navigation, mixed input |
| Desktop  | [Desktop](resources/manual/desktop.md)   | Resizable windows, keyboard commands, dense workspaces      |
| Mobile   | [Mobile](resources/manual/mobile.md)     | Touch, system navigation, insets, text scaling              |
| Terminal | [Terminal](resources/manual/terminal.md) | Cell geometry, focus, terminal capabilities, plain output   |
| Game UI  | [Game UI](resources/manual/game-ui.md)   | Controller navigation, HUD readability, gameplay input      |

The bundled references work offline. When a decision depends on a current platform requirement or numeric threshold, verify it against the official sources linked in the relevant reference. If retrieval is unavailable, name the unverified requirement rather than inventing a value. [Sources and maintenance](resources/manual/sources.md) records provenance and adaptation boundaries.

## 3. Choose and execute the requested operation

### Plan

Return a brief with the primary task, screen or flow structure, interaction model, visual direction, state coverage, constraints, and acceptance checks. Use a wireframe or flow diagram when relationships are easier to inspect visually. Compare alternatives only where a real tradeoff remains. Stop at the requested planning deliverable.

### Build or redesign

For new work, describe a coherent direction in concrete terms: hierarchy, density, typography, palette roles, composition, and interaction. For an established product, derive those choices from its current system. A redesign may replace visual language within its agreed scope while retaining product facts and required behavior.

Build one representative task path with realistic content before extending the pattern across screens. Reuse the project's controls and semantic tokens; extend them only for a recurring need. Implement the applicable empty, loading, error, success, selection, and permission states alongside the normal path. Preserve factual copy and use approved assets or clearly identified placeholders; invented claims are not design material.

### Refine

Name the observed problem and change the smallest set of elements that resolves it. Preserve the incumbent identity and behavior outside scope. Evaluate spacing, hierarchy, copy, contrast, and feedback against the task; a request to polish a toolbar does not authorize replacing navigation or branding.

### Critique

Inspect the actual interface and primary task before judging its styling. Report findings as: location and state, observed evidence, user impact, severity, and a concrete correction. Prioritize blocked tasks and access barriers, then confusion and inconsistency, then craft details. Distinguish measured failures from hypotheses that need user testing. Stop at findings unless fixes were requested.

Done when the chosen operation has produced its requested artifact or scoped change, with every brief requirement accounted for.

## 4. Verify the result

Use the project's supported build, test, and capture tools. Inspect the rendered result on the target runtime at representative supported sizes, themes, content extremes, and text scales. Batch observations, fix related issues, and inspect again. A browser capture verifies a web surface; native apps, terminal interfaces, and games need evidence from their own runtime.

Walk the primary task and a recovery path using every supported input class. Check focus order and return, readable labels, state feedback, overflow, and the platform reference's acceptance checks. Use accessibility inspection and assistive technology where available; screenshots alone cannot verify interaction or accessibility. Run checks relevant to the changed behavior and report their results.

When rendering or an input device is unavailable, continue source inspection or use existing captures, state their freshness, and identify exactly what remains unverified. Never present source review as a completed visual or usability test.

For durable decisions, update the existing design documentation. Create `DESIGN.md` only when a new or changed system needs a lasting record: purpose, visual roles, component conventions, interaction rules, and rationale. A local fix does not require a new document.

Finish with the delivered result, verification evidence, and remaining gaps. Planning and critique stop at their requested artifacts; implemented work is complete when the brief is covered, applicable checks pass, and runtime limitations are explicit.
