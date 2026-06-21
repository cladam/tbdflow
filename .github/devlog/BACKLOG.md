# Backlog

Items to investigate and potentially implement.

---

## GPG Signing

**Status:** Needs investigation  
**Inspiration:** ajsb85/tbdflow fork (v0.29.0+)

Automatic GPG-sign commits (`-S`) and tags (`-u`/`-s`) when a signing key is
configured (`user.signingkey` or `commit.gpgsign=true`). Honour `commit.gpgsign`;
add `--no-sign` to opt out per call. Set `GPG_TTY` for agent environments.

### Questions to resolve

- Do we want signing to be fully automatic when a key exists, or opt-in via config?
- How should we handle `gpg-agent` passphrase prompts in non-interactive (CI) mode?
- Should `tbdflow doctor` (if added) verify the signing key is usable?
- Tag signing: annotated tags only, or also lightweight?

### Implementation sketch

1. Add `git::signing_key(opts) -> Option<String>` — reads `user.signingkey` from git config.
2. If a key is present, pass `-S` to `git commit` and `-s`/`-u <key>` to `git tag`.
3. Add `--no-sign` global CLI flag to disable for one invocation.
4. Set `GPG_TTY=$(tty)` in the environment when spawning git, so pinentry works.

