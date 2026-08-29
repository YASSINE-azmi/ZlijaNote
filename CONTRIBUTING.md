# Contributing to ZlijaNote

Thanks for your interest in contributing! ZlijaNote is a local-first visual note editor for developers. This guide explains how to participate in design decisions and code contributions.

## Ways to contribute

- Report bugs or propose features via GitHub issues
- Write or improve RFCs in `doc/decisions/`
- Improve documentation in `doc/`
- Submit code for v0.1.0-alpha

## Ground rules

- **User data is sacred.** ZlijaNote never silently rewrites, deletes, or "fixes" user code. Any change that could lose data requires a safe copy first.
- **The `.html` file is the source of truth.** Never let editor state become the primary storage.
- **No lock-in.** Format decisions must keep notes openable outside the app.
- **Be kind.** Assume good faith; keep feedback constructive and focused on the work.

## RFC process (design decisions)

Significant design decisions — anything touching the file format, security model, editor behavior, or product scope — go through an RFC before code is written:

1. Copy [`doc/decisions/0000-template.md`](doc/decisions/0000-template.md) to `doc/decisions/NNNN-title.md` (next number, lowercase-with-dashes).
2. Fill in summary, motivation, detailed design, trade-offs, alternatives, and open questions.
3. Open a PR with status `Draft` or `Under Review`.
4. Discussion happens on the PR. The project owner has the final say on every decision.
5. Once accepted, set status to `Accepted` and merge before implementing.

Small fixes (typos, bugs, refactors) don't need an RFC, but anything that changes user-visible behavior or the on-disk format does.

## Development setup

```bash
git clone <repo-url> && cd ZlijaNote
cargo run
```

## Workflow

1. Fork the repo and create a branch: `feature/short-description` or `fix/short-description`.
2. Keep commits small and focused; write messages that explain *why*.
3. Run `cargo fmt` and `cargo clippy` before submitting.
4. Open a PR with a clear description and link any related issue or RFC.

## Code style

- Follow standard Rust conventions (`rustfmt` defaults, `clippy` clean).
- No comments unless they explain non-obvious intent — code should read clearly.
- New files must match existing structure and naming conventions.

## Testing

v0.1 is considered "done" only when a brand-new user succeeds at the core scenarios listed in `doc/product-spec.md` — most importantly:

- Installing on Arch Linux and opening the app
- Creating a project, dragging components, editing HTML/CSS, and previewing
- Closing and reopening the app without losing notes
- Opening a `.html` note outside the app in a browser and seeing it correctly
- Confirming notes cannot execute JavaScript or touch the device file system

Write tests for any logic that can be tested headlessly; manual scenario checklists cover the rest.

## Reporting bugs

Include: what you did, what you expected, what happened, and (if relevant) the note file or workspace structure. Never attach notes containing sensitive data.
