# Contributing to SPROUT
## Artificial Intelligence
Artificial Intelligence can be used in contributions, however there are restrictions.
- you should know what the code does, and be able to explain it.
- the AI is not allowed to generate over 25% of the PR.
- AI assistance is allowed, Full AI generation isn't.
- AI usage must be clearly labeled.
- AI code must be rewritten and reviewed by a human, to ensure quality.
if you are caught not following these guidelines, you will be banned from contributing to SPROUT.

## PRs
a PR needs a clear purpose.
you should:
- explain what it changes
- explain why it's needed
- tell us what problem it solves
- how it was tested (distro, libc, compiler, rust/cargo versions)

PRs should also address only a few features at a time, they should not be monolithic rewrites.
if you're for example planning to add a feature, fix a bug and add support for an OS, seperate that
into 3 PRs, rather than 1 huge one.

note that this project uses hard tabs rather than 4 spaced indents as a design choice,
if you made your PR with four spaced indents, please run `cargo fmt` after checking `cargo fmt --check`.

you should format your code using cargo, example below:
```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

PRs should not bring in unnecessary dependencies, only bring in dependencies if they make sense.
also avoid rewriting systems just because you prefer a different philosophy.
major rewrites must be discussed before being merged, and you must get a green light from a majority
of maintainers, or the project owner before it gets merged.
if your major rewrite does not get a green light, you should fork the project instead.

you should also test if basic behavior changes, and document it in your PR.
changes to behavior must be documented and intentional.
you should also update the .md files if something changes thats mentioned in them.

## Commit standards
avoid spamming commits,or making unnecessary commits. the commit history should remain somewhat clean.
also prefix your commit with what kind of change it is.
for example these:
- fix: bug name/issue reference
- test: add tests for x
- refactor: optimize x
- docs: make documentation clearer in x
- add: add x because y
- extend: add x to feature z because y
- security: fix vulnerability
- chore: cleaning up, initializing, whatever
- WIP: working on x, not allowed in branch: MAIN
- packaging: changes for packaging

the following prefixes are reserved for the project owner:
- style: change project style (i.e. hard tabs > four spaced indent)
- CI: change CI processes
- bump: change version numbering

while these prefixes are reserved for the project owner, regular maintainers and contributors, or even
just regular users can still reccomend those changes.
if you have an issue with how CI/style is handled currently, please open an issue or a discussion rather than a PR,
unless that change has already been discussed.

please remember that WIP is not allowed in the branch MAIN and should be resolved by the time its in
your PR.
allowing/expecting dead code or other things to bypass CI should not be in a PR.
your PR will not be accepted if it includes CI bypasses, until your CI bypasses are resolved.
