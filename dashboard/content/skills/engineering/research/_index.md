+++
title = "research"
description = "Research a technical question against primary sources and save the cited findings as a Markdown note in the repo. Use when the user wants a saved, cited research note, not a quick doc lookup."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "engineering"
mermaid = false
+++


# Research

Hand the research to a background subagent through your agent's subagent mechanism, so you keep working while it reads. If the runtime has no subagent mechanism, do the research inline yourself, following the same steps.

The research job:

1. Investigate the question against **primary sources** - official docs, source code, specs, first-party APIs - not a secondary write-up of them. Follow every claim back to the source that owns it.
2. Write the findings to a single Markdown file, citing each claim's source.
3. Save it where the repo already keeps such notes; match the existing convention, and if there is none, choose a docs location and tell the user the path.

Completion: one Markdown file exists at a reported path, and every claim in it carries a citation to a primary source.

> Adapted from [mattpocock/skills](https://github.com/mattpocock/skills) (MIT).

