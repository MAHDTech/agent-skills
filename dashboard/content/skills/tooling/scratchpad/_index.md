+++
title = "scratchpad"
description = "Manage temporary scripts, experiment files, and debugging artifacts during a task. Use one session scratch directory, keep the source tree clean, and remove temporary files afterward."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "tooling"
mermaid = false
+++


# Scratchpad

Keep one-off scripts, experiments, and debugging artifacts in a single scratch directory outside the project's source tree:

1. Use the session-scoped scratch or temp directory your runtime provides, if it has one.
2. Otherwise create one outside the repository and note its path for the rest of the session:

   ```bash
   scratch="$(mktemp -d)"
   ```

3. Write every temporary file of the session to that same directory. Reference project files by path from there instead of copying scratch files into the repository.
4. When the experiment or debugging session ends, delete the files you created (for a `mktemp -d` directory, `rm -rf "$scratch"`).

## Completion criteria

- `git status` shows no files created by the experiment.
- The scratch directory is deleted, or it is the runtime's own session directory.

