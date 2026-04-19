---
name: cmd-scratchpad
description: Enforces the use of a "scratch/" directory for all temporary or experimental AI agent scripts. Ensures the directory is .gitignored and instructs agents to clean up after completion.
triggers:
  - "create a temporary file"
  - "run a quick test"
  - "make a scratch script"
  - "create a test script"
  - "experiment with"
category: utility
---

# Scratchpad Policy

When interacting with tasks that require creating one-off, experimental, or temporary scripts and files, you MUST adhere to the following policy to prevent source tree pollution and keep the Git history clean:

1. **Use the Scratch Folder**: ALWAYS place your temporary files, throwaway code, or test scripts inside a `scratch/` directory located at the root of the project. Use the native `Write` or `Edit` tools to manage these files (do not use `echo >` or `cat <<EOF` via bash).

2. **Ensure Git Ignore**: Before creating the first file in `scratch/`, verify if the `scratch/` directory is explicitly excluded in the project's `.gitignore` file. If it is not, append `scratch/` to the `.gitignore`.

3. **Cleanup**: When your test, debugging session, or experiment concludes, explicitly delete the temporary scripts you created inside `scratch/`. If you forget to delete them, the Git repository is still protected since the directory is ignored.
