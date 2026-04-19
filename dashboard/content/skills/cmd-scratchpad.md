+++
title = "cmd-scratchpad"
description = "Enforces the use of a \"scratch/\" directory for all temporary or experimental AI agent scripts. Ensures the directory is .gitignored and instructs agents to clean up after completion."
date = 2026-04-19
[extra]
triggers = ["create a temporary file","run a quick test","make a scratch script","create a test script","experiment with"]
mermaid = false
+++


# Scratchpad Policy

When interacting with tasks that require creating one-off, experimental, or temporary scripts and files, you MUST adhere to the following policy to prevent source tree pollution and keep the Git history clean:

1. **Use the Scratch Folder**: ALWAYS place your temporary files, throwaway code, or test scripts inside a `scratch/` directory located at the root of the project (e.g., `scratch/test_db.js`, `scratch/debug_output.txt`).

2. **Ensure Git Ignore**: Before creating the first file in `scratch/`, verify if the `scratch/` directory is explicitly excluded in the project's `.gitignore` file. If it is not, append `scratch/` to the `.gitignore`.

3. **Cleanup**: When your test, debugging session, or experiment concludes, explicitly delete the temporary scripts you created inside `scratch/`. If you forget to delete them, the Git repository is still protected since the directory is ignored.

