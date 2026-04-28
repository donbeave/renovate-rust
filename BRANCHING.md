# Branching

This repository uses `main` as the primary branch. Keep `main` releasable and
easy to review.

All non-trivial features and bug fixes should be developed on a dedicated
branch once a remote/PR workflow exists. Direct commits to `main` are acceptable
only for local bootstrap work or when the operator explicitly asks for it.

- Create branches from `main`: `git checkout -b feature/<short-description>`
- Use prefixes that match the change type: `feature/`, `fix/`, `refactor/`,
  `docs/`, `test/`, `build/`, or `chore/`
- Keep branch names short, lowercase, and hyphen-separated
- Merge back to `main` through a pull request after review when working with a
  remote repository
- Do not force-push shared branches without explicit operator approval
- Do not commit credentials, local Renovate reference clones, caches, build
  artifacts, or registry data
