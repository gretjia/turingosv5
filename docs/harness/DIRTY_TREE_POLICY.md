# Dirty Tree Policy

Workers may start in a dirty tree but must understand it before editing.

Never revert user changes, generated evidence, or unrelated drift unless the
user explicitly asks. If unrelated files are modified, ignore them. If existing
changes affect the task, work with them and explain the interaction.

Before evidence-bearing runs, verify tree state, fresh binaries, evidence
immutability, risk class, FC trace, charter/directive completeness, and audit
round state.

## Merge Conflict Quarantine

If GitHub reports `mergeStateStatus == "dirty"` for a PR, Meta may not merge or
repair it in place. The only allowed decision is `SUPERSEDE`: close or leave the
dirty PR as rejected evidence, publish a replacement TaskPacket or branch, and
preserve the dirty PR as audit input.

Workers must not rebase, hand-resolve, or force-push around this quarantine.
