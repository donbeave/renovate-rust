# Branching

This repository uses `main` as the normal working branch. Agents may commit
straight to `main` for requested work and for autonomous loop iterations.

Feature branches and pull requests are optional in this project. Use them only
when the operator explicitly asks for a branch or PR workflow.

- Keep `main` buildable, tested, and easy to review.
- Make small, coherent commits directly on `main`.
- Stage only files related to the current task.
- If the operator asks for a branch, create it from `main` and keep its name
  short, lowercase, and hyphen-separated.
- Do not force-push remote history unless the operator explicitly asks for a
  history rewrite or force push.
- Do not commit credentials, local Renovate reference clones, caches, build
  artifacts, or registry data
