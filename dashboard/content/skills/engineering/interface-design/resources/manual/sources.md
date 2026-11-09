+++
title = "sources"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "interface-design"
+++

# Sources and maintenance

## Impeccable attribution

This skill adapts selected design guidance from [Impeccable](https://github.com/pbakaus/impeccable), copyright 2025 Paul Bakaus, licensed under Apache-2.0. The license is bundled as [LICENSE.impeccable](LICENSE.impeccable).

Reviewed upstream version: **4.8.1**, commit **4b6b80f9c21cac9376deadaa2ef4924f341b8b4d**, on **2026-11-09**. Source paths below are relative to that repository's `plugin/skills/impeccable/` directory:

| Source                                        | Adaptation                                                                                    |
| --------------------------------------------- | --------------------------------------------------------------------------------------------- |
| `SKILL.md`                                    | Purpose per surface, respect for the brief, refinement versus redesign, rendered verification |
| `reference/shape.md`                          | Task, audience, outcome, scope, state coverage, and a checkable brief                         |
| `reference/operate.md`                        | Familiar task-oriented interaction, semantic states, restrained motion, useful density        |
| `reference/craft-floor.md`                    | Hierarchy, spacing, readable type, feedback, and checking actual renders                      |
| `reference/ios.md` and `reference/android.md` | Native navigation, insets, text scaling, and runtime-specific evidence                        |

The entry point and local references are modified guidance, not an upstream snapshot. They reorganize these ideas into a host-independent workflow and add desktop, terminal, and game guidance. They replace universal stylistic bans and fixed visual prescriptions with decisions tied to the user's brief, platform, content, and accessibility needs.

The skill does not bundle or require Impeccable's binary launcher, CLI, hooks, browser integration, or project sidecars. Consult the [Impeccable documentation index](https://impeccable.style/llms.txt) when investigating upstream changes; it is a navigation index, not the skill's knowledge base or an automatically mirrored resource tree.

## Ownership and catalog boundaries

The approved approach is a custom design skill with curated local knowledge and official external references. Keep guidance useful offline and retrieve current platform details when a decision depends on them. No vendor import or CLI modification is required for this approach. The existing vendor importer can target upstream's flat `plugin/skills` collection if a separate upstream integration is wanted later.

The catalog review found implementation specialists for egui, Bevy/egui, and Tailwind, and a vendor web performance skill. Retain them: they cover framework APIs, integration, compilation, or measurement that a general design workflow does not replace. No existing general UI/UX design skill was identified for retirement. This skill stays self-contained and adds no cross-skill invocation dependency.

## Updating the references

1. Compare the recorded upstream commit with the proposed revision; inspect the listed source files and any changed license or attribution notices.
2. Adopt a change only when it improves this skill's design decisions or verification across its supported surfaces. Keep upstream runtime instructions outside this skill.
3. Preserve attribution, mark adaptations, and update the version, commit, and review date when refreshing derived guidance.
4. Check the official links in the surface references. Verify changing numeric requirements and API behavior against the target platform and installed version.
5. Keep hand-authored content under `resources/manual/`; use a declared `resources:` download only if intentionally adding downloader-owned material under `resources/auto/`.

Validate frontmatter, invocation policy, relative links, router reachability, and generated catalog/dashboard artifacts through the repository's devenv checks after changes. Include manual reference files when running markdown checks, since general hook exclusions skip resource directories.

