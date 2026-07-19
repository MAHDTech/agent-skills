---
name: scratchpad
description: Keep one-off, experimental, or temporary scripts and files out of the source tree by writing them to a single, consistent session scratch location, then cleaning them up. Use when creating throwaway, experimental, or temporary scripts/files during a task.
---

# Scratchpad Policy

When a task requires creating one-off, experimental, or temporary scripts and files, you MUST adhere to the following policy to prevent source tree pollution and keep the Git history clean:

1. **Use your runtime's session scratch location**: Write temporary files, throwaway code, and test scripts to the session-scoped scratch or temp directory your runtime provides (many runtimes expose one per session). Use the native file `Write` or `Edit` tools to manage these files (do not use `echo >` or `cat <<EOF` via bash).

2. **Stay consistent**: Use the SAME scratch location for the whole session. Do not scatter temporary files across different directories, and never leave them inside the project's source tree.

3. **Fallback only if needed**: If (and only if) your runtime provides no session scratch location, create a dedicated directory for temp files and ensure it is git-ignored. Before creating the first file, verify that directory is excluded in the project's `.gitignore`; if it is not, append it. Prefer the runtime's own scratch location whenever one exists.

4. **Cleanup**: When your test, debugging session, or experiment concludes, explicitly delete the temporary files you created.
