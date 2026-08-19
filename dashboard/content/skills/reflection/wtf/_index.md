+++
title = "wtf"
description = "Re-pitch your own last message in plain language when it did not land. Use when the user reacts to your previous response with confusion - \"wtf\", \"what?\", \"huh\", \"in English?\", \"that made no sense\", \"I don't understand\", \"say that again properly\". Not for \"wtf is this error/output/regex\", where the confusion is about a thing rather than about your message - that is debugging."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "reflection"
mermaid = false
+++


# WTF

Your last message did not land. Say it again, properly.

This is not an explanation _about_ the message. It is the message, rewritten so it works.

## Rules

1. **Shorter than what it replaces.** If the re-pitch is longer, you have explained instead of restated. Cut it down.
2. **Lead with the point.** First sentence is the conclusion, or the thing you need from the user. Context follows only if it changes what they do.
3. **No meta, no apology.** Skip "sorry for the confusion", "let me clarify", "great question", and any narration of the fact that you are restating. Just say the thing.
4. **Prose by default.** One human talking to another. Use a list only when the content genuinely is a list - three parallel items, not three sentences you did not want to join.
5. **Keep the real words.** Do not paraphrase away established domain terms - `merge-base`, `reducer`, `spoke workspace`. Keep the term and gloss it once inline: "the merge-base (the commit where the two branches last agreed)". If the project has a `CONTEXT.md` glossary, use its wording rather than inventing a synonym.
6. **Restate first, always.** Never answer a `wtf` with a bare question.

## When restating is not enough

If it is genuinely ambiguous which part lost them, add **one** short question at the end - after the re-pitch, never instead of it.

One question. This is not an interview.

## What went wrong, usually

Worth a glance before you rewrite, but do not say any of this out loud:

- Buried the point under caveats and setup.
- Used a term the user has not agreed to yet.
- Answered a bigger question than the one asked.
- Wrote for a reader who already knows the codebase.

